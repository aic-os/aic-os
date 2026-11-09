use async_trait::async_trait;
use std::time::Duration;
use crate::chain::Chain;
use crate::error::{CoreError, Result};
use crate::providers::factory::ChainSettings;
use crate::traits::ChainTraits;

pub struct ArweaveProvider { chain: Chain, api_urls: Vec<String>, client: reqwest::Client }
impl ArweaveProvider {
    pub fn new(chain: Chain, s: ChainSettings) -> Self {
        let client = reqwest::Client::builder().user_agent("aic-core/0.1")
            .timeout(Duration::from_secs(8)).build().expect("http");
        Self { chain, api_urls: s.rpc_urls, client }
    }
}
#[async_trait]
impl ChainTraits for ArweaveProvider {
    fn chain(&self) -> Chain { self.chain }
    async fn get_balance(&self, address: &str) -> Result<u128> {
        // REST: /wallet/{address}/balance returns winston as string
        let mut errors = Vec::new();
        for base in &self.api_urls {
            let url = format!("{base}/wallet/{address}/balance");
            match self.client.get(&url).send().await {
                Ok(resp) => {
                    if !resp.status().is_success() { errors.push(format!("{url}: HTTP {}", resp.status())); continue; }
                    let txt = resp.text().await.unwrap_or_default();
                    if let Ok(n) = txt.trim().parse::<u128>() {
                        return Ok(n);
                    }
                    errors.push(format!("{url}: not u128 (got {:?})", &txt[..txt.len().min(30)]));
                }
                Err(e) => errors.push(format!("{url}: {e}")),
            }
        }
        Err(CoreError::Rpc(format!("AR: {}", errors.join(" | "))))
    }
    async fn estimate_fee(&self, _f:&str,_t:&str,_a:u128)->Result<u128>{Err(CoreError::NotImplemented("AR fee"))}
    async fn get_nonce(&self, _a:&str)->Result<u64>{Err(CoreError::NotImplemented("AR nonce"))}
    async fn get_gas_price(&self)->Result<u128>{Err(CoreError::NotImplemented("AR gas"))}
    async fn broadcast(&self,_r:&str)->Result<String>{Err(CoreError::NotImplemented("AR broadcast"))}
    async fn health(&self)->Result<bool>{
        for base in &self.api_urls {
            let url = format!("{base}/info");
            if let Ok(r) = self.client.get(&url).send().await { if r.status().is_success() { return Ok(true); } }
        }
        Ok(false)
    }
}
