use async_trait::async_trait;
use std::time::Duration;
use crate::chain::Chain;
use crate::error::{CoreError, Result};
use crate::providers::factory::ChainSettings;
use crate::traits::ChainTraits;

pub struct NearProvider { chain: Chain, api_urls: Vec<String>, client: reqwest::Client }
impl NearProvider {
    pub fn new(chain: Chain, s: ChainSettings) -> Self {
        let client = reqwest::Client::builder().user_agent("aic-core/0.1")
            .timeout(Duration::from_secs(6)).build().expect("http");
        Self { chain, api_urls: s.rpc_urls, client }
    }
}
#[async_trait]
impl ChainTraits for NearProvider {
    fn chain(&self) -> Chain { self.chain }
    async fn get_balance(&self, address: &str) -> Result<u128> {
        let body = serde_json::json!({
            "jsonrpc":"2.0","id":"dontcare","method":"query",
            "params":{"request_type":"view_account","finality":"final","account_id":address}
        });
        let mut errors = Vec::new();
        for url in &self.api_urls {
            match self.client.post(url).json(&body).send().await {
                Ok(resp) => {
                    let json: serde_json::Value = resp.json().await?;
                    if let Some(r) = json.get("result").and_then(|r| r.get("amount")).and_then(|a| a.as_str()) {
                        return Ok(r.parse().unwrap_or(0));
                    }
                    errors.push(format!("{url}: no amount"));
                }
                Err(e) => errors.push(format!("{url}: {e}")),
            }
        }
        Err(CoreError::Rpc(format!("NEAR: {}", errors.join(" | "))))
    }
    async fn estimate_fee(&self, _f:&str,_t:&str,_a:u128)->Result<u128>{Err(CoreError::NotImplemented("NEAR fee"))}
    async fn get_nonce(&self, _a:&str)->Result<u64>{Err(CoreError::NotImplemented("NEAR nonce"))}
    async fn get_gas_price(&self)->Result<u128>{Err(CoreError::NotImplemented("NEAR gas"))}
    async fn broadcast(&self,_r:&str)->Result<String>{Err(CoreError::NotImplemented("NEAR broadcast"))}
    async fn health(&self)->Result<bool>{
        for url in &self.api_urls {
            let body = serde_json::json!({"jsonrpc":"2.0","id":"dontcare","method":"status","params":[]});
            if let Ok(r) = self.client.post(url).json(&body).send().await { if r.status().is_success() { return Ok(true); } }
        }
        Ok(false)
    }
}
