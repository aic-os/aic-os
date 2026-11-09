use async_trait::async_trait;
use std::time::Duration;

use crate::chain::Chain;
use crate::error::{CoreError, Result};
use crate::providers::factory::ChainSettings;
use crate::traits::ChainTraits;

pub struct AptosProvider {
    chain: Chain,
    api_urls: Vec<String>,
    client: reqwest::Client,
}

impl AptosProvider {
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
impl ChainTraits for AptosProvider {
    fn chain(&self) -> Chain { self.chain }

    async fn get_balance(&self, address: &str) -> Result<u128> {
        let addr = address.trim();
        if addr.is_empty() {
            return Err(CoreError::InvalidAddress("empty Aptos address".into()));
        }

        let mut errors = Vec::new();
        for base in &self.api_urls {
            // base like https://fullnode.mainnet.aptoslabs.com/v1
            let url = format!("{base}/accounts/{addr}/resource/0x1::coin::CoinStore%3C0x1::aptos_coin::AptosCoin%3E");
            match self.client.get(&url).send().await {
                Ok(resp) => {
                    if resp.status() == 404 {
                        return Ok(0); // account not found
                    }
                    if !resp.status().is_success() {
                        errors.push(format!("{url}: HTTP {}", resp.status()));
                        continue;
                    }
                    let json: serde_json::Value = resp.json().await?;
                    let value = json
                        .get("data")
                        .and_then(|d| d.get("coin"))
                        .and_then(|c| c.get("value"))
                        .and_then(|v| v.as_str());
                    match value {
                        Some(s) => {
                            let n: u128 = s.parse().map_err(|e| CoreError::Rpc(format!("aptos parse: {e}")))?;
                            return Ok(n);
                        }
                        None => { errors.push(format!("{url}: no coin.value")); continue; }
                    }
                }
                Err(e) => errors.push(format!("{url}: {e}")),
            }
        }
        Err(CoreError::Rpc(format!("all Aptos endpoints failed: {}", errors.join(" | "))))
    }

    async fn estimate_fee(&self, _f: &str, _t: &str, _a: u128) -> Result<u128> { Err(CoreError::NotImplemented("Aptos estimate_fee")) }
    async fn get_nonce(&self, _a: &str) -> Result<u64> { Err(CoreError::NotImplemented("Aptos get_nonce (sequence_number)")) }
    async fn get_gas_price(&self) -> Result<u128> { Err(CoreError::NotImplemented("Aptos get_gas_price")) }
    async fn broadcast(&self, _r: &str) -> Result<String> { Err(CoreError::NotImplemented("Aptos broadcast (BCS)")) }

    async fn health(&self) -> Result<bool> {
        for base in &self.api_urls {
            // base ends with /v1
            if let Ok(resp) = self.client.get(base).send().await {
                if resp.status().is_success() { return Ok(true); }
            }
        }
        Ok(false)
    }
}
