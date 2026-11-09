use async_trait::async_trait;
use std::time::Duration;
use crate::chain::Chain;
use crate::error::{CoreError, Result};
use crate::providers::factory::ChainSettings;
use crate::traits::ChainTraits;

pub struct AlgorandProvider { chain: Chain, api_urls: Vec<String>, client: reqwest::Client }
impl AlgorandProvider {
    pub fn new(chain: Chain, s: ChainSettings) -> Self {
        let client = reqwest::Client::builder().user_agent("aic-core/0.1")
            .timeout(Duration::from_secs(6)).build().expect("http");
        Self { chain, api_urls: s.rpc_urls, client }
    }
}
#[async_trait]
impl ChainTraits for AlgorandProvider {
    fn chain(&self) -> Chain { self.chain }
    async fn get_balance(&self, address: &str) -> Result<u128> {
        let mut errors = Vec::new();
        for base in &self.api_urls {
            let url = format!("{base}/v2/accounts/{address}");
            match self.client.get(&url).send().await {
                Ok(resp) => {
                    if resp.status() == 404 { return Ok(0); }
                    if !resp.status().is_success() { errors.push(format!("{url}: HTTP {}", resp.status())); continue; }
                    let json: serde_json::Value = resp.json().await?;
                    if let Some(amount) = json.get("account").and_then(|a| a.get("amount")).and_then(|v| v.as_u64()) {
                        return Ok(amount as u128);
                    }
                    errors.push(format!("{url}: no amount"));
                }
                Err(e) => errors.push(format!("{url}: {e}")),
            }
        }
        Err(CoreError::Rpc(format!("ALGO: {}", errors.join(" | "))))
    }
    async fn estimate_fee(&self, _f:&str,_t:&str,_a:u128)->Result<u128>{Err(CoreError::NotImplemented("ALGO fee"))}
    async fn get_nonce(&self, _a:&str)->Result<u64>{Err(CoreError::NotImplemented("ALGO nonce"))}
    async fn get_gas_price(&self)->Result<u128>{Err(CoreError::NotImplemented("ALGO gas"))}
    async fn broadcast(&self,_r:&str)->Result<String>{Err(CoreError::NotImplemented("ALGO broadcast"))}
    async fn health(&self)->Result<bool>{
        for base in &self.api_urls {
            let url = format!("{base}/health");
            if let Ok(r) = self.client.get(&url).send().await { if r.status().is_success() { return Ok(true); } }
        }
        Ok(false)
    }
}
