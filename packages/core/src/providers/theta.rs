use async_trait::async_trait;
use std::time::Duration;
use crate::chain::Chain;
use crate::error::{CoreError, Result};
use crate::providers::factory::ChainSettings;
use crate::traits::ChainTraits;

pub struct ThetaProvider { chain: Chain, api_urls: Vec<String>, client: reqwest::Client }
impl ThetaProvider {
    pub fn new(chain: Chain, s: ChainSettings) -> Self {
        let client = reqwest::Client::builder().user_agent("aic-core/0.1")
            .timeout(Duration::from_secs(6)).build().expect("http");
        Self { chain, api_urls: s.rpc_urls, client }
    }
}
#[async_trait]
impl ChainTraits for ThetaProvider {
    fn chain(&self) -> Chain { self.chain }
    async fn get_balance(&self, address: &str) -> Result<u128> {
        let mut errors = Vec::new();
        for base in &self.api_urls {
            let url = format!("{base}/account/{address}");
            match self.client.get(&url).send().await {
                Ok(resp) => {
                    if !resp.status().is_success() { errors.push(format!("{url}: HTTP {}", resp.status())); continue; }
                    let json: serde_json::Value = resp.json().await?;
                    if let Some(bal) = json.get("body").and_then(|b| b.get("coins")).and_then(|c| c.get("thetawei")).and_then(|v| v.as_str()) {
                        return Ok(bal.parse().unwrap_or(0));
                    }
                    errors.push(format!("{url}: no body.coins.thetawei"));
                }
                Err(e) => errors.push(format!("{url}: {e}")),
            }
        }
        Err(CoreError::Rpc(format!("THETA: {}", errors.join(" | "))))
    }
    async fn estimate_fee(&self, _f:&str,_t:&str,_a:u128)->Result<u128>{Err(CoreError::NotImplemented("THETA fee"))}
    async fn get_nonce(&self, _a:&str)->Result<u64>{Err(CoreError::NotImplemented("THETA nonce"))}
    async fn get_gas_price(&self)->Result<u128>{Err(CoreError::NotImplemented("THETA gas"))}
    async fn broadcast(&self,_r:&str)->Result<String>{Err(CoreError::NotImplemented("THETA broadcast"))}
    async fn health(&self)->Result<bool>{
        for base in &self.api_urls {
            let url = format!("{base}/status");
            if let Ok(r) = self.client.get(&url).send().await { if r.status().is_success() { return Ok(true); } }
        }
        Ok(false)
    }
}
