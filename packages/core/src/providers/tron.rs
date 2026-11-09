use async_trait::async_trait;
use std::time::Duration;

use crate::chain::Chain;
use crate::error::{CoreError, Result};
use crate::providers::factory::ChainSettings;
use crate::traits::ChainTraits;

pub struct TronProvider {
    chain: Chain,
    api_urls: Vec<String>,
    client: reqwest::Client,
}

impl TronProvider {
    pub fn new(chain: Chain, settings: ChainSettings) -> Self {
        let client = reqwest::Client::builder()
            .user_agent("aic-core/0.1")
            .timeout(Duration::from_secs(5))
            .build()
            .expect("http client");
        Self { chain, api_urls: settings.rpc_urls, client }
    }
}

#[async_trait]
impl ChainTraits for TronProvider {
    fn chain(&self) -> Chain { self.chain }

    async fn get_balance(&self, address: &str) -> Result<u128> {
        let addr = address.trim();
        if addr.is_empty() || !addr.starts_with('T') {
            return Err(CoreError::InvalidAddress(format!("invalid TRON address: {addr}")));
        }

        let mut errors = Vec::new();
        for base in &self.api_urls {
            let url = format!("{base}/v1/accounts/{addr}");
            match self.client.get(&url).send().await {
                Ok(resp) => {
                    if !resp.status().is_success() {
                        errors.push(format!("{url}: HTTP {}", resp.status()));
                        continue;
                    }
                    let json: serde_json::Value = resp.json().await?;
                    let data = json.get("data").and_then(|v| v.as_array());
                    match data {
                        Some(arr) if !arr.is_empty() => {
                            let bal = arr[0].get("balance").and_then(|v| v.as_u64()).unwrap_or(0);
                            return Ok(bal as u128);
                        }
                        _ => return Ok(0),
                    }
                }
                Err(e) => errors.push(format!("{url}: {e}")),
            }
        }
        Err(CoreError::Rpc(format!("all TRON endpoints failed: {}", errors.join(" | "))))
    }

    async fn estimate_fee(&self, _f: &str, _t: &str, _a: u128) -> Result<u128> { Err(CoreError::NotImplemented("TRON estimate_fee")) }
    async fn get_nonce(&self, _a: &str) -> Result<u64> { Err(CoreError::NotImplemented("TRON get_nonce")) }
    async fn get_gas_price(&self) -> Result<u128> { Err(CoreError::NotImplemented("TRON get_gas_price")) }
    async fn broadcast(&self, _r: &str) -> Result<String> { Err(CoreError::NotImplemented("TRON broadcast")) }

    async fn health(&self) -> Result<bool> {
        for base in &self.api_urls {
            let url = format!("{base}/wallet/getnowblock");
            if let Ok(r) = self.client.get(&url).send().await {
                if r.status().is_success() { return Ok(true); }
            }
        }
        Ok(false)
    }
}
