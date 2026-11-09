use async_trait::async_trait;
use std::time::Duration;
use crate::chain::Chain;
use crate::error::{CoreError, Result};
use crate::providers::factory::ChainSettings;
use crate::traits::ChainTraits;

pub struct PolkadotProvider { chain: Chain, api_urls: Vec<String>, client: reqwest::Client }
impl PolkadotProvider {
    pub fn new(chain: Chain, s: ChainSettings) -> Self {
        let client = reqwest::Client::builder().user_agent("aic-core/0.1")
            .timeout(Duration::from_secs(6)).build().expect("http");
        Self { chain, api_urls: s.rpc_urls, client }
    }
}
#[async_trait]
impl ChainTraits for PolkadotProvider {
    fn chain(&self) -> Chain { self.chain }
    async fn get_balance(&self, address: &str) -> Result<u128> {
        // Substrate: use `state_getStorage` requires key mapping -- too complex for REST-style.
        // For simplicity, use Subscan public balance endpoint (works without key for basic queries).
        let mut errors = Vec::new();
        for base in &self.api_urls {
            // base like https://polkadot.api.subscan.io
            let url = format!("{base}/api/v2/scan/search");
            let body = serde_json::json!({ "key": address });
            match self.client.post(&url).json(&body).send().await {
                Ok(resp) => {
                    if !resp.status().is_success() { errors.push(format!("{url}: HTTP {}", resp.status())); continue; }
                    let json: serde_json::Value = resp.json().await?;
                    if let Some(bal) = json.get("data").and_then(|d| d.get("account")).and_then(|a| a.get("balance")).and_then(|b| b.as_str()) {
                        let f: f64 = bal.parse().unwrap_or(0.0);
                        return Ok((f * 1e10) as u128);
                    }
                    errors.push(format!("{url}: no data.account.balance"));
                }
                Err(e) => errors.push(format!("{url}: {e}")),
            }
        }
        Err(CoreError::Rpc(format!("DOT: {}", errors.join(" | "))))
    }
    async fn estimate_fee(&self, _f:&str,_t:&str,_a:u128)->Result<u128>{Err(CoreError::NotImplemented("DOT fee"))}
    async fn get_nonce(&self, _a:&str)->Result<u64>{Err(CoreError::NotImplemented("DOT nonce"))}
    async fn get_gas_price(&self)->Result<u128>{Err(CoreError::NotImplemented("DOT gas"))}
    async fn broadcast(&self,_r:&str)->Result<String>{Err(CoreError::NotImplemented("DOT broadcast"))}
    async fn health(&self)->Result<bool>{
        for base in &self.api_urls {
            let url = format!("{base}/api/scan/metadata");
            if let Ok(r) = self.client.get(&url).send().await { if r.status().is_success() { return Ok(true); } }
        }
        Ok(false)
    }
}
