use std::sync::Arc;
use axum::{routing::post, Router, Json};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio::net::TcpListener;

use crate::mempool::Mempool;
use crate::state::State;
use std::sync::RwLock;

pub struct AppState {
    pub mempool: Mempool,
    pub state: RwLock<State>,
    pub chain_id: u64,
}

#[derive(Deserialize)]
struct RpcRequest {
    #[serde(default)]
    id: Value,
    method: String,
    #[serde(default)]
    params: Value,
}

#[derive(Serialize)]
struct RpcResponse {
    jsonrpc: &'static str,
    id: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<Value>,
}

pub async fn run_rpc(addr: &str, chain_id: u64) -> Result<(), Box<dyn std::error::Error>> {
    let state = Arc::new(AppState {
        mempool: Mempool::new(),
        state: RwLock::new(State::new()),
        chain_id,
    });

    let app = Router::new()
        .route("/", post(handle_rpc))
        .with_state(state);

    let listener = TcpListener::bind(addr).await?;
    println!("AIC Node RPC listening on http://{addr}");
    axum::serve(listener, app).await?;
    Ok(())
}

async fn handle_rpc(
    axum::extract::State(st): axum::extract::State<Arc<AppState>>,
    Json(req): Json<RpcRequest>,
) -> Json<RpcResponse> {
    let result = match req.method.as_str() {
        "aic_chainId" => Some(json!(st.chain_id)),
        "aic_blockNumber" => Some(json!(0)),
        "aic_mempoolSize" => {
            let n = st.mempool.len().await;
            Some(json!(n))
        }
        "aic_getBalance" => {
            if let Some(addr_hex) = req.params.get(0).and_then(|v| v.as_str()) {
                if let Ok(bytes) = hex::decode(addr_hex.trim_start_matches("0x")) {
                    if bytes.len() == 20 {
                        let mut addr = [0u8; 20];
                        addr.copy_from_slice(&bytes);
                        let s = st.state.read().unwrap();
                        let acc = s.get_account(&addr);
                        Some(json!(format!("0x{:x}", acc.balance)))
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        }
        _ => None,
    };

    // Считаем error ДО перемещения result в структуру.
    let error = if result.is_none() {
        Some(json!({"code": -32601, "message": "method not found"}))
    } else {
        None
    };

    Json(RpcResponse {
        jsonrpc: "2.0",
        id: req.id,
        result,
        error,
    })
}
