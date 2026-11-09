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

use revm::{
    context::{Context, TxEnv},
    context_interface::result::{ExecutionResult, Output},
    primitives::{Address, Bytes, TxKind, U256},
    ExecuteEvm, MainBuilder, MainContext,
};

pub struct AppState {
    pub mempool: Mempool,
    pub chain: RwLock<Chain>,
    pub chain_id: u64,
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

pub async fn run_rpc(addr: &str, chain_id: u64) -> Result<(), Box<dyn std::error::Error>> {
    const AIC: u128 = 1_000_000_000_000_000_000;
    let total: u128 = 71_000_000_000u128 * AIC;

    let treasury: [u8; 20] = TREASURY;
    let airdrop: [u8; 20] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0x0A];
    let staking: [u8; 20] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0x05];
    let team: [u8; 20] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0x01];
    let dev: [u8; 20] = [
        0xf3, 0x9F, 0xd6, 0xe5, 0x1a, 0xad, 0x88, 0xF6,
        0xF4, 0xce, 0x6a, 0xB8, 0x82, 0x72, 0x79, 0xcf,
        0xff, 0xb9, 0x22, 0x66,
    ];
    let dev_balance: u128 = 1_000_000u128 * AIC;

    let alloc: Vec<([u8; 20], u128)> = vec![
        (treasury, total / 100 * 40),
        (airdrop,  total / 100 * 30),
        (staking,  total / 100 * 20),
        (team,     total / 100 * 10),
        (dev,      dev_balance),
    ];

    let data_dir = std::path::PathBuf::from("data");
    let chain_file = data_dir.join("chain.json");

    let chain = if chain_file.exists() {
        match Chain::load(&chain_file) {
            Ok(c) => {
                println!("\n========= AIC CHAIN LOADED =========");
                println!("Loaded: {}, Blocks: {}, Head: #{}", chain_file.display(), c.blocks.len(), c.block_number());
                println!("====================================\n");
                c
            }
            Err(e) => {
                eprintln!("⚠ Load failed: {e}. Recreating genesis.");
                std::fs::remove_file(&chain_file).ok();
                let c = Chain::genesis(chain_id, &alloc);
                c.save(&chain_file).ok();
                c
            }
        }
    } else {
        let c = Chain::genesis(chain_id, &alloc);
        c.save(&chain_file)?;
        println!("\n========= AIC CHAIN GENESIS =========");
        println!("Chain ID: {chain_id}, State root: 0x{}", hex::encode(c.head().header.state_root));
        for (a, bal) in &alloc { println!("  {}  =>  {} AIC", hex::encode(a), bal / AIC); }
        println!("====================================\n");
        c
    };

    let state = Arc::new(AppState {
        mempool: Mempool::new(),
        chain: RwLock::new(chain),
        chain_id,
    });

    let producer_state = state.clone();
    tokio::spawn(async move {
        let mut ticker = tokio::time::interval(std::time::Duration::from_secs(3));
        ticker.tick().await;
        loop {
            ticker.tick().await;
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
        }
    });

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
                        Err(e) => {
                            eprintln!("[aic_call] INVALID DATA HEX: len={} err={}", clean.len(), e);
                            return Json(RpcResponse {
                                jsonrpc: "2.0", id: req.id, result: None,
                                error: Some(json!({
                                    "code": -32602,
                                    "message": format!("aic_call: invalid data hex (len={}, err={}). data must be hex string, no spaces.", clean.len(), e)
                                })),
                            });
                        }
                    };
                    eprintln!("[aic_call] to=0x{} data_len={} bytes", hex::encode(to_addr), data_bytes.len());
                    let chain_guard = st.chain.read().unwrap();
                    let db = chain_guard.state.to_inmemory();
                    drop(chain_guard);

                    // Проверка: есть ли код в нашем State
                    {
                        let guard = st.chain.read().unwrap();
                        match guard.state.get_code(&to_addr) {
                            Some(code) => eprintln!("[aic_call] state.get_code OK: len={}", code.len()),
                            None => eprintln!("[aic_call] state.get_code: NO CODE at 0x{}", hex::encode(to_addr)),
                        }
                    }let mut evm = Context::mainnet().with_db(db).build_mainnet();

                    let mut tx = TxEnv::default();
                    tx.caller = Address::from(TREASURY);
                    tx.gas_limit = 10_000_000;
                    tx.gas_price = 0;
                    tx.kind = TxKind::Call(Address::from(to_addr));
                    tx.value = U256::ZERO;
                    tx.data = Bytes::from(data_bytes.clone());
                    tx.nonce = 0;
                    eprintln!("[aic_call] BEFORE transact: tx.data.len()={}", tx.data.len());
                    

                    match evm.transact(tx) {
                        Ok(res) => {
                            eprintln!("[aic_call] EVM result = {:?}", res.result);
                            match res.result {
                            ExecutionResult::Success { output: Output::Call(b), .. } => {
                                Some(json!(format!("0x{}", hex::encode(&b))))
                            }
                            ExecutionResult::Success { output: Output::Create(b, _), .. } => {
                                Some(json!(format!("0x{}", hex::encode(&b))))
                            }
                            ExecutionResult::Revert { output, .. } => {
                                return Json(RpcResponse {
                                    jsonrpc: "2.0", id: req.id, result: None,
                                    error: Some(json!({"code": -32000, "message": format!("revert: 0x{}", hex::encode(&output))})),
                                });
                            }
                            ExecutionResult::Halt { reason, .. } => {
                                return Json(RpcResponse {
                                    jsonrpc: "2.0", id: req.id, result: None,
                                    error: Some(json!({"code": -32000, "message": format!("halt: {:?}", reason)})),
                                });
                            }
                            }
                        },
                        Err(e) => {
                            return Json(RpcResponse {
                                jsonrpc: "2.0", id: req.id, result: None,
                                error: Some(json!({"code": -32000, "message": format!("evm error: {e:?}")})),
                            });
                        }
                    }
                }
                (Some(_), None) => {
                    return Json(RpcResponse {
                        jsonrpc: "2.0", id: req.id, result: None,
                        error: Some(json!({"code": -32602, "message": "aic_call: missing data"})),
                    });
                }
                (None, _) => {
                    return Json(RpcResponse {
                        jsonrpc: "2.0", id: req.id, result: None,
                        error: Some(json!({"code": -32602, "message": "aic_call: bad contract addr"})),
                    });
                }
            }
        }

        "aic_sendRawTransaction" => {
            // params: [raw_hex, pq_pubkey_hex?, pq_signature_hex?]
            let raw = req.params.get(0).and_then(|v| v.as_str());
            let pq_pk_hex = req.params.get(1).and_then(|v| v.as_str());
            let pq_sig_hex = req.params.get(2).and_then(|v| v.as_str());

            match raw.and_then(Transaction::from_raw_hex) {
                Some(mut tx) => {
                    // Подключаем PQ-поля, если пришли
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
                    st.mempool.add(tx).await;
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







