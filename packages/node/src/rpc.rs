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

pub async fn run_rpc(addr: &str, chain_id: u64) -> Result<(), Box<dyn std::error::Error>> {
    // Genesis allocation (1 млрд AIC = 1e27 wei, 18 decimals)
    let total: u128 = 1_000_000_000_000_000_000_000_000_000; // 1e27
    let treasury: [u8; 20] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0xF];
    let airdrop:  [u8; 20] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0xA];
    let staking:  [u8; 20] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0x5];
    let team:     [u8; 20] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0x1];

    let alloc = vec![
        (treasury, total * 40 / 100),
        (airdrop,  total * 30 / 100),
        (staking,  total * 20 / 100),
        (team,     total * 10 / 100),
    ];

    let chain = Chain::genesis(chain_id, &alloc);
    println!("=== GENESIS ===");
    println!("Chain ID: {chain_id}");
    println!("Block 0 hash: 0x{}", hex::encode(chain.head().hash()));
    println!("State root:   0x{}", hex::encode(chain.head().header.state_root));
    for (addr, bal) in &alloc {
        println!("  alloc 0x{}: {} wei ({} AIC)",
            hex::encode(addr), bal, bal / 1_000_000_000_000_000_000u128);
    }

    let state = Arc::new(AppState {
        mempool: Mempool::new(),
        chain: RwLock::new(chain),
        chain_id,
    });

    // Запускаем block producer
    let producer_state = state.clone();
    tokio::spawn(async move {
        let mut ticker = tokio::time::interval(std::time::Duration::from_secs(3));
        ticker.tick().await; // первый тик сразу -- пропускаем
        loop {
            ticker.tick().await;
            let txs = producer_state.mempool.drain(100).await;
            let mut chain = producer_state.chain.write().unwrap();
            let bn = chain.block_number() + 1;
            let block = chain.produce_block(txs, now_unix());
            println!("[BLOCK] #{bn} hash=0x{} txs={} gas={}",
                hex::encode(&block.hash()[..8]),
                block.transactions.len(),
                block.header.gas_used);
        }
    });

    let app = Router::new().route("/", post(handle_rpc)).with_state(state);
    let listener = TcpListener::bind(addr).await?;
    println!("\nAIC Node RPC listening on http://{addr}");
    axum::serve(listener, app).await?;
    Ok(())
}

async fn handle_rpc(
    axum::extract::State(st): axum::extract::State<Arc<AppState>>,
    Json(req): Json<RpcRequest>,
) -> Json<RpcResponse> {
    let result: Option<Value> = match req.method.as_str() {
        "aic_chainId" => Some(json!(st.chain_id)),

        "aic_blockNumber" => {
            let c = st.chain.read().unwrap();
            Some(json!(format!("0x{:x}", c.block_number())))
        }

        "aic_mempoolSize" => {
            let n = st.mempool.len().await;
            Some(json!(n))
        }

        "aic_getBalance" => {
            let addr = req.params.get(0).and_then(|v| v.as_str());
            match addr.and_then(parse_addr) {
                Some(a) => {
                    let c = st.chain.read().unwrap();
                    let acc = c.state.get_account(&a);
                    Some(json!(format!("0x{:x}", acc.balance)))
                }
                None => None,
            }
        }

        "aic_getAccount" => {
            let addr = req.params.get(0).and_then(|v| v.as_str());
            match addr.and_then(parse_addr) {
                Some(a) => {
                    let c = st.chain.read().unwrap();
                    let acc = c.state.get_account(&a);
                    Some(json!({
                        "balance": format!("0x{:x}", acc.balance),
                        "nonce": acc.nonce,
                    }))
                }
                None => None,
            }
        }

        "aic_getBlockByNumber" => {
            let n = req.params.get(0).and_then(|v| v.as_str());
            let number = match n {
                Some("latest") => {
                    let c = st.chain.read().unwrap();
                    c.block_number()
                }
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

        "aic_sendRawTransaction" => {
            let tx_json = req.params.get(0);
            match tx_json.and_then(Transaction::from_json) {
                Some(tx) => {
                    let hash = tx.hash();
                    st.mempool.add(tx).await;
                    Some(json!(format!("0x{}", hex::encode(hash))))
                }
                None => None,
            }
        }

        _ => None,
    };

    let error = if result.is_none() {
        Some(json!({"code": -32601, "message": "method not found"}))
    } else { None };

    Json(RpcResponse { jsonrpc: "2.0", id: req.id, result, error })
}

fn parse_addr(s: &str) -> Option<[u8; 20]> {
    let b = hex::decode(s.trim_start_matches("0x")).ok()?;
    if b.len() != 20 { return None; }
    let mut a = [0u8; 20]; a.copy_from_slice(&b); Some(a)
}
