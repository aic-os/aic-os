use async_trait::async_trait;
use std::time::Duration;

use crate::chain::Chain;
use crate::error::{CoreError, Result};
use crate::providers::factory::ChainSettings;
use crate::traits::ChainTraits;

pub struct SolanaProvider {
    chain: Chain,
    rpc_urls: Vec<String>,
    client: reqwest::Client,
}

impl SolanaProvider {
    pub fn new(chain: Chain, settings: ChainSettings) -> Self {
        let client = reqwest::Client::builder()
            .user_agent("aic-core/0.1")
            .timeout(Duration::from_secs(5))
            .connect_timeout(Duration::from_secs(5))
            .build()
            .expect("http client");
        Self { chain, rpc_urls: settings.rpc_urls, client }
    }
}

#[async_trait]
impl ChainTraits for SolanaProvider {
    fn chain(&self) -> Chain { self.chain }

    async fn get_balance(&self, address: &str) -> Result<u128> {
        let addr = address.trim();
        if addr.is_empty() {
            return Err(CoreError::InvalidAddress("empty Solana address".into()));
        }
        // Base58 sanity: 32..44 chars.
        if addr.len() < 32 || addr.len() > 44 {
            return Err(CoreError::InvalidAddress(format!(
                "Solana address length must be 32..44, got {}", addr.len()
            )));
        }

        let body = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "getBalance",
            "params": [addr]
        });

        let mut errors = Vec::new();
        for url in &self.rpc_urls {
            match self.client.post(url).json(&body).send().await {
                Ok(resp) => {
                    if !resp.status().is_success() {
                        errors.push(format!("{url}: HTTP {}", resp.status()));
                        continue;
                    }
                    let json: serde_json::Value = resp.json().await?;
                    if let Some(err) = json.get("error") {
                        errors.push(format!("{url}: {err}"));
                        continue;
                    }
                    let lamports = json
                        .get("result")
                        .and_then(|r| r.get("value"))
                        .and_then(|v| v.as_u64());
                    match lamports {
                        Some(v) => return Ok(v as u128),
                        None => {
                            errors.push(format!("{url}: missing result.value"));
                            continue;
                        }
                    }
                }
                Err(e) => errors.push(format!("{url}: {e}")),
            }
        }
        Err(CoreError::Rpc(format!("all SOL endpoints failed: {}", errors.join(" | "))))
    }

    async fn estimate_fee(&self, _from: &str, _to: &str, _amount: u128) -> Result<u128> {
        Err(CoreError::NotImplemented("Solana estimate_fee (Phase 1)"))
    }

    async fn get_nonce(&self, _address: &str) -> Result<u64> {
        Err(CoreError::NotImplemented("Solana get_nonce (requires blockhash)"))
    }

    async fn get_gas_price(&self) -> Result<u128> {
        Err(CoreError::NotImplemented("Solana get_gas_price"))
    }

    async fn broadcast(&self, _raw_tx_hex: &str) -> Result<String> {
        Err(CoreError::NotImplemented("Solana broadcast (requires signed tx base64)"))
    }

    async fn health(&self) -> Result<bool> {
        let body = serde_json::json!({
            "jsonrpc": "2.0", "id": 1, "method": "getHealth", "params": []
        });
        for url in &self.rpc_urls {
            if let Ok(resp) = self.client.post(url).json(&body).send().await {
                if resp.status().is_success() {
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }
}
