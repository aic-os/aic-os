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
    // ===== ЭМИССИЯ: 71 млрд AIC, 18 decimals =====
    const AIC: u128 = 1_000_000_000_000_000_000; // 1 AIC в wei
    let total: u128 = 71_000_000_000u128 * AIC; // 71 млрд AIC

    // Казна: 40% = 28.4 млрд AIC, адрес 0x...0F
    let treasury: [u8; 20] = [
        0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0x0F,
    ];
    // Airdrop: 30% = 21.3 млрд AIC, адрес 0x...0A
    let airdrop: [u8; 20] = [
        0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0x0A,
    ];
    // Стейкинг: 20% = 14.2 млрд AIC, адрес 0x...05
    let staking: [u8; 20] = [
        0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0x05,
    ];
    // Foundation: 10% = 7.1 млрд AIC, адрес 0x...01
    let team: [u8; 20] = [
        0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0x01,
    ];
    // Dev: Hardhat-0 (Alice), 1 000 000 AIC для тестов
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

    let chain = Chain::genesis(chain_id, &alloc);

    // ===== GENESIS LOG =====
    println!("\n========= AIC CHAIN GENESIS =========");
    println!("Chain ID:    {chain_id}");
    println!("Block 0 hash: 0x{}", hex::encode(chain.head().hash()));
    println!("State root:  0x{}", hex::encode(chain.head().header.state_root));
    println!("--- Allocations ---");
    for (a, bal) in &alloc {
        println!("  {}  =>  {} wei ({} AIC)",
            hex::encode(a), bal, bal / AIC);
    }
    println!("--- Verification ---");
    for (a, expected) in &alloc {
        let actual = chain.state.get_account(a).balance;
        let mark = if actual == *expected { "OK" } else { "MISMATCH" };
        println!("  {}: {} (expected {})", hex::encode(a), mark, actual);
    }
    println!("=====================================\n");

    let state = Arc::new(AppState {
        mempool: Mempool::new(),
        chain: RwLock::new(chain),
        chain_id,
    });

    // Block producer
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
            println!("[BLOCK] #{} hash=0x{} txs={} gas={}",
                bn, hex::encode(&block.hash()[..8]),
                block.transactions.len(), block.header.gas_used);
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
            let raw = req.params.get(0).and_then(|v| v.as_str());
            match raw.and_then(Transaction::from_raw_hex) {
                Some(tx) if tx.verify() => {
                    let hash = tx.hash();
                    st.mempool.add(tx).await;
                    Some(json!(format!("0x{}", hex::encode(hash))))
                }
                _ => None,
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
    let clean = s.trim_start_matches("0x").trim_start_matches("0X");
    let b = hex::decode(clean).ok()?;
    if b.len() != 20 { return None; }
    let mut a = [0u8; 20]; a.copy_from_slice(&b); Some(a)
}

