use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use axum::{routing::post, Router, Json};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio::net::TcpListener;
use std::sync::RwLock;

use crate::chain::Chain;
use crate::mempool::Mempool;
use crate::transaction::Transaction;
use crate::p2p::{P2PCommand, P2PEvent, P2PHandles};

use revm::{
    context::{Context, TxEnv},
    context_interface::result::{ExecutionResult, Output},
    primitives::{Address, Bytes, TxKind, U256},
    ExecuteEvm, MainBuilder, MainContext,
};

pub struct AppState {
    pub mempool: Mempool,
    pub chain: Arc<RwLock<Chain>>,
    pub chain_id: u64,
    pub p2p_tx: tokio::sync::mpsc::UnboundedSender<P2PCommand>,
}

#[derive(Deserialize)]
struct RpcRequest {
    #[serde(default)] id: Value,
    method: String,
    #[serde(default)] params: Value,
}

#[derive(Serialize)]
struct RpcResponse {
    jsonrpc: &'static str,
    id: Value,
    #[serde(skip_serializing_if = "Option::is_none")] result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")] error: Option<Value>,
}

fn now_unix() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0)
}

const TREASURY: [u8; 20] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0x0F];

pub async fn run_rpc(
    addr: &str,
    chain_id: u64,
    mut p2p_handles: P2PHandles,
    no_mine: bool,
    chain: Arc<RwLock<Chain>>,
    my_addr: Option<[u8; 20]>,
) -> Result<(), Box<dyn std::error::Error>> {
let p2p_tx = p2p_handles.command_tx.clone();

    let state = Arc::new(AppState {
        mempool: Mempool::new(),
        chain: chain.clone(),
        chain_id,
        p2p_tx: p2p_tx.clone(),
    });

    // === P2P event loop: принимаем чужие блоки и транзакции ===
    let p2p_state = state.clone();
    let mut event_rx = p2p_handles.event_rx;
    tokio::spawn(async move {
        while let Some(event) = event_rx.recv().await {
            match event {
                P2PEvent::NewBlock(block) => {
                    let mut chain = p2p_state.chain.write().unwrap();
                    let bn = block.header.number;
                    if chain.add_block_from_peer(block) {
                        let chain_file = std::path::PathBuf::from("data").join("chain.json");
                        if let Err(e) = chain.save(&chain_file) {
                            eprintln!("⚠ Save failed: {e}");
                        }
                        println!("[BLOCK-P2P] accepted #{}", bn);
                    }
                }
                P2PEvent::NewTransaction(tx) => {
                    let from = hex::encode(&tx.from[..4]);
                    p2p_state.mempool.add(tx).await;
                    eprintln!("[TX-P2P] received from 0x{}...", from);
                }
                P2PEvent::PeerConnected(peer) => {
                    eprintln!("[P2P] peer connected: {peer}");
                }
                P2PEvent::PeerDisconnected(peer) => {
                    eprintln!("[P2P] peer disconnected: {peer}");
                }
            }
        }
    });

    // === Block producer: рожаем блок -> сохраняем -> broadcast ===
    if !no_mine {
        let producer_state = state.clone();
        tokio::spawn(async move {
        let mut ticker = tokio::time::interval(std::time::Duration::from_secs(3));
        ticker.tick().await;
        loop {
            ticker.tick().await;

            // DPoS: проверяем, наша ли очередь
            if let Some(me) = my_addr {
                let chain_guard = producer_state.chain.read().unwrap();
                if !chain_guard.can_mine(&me) {
                    let next = chain_guard.next_validator().map(|v| hex::encode(&v[..4])).unwrap_or_default();
                    drop(chain_guard);
                    continue; // ждём, пока следующий валидатор создаст блок
                }
            }

            let txs = producer_state.mempool.drain(100).await;
            let mut chain = producer_state.chain.write().unwrap();
            let bn = chain.block_number() + 1;
            let block = chain.produce_block(txs, now_unix());
            let tx_count = block.transactions.len();
            let gas_used = block.header.gas_used;
            let hash_short = hex::encode(&block.hash()[..8]);
            let chain_file = std::path::PathBuf::from("data").join("chain.json");
            if let Err(e) = chain.save(&chain_file) { eprintln!("⚠ Save failed: {e}"); }
            println!("[BLOCK] #{} hash=0x{} txs={} gas={}", bn, hash_short, tx_count, gas_used);
            // Broadcast
            let _ = producer_state.p2p_tx.send(P2PCommand::BroadcastBlock(block.clone()));
        }
        });
    } else {
        println!("[miner] disabled (--no-mine): syncing only");
    }

    

    let app = Router::new().route("/", post(handle_rpc)).with_state(state);
    let listener = TcpListener::bind(addr).await?;
    println!("AIC Node RPC listening on http://{addr}");
    axum::serve(listener, app).await?;
    Ok(())
}

async fn handle_rpc(
    axum::extract::State(st): axum::extract::State<Arc<AppState>>,
    Json(req): Json<RpcRequest>,
) -> Json<RpcResponse> {
    let method_owned = req.method.clone();
    let result: Option<Value> = match req.method.as_str() {
        "aic_chainId" => Some(json!(st.chain_id)),

        "aic_blockNumber" => {
            let c = st.chain.read().unwrap();
            Some(json!(format!("0x{:x}", c.block_number())))
        }

        "aic_mempoolSize" => Some(json!(st.mempool.len().await)),

        "aic_getBalance" => {
            let addr = req.params.get(0).and_then(|v| v.as_str());
            addr.and_then(parse_addr).map(|a| {
                let c = st.chain.read().unwrap();
                let acc = c.state.get_account(&a);
                json!(format!("0x{:x}", acc.balance))
            })
        }

        "aic_getAccount" => {
            let addr = req.params.get(0).and_then(|v| v.as_str());
            addr.and_then(parse_addr).map(|a| {
                let c = st.chain.read().unwrap();
                let acc = c.state.get_account(&a);
                json!({"balance": format!("0x{:x}", acc.balance), "nonce": acc.nonce})
            })
        }

        "aic_getBlockByNumber" => {
            let n = req.params.get(0).and_then(|v| v.as_str());
            let number = match n {
                Some("latest") => st.chain.read().unwrap().block_number(),
                Some("earliest") => 0,
                Some(h) => u64::from_str_radix(h.trim_start_matches("0x"), 16).unwrap_or(u64::MAX),
                None => 0,
            };
            let c = st.chain.read().unwrap();
            c.blocks.get(number as usize).map(|b| json!({
                "number": format!("0x{:x}", b.number()),
                "hash": format!("0x{}", hex::encode(b.hash())),
                "parentHash": format!("0x{}", hex::encode(b.header.parent_hash)),
                "timestamp": format!("0x{:x}", b.header.timestamp),
                "stateRoot": format!("0x{}", hex::encode(b.header.state_root)),
                "gasUsed": format!("0x{:x}", b.header.gas_used),
                "gasLimit": format!("0x{:x}", b.header.gas_limit),
                "txCount": b.transactions.len(),
            }))
        }

        "aic_call" => {
            let to_str = req.params.get(0).and_then(|v| v.as_str());
            let data_str = req.params.get(1).and_then(|v| v.as_str());
            match (to_str.and_then(parse_addr), data_str) {
                (Some(to_addr), Some(d)) => {
                    let clean = d.trim_start_matches("0x").trim_start_matches("0X");
                    let data_bytes = match hex::decode(clean) {
                        Ok(b) => b,
                        Err(e) => return Json(RpcResponse {
                            jsonrpc: "2.0", id: req.id, result: None,
                            error: Some(json!({"code": -32602, "message": format!("aic_call: invalid data hex: {e}")})),
                        }),
                    };
                    let chain_guard = st.chain.read().unwrap();
                    let db = chain_guard.state.to_inmemory();
                    drop(chain_guard);

                    let mut cfg: revm::context::CfgEnv<revm::primitives::hardfork::SpecId> = Default::default();
                    cfg.chain_id = st.chain_id;
                    let mut evm = Context::mainnet().with_db(db).with_cfg(cfg).build_mainnet();

                    let mut tx = TxEnv::default();
                    tx.caller = Address::from(TREASURY);
                    tx.gas_limit = 10_000_000;
                    tx.gas_price = 0;
                    tx.kind = TxKind::Call(Address::from(to_addr));
                    tx.value = U256::ZERO;
                    tx.data = Bytes::from(data_bytes);
                    tx.nonce = 0;
                    tx.chain_id = Some(st.chain_id);

                    match evm.transact(tx) {
                        Ok(res) => match res.result {
                            ExecutionResult::Success { output: Output::Call(b), .. } => {
                                Some(json!(format!("0x{}", hex::encode(&b))))
                            }
                            ExecutionResult::Revert { output, .. } => return Json(RpcResponse {
                                jsonrpc: "2.0", id: req.id, result: None,
                                error: Some(json!({"code": -32000, "message": format!("revert: 0x{}", hex::encode(&output))})),
                            }),
                            ExecutionResult::Halt { reason, .. } => return Json(RpcResponse {
                                jsonrpc: "2.0", id: req.id, result: None,
                                error: Some(json!({"code": -32000, "message": format!("halt: {:?}", reason)})),
                            }),
                            _ => None,
                        },
                        Err(e) => return Json(RpcResponse {
                            jsonrpc: "2.0", id: req.id, result: None,
                            error: Some(json!({"code": -32000, "message": format!("evm error: {e:?}")})),
                        }),
                    }
                }
                _ => None,
            }
        }

        "aic_sendRawTransaction" => {
            let raw = req.params.get(0).and_then(|v| v.as_str());
            let pq_pk_hex = req.params.get(1).and_then(|v| v.as_str());
            let pq_sig_hex = req.params.get(2).and_then(|v| v.as_str());

            match raw.and_then(Transaction::from_raw_hex) {
                Some(mut tx) => {
                    if let Some(pk) = pq_pk_hex {
                        if let Ok(bytes) = hex::decode(pk.trim_start_matches("0x")) {
                            if !bytes.is_empty() { tx.pq_pubkey = Some(bytes); }
                        }
                    }
                    if let Some(sig) = pq_sig_hex {
                        if let Ok(bytes) = hex::decode(sig.trim_start_matches("0x")) {
                            if !bytes.is_empty() { tx.pq_signature = Some(bytes); }
                        }
                    }
                    if !tx.verify() {
                        return Json(RpcResponse {
                            jsonrpc: "2.0", id: req.id, result: None,
                            error: Some(json!({"code": -32000, "message": "ECDSA signature invalid"})),
                        });
                    }
                    if !tx.verify_pq() {
                        return Json(RpcResponse {
                            jsonrpc: "2.0", id: req.id, result: None,
                            error: Some(json!({"code": -32000,
                                "message": "PQ signature required for value >= 1000 AIC, or invalid Dilithium"})),
                        });
                    }

                    let hash = tx.hash();
                    let value_aic = tx.value / 1_000_000_000_000_000_000;
                    let has_pq = tx.pq_pubkey.is_some();
                    st.mempool.add(tx.clone()).await;
                    let _ = st.p2p_tx.send(P2PCommand::BroadcastTx(tx));
                    eprintln!("[mempool] accepted tx hash=0x{} value={} AIC pq={}",
                        hex::encode(&hash[..8]), value_aic, has_pq);
                    Some(json!(format!("0x{}", hex::encode(hash))))
                }
                _ => None,
            }
        }

        _ => None,
    };

    let error = if result.is_none() {
        Some(json!({"code": -32651, "message": format!("method not found: {}", method_owned)}))
    } else { None };

    Json(RpcResponse { jsonrpc: "2.0", id: req.id, result, error })
}

fn parse_addr(s: &str) -> Option<[u8; 20]> {
    let clean = s.trim_start_matches("0x").trim_start_matches("0X");
    let b = hex::decode(clean).ok()?;
    if b.len() != 20 { return None; }
    let mut a = [0u8; 20]; a.copy_from_slice(&b); Some(a)
}




