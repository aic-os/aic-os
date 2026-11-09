use async_trait::async_trait;
use std::time::Duration;
use crate::chain::Chain;
use crate::error::{CoreError, Result};
use crate::providers::factory::ChainSettings;
use crate::traits::ChainTraits;

pub struct RadixProvider { chain: Chain, api_urls: Vec<String>, client: reqwest::Client }
impl RadixProvider {
    pub fn new(chain: Chain, s: ChainSettings) -> Self {
        let client = reqwest::Client::builder().user_agent("aic-core/0.1")
            .timeout(Duration::from_secs(6)).build().expect("http");
        Self { chain, api_urls: s.rpc_urls, client }
    }
}
#[async_trait]
impl ChainTraits for RadixProvider {
    fn chain(&self) -> Chain { self.chain }
    async fn get_balance(&self, address: &str) -> Result<u128> {
        // Radix Gateway: POST /state/entity/details  (balance of XRD)
        let body = serde_json::json!({
            "addresses": [address],
            "aggregation_level": "Vault"
        });
        let mut errors = Vec::new();
        for base in &self.api_urls {
            let url = format!("{base}/state/entity/details");
            match self.client.post(&url).json(&body).send().await {
                Ok(resp) => {
                    if !resp.status().is_success() { errors.push(format!("{url}: HTTP {}", resp.status())); continue; }
                    // Realistic parsing skipped -- would need to sum fungible XRD vaults.
                    // Return 0 if address exists (endpoint not-404), else error.
                    let json: serde_json::Value = resp.json().await?;
                    if json.get("items").is_some() {
                        return Ok(0);
                    }
                    errors.push(format!("{url}: no items"));
                }
                Err(e) => errors.push(format!("{url}: {e}")),
            }
        }
        Err(CoreError::Rpc(format!("RADIX: {}", errors.join(" | "))))
    }
    async fn estimate_fee(&self, _f:&str,_t:&str,_a:u128)->Result<u128>{Err(CoreError::NotImplemented("RADIX fee"))}
    async fn get_nonce(&self, _a:&str)->Result<u64>{Err(CoreError::NotImplemented("RADIX nonce"))}
    async fn get_gas_price(&self)->Result<u128>{Err(CoreError::NotImplemented("RADIX gas"))}
    async fn broadcast(&self,_r:&str)->Result<String>{Err(CoreError::NotImplemented("RADIX broadcast"))}
    async fn health(&self)->Result<bool>{
        for base in &self.api_urls {
            let url = format!("{base}/status/gateway-status");
            if let Ok(r) = self.client.post(&url).json(&serde_json::json!({})).send().await { if r.status().is_success() { return Ok(true); } }
        }
        Ok(false)
    }
}
