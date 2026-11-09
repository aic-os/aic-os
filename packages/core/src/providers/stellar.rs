use async_trait::async_trait;
use std::time::Duration;
use crate::chain::Chain;
use crate::error::{CoreError, Result};
use crate::providers::factory::ChainSettings;
use crate::traits::ChainTraits;

pub struct StellarProvider { chain: Chain, api_urls: Vec<String>, client: reqwest::Client }
impl StellarProvider {
    pub fn new(chain: Chain, s: ChainSettings) -> Self {
        let client = reqwest::Client::builder().user_agent("aic-core/0.1")
            .timeout(Duration::from_secs(6)).build().expect("http");
        Self { chain, api_urls: s.rpc_urls, client }
    }
}
#[async_trait]
impl ChainTraits for StellarProvider {
    fn chain(&self) -> Chain { self.chain }
    async fn get_balance(&self, address: &str) -> Result<u128> {
        let mut errors = Vec::new();
        for base in &self.api_urls {
            let url = format!("{base}/accounts/{address}");
            match self.client.get(&url).send().await {
                Ok(resp) => {
                    if resp.status() == 404 { return Ok(0); }
                    if !resp.status().is_success() { errors.push(format!("{url}: HTTP {}", resp.status())); continue; }
                    let json: serde_json::Value = resp.json().await?;
                    if let Some(arr) = json.get("balances").and_then(|b| b.as_array()) {
                        for b in arr {
                            if b.get("asset_type").and_then(|a| a.as_str()) == Some("native") {
                                if let Some(bal) = b.get("balance").and_then(|v| v.as_str()) {
                                    // XLM = 7 decimals
                                    if let Ok(f) = bal.parse::<f64>() {
                                        return Ok((f * 10_000_000.0) as u128);
                                    }
                                }
                            }
                        }
                    }
                    errors.push(format!("{url}: no native balance"));
                }
                Err(e) => errors.push(format!("{url}: {e}")),
            }
        }
        Err(CoreError::Rpc(format!("XLM: {}", errors.join(" | "))))
    }
    async fn estimate_fee(&self, _f:&str,_t:&str,_a:u128)->Result<u128>{Err(CoreError::NotImplemented("XLM fee"))}
    async fn get_nonce(&self, _a:&str)->Result<u64>{Err(CoreError::NotImplemented("XLM nonce"))}
    async fn get_gas_price(&self)->Result<u128>{Err(CoreError::NotImplemented("XLM gas"))}
    async fn broadcast(&self,_r:&str)->Result<String>{Err(CoreError::NotImplemented("XLM broadcast"))}
    async fn health(&self)->Result<bool>{
        for base in &self.api_urls {
            let url = format!("{base}/fee_stats");
            if let Ok(r) = self.client.get(&url).send().await { if r.status().is_success() { return Ok(true); } }
        }
        Ok(false)
    }
}
