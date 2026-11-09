use async_trait::async_trait;
use std::time::Duration;
use crate::chain::Chain;
use crate::error::{CoreError, Result};
use crate::providers::factory::ChainSettings;
use crate::traits::ChainTraits;

pub struct CkbProvider { chain: Chain, api_urls: Vec<String>, client: reqwest::Client }
impl CkbProvider {
    pub fn new(chain: Chain, s: ChainSettings) -> Self {
        let client = reqwest::Client::builder().user_agent("aic-core/0.1")
            .timeout(Duration::from_secs(6)).build().expect("http");
        Self { chain, api_urls: s.rpc_urls, client }
    }
}
#[async_trait]
impl ChainTraits for CkbProvider {
    fn chain(&self) -> Chain { self.chain }
    async fn get_balance(&self, address: &str) -> Result<u128> {
        // Explorer API: /api/v1/addresses/{addr}
        let mut errors = Vec::new();
        for base in &self.api_urls {
            let url = format!("{base}/api/v1/addresses/{address}");
            match self.client.get(&url).send().await {
                Ok(resp) => {
                    if !resp.status().is_success() { errors.push(format!("{url}: HTTP {}", resp.status())); continue; }
                    let json: serde_json::Value = resp.json().await?;
                    // data.attributes.balance as string
                    if let Some(bal) = json.get("data").and_then(|d| d.get("attributes")).and_then(|a| a.get("balance")).and_then(|b| b.as_str()) {
                        return Ok(bal.parse().unwrap_or(0));
                    }
                    errors.push(format!("{url}: no data.attributes.balance"));
                }
                Err(e) => errors.push(format!("{url}: {e}")),
            }
        }
        Err(CoreError::Rpc(format!("CKB: {}", errors.join(" | "))))
    }
    async fn estimate_fee(&self, _f:&str,_t:&str,_a:u128)->Result<u128>{Err(CoreError::NotImplemented("CKB fee"))}
    async fn get_nonce(&self, _a:&str)->Result<u64>{Err(CoreError::NotImplemented("CKB nonce"))}
    async fn get_gas_price(&self)->Result<u128>{Err(CoreError::NotImplemented("CKB gas"))}
    async fn broadcast(&self,_r:&str)->Result<String>{Err(CoreError::NotImplemented("CKB broadcast"))}
    async fn health(&self)->Result<bool>{
        for base in &self.api_urls {
            let url = format!("{base}/api/v1/blocks");
            if let Ok(r) = self.client.get(&url).send().await { if r.status().is_success() { return Ok(true); } }
        }
        Ok(false)
    }
}
