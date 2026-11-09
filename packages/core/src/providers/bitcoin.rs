use async_trait::async_trait;
use std::time::Duration;

use crate::chain::Chain;
use crate::error::{CoreError, Result};
use crate::providers::factory::ChainSettings;
use crate::traits::ChainTraits;

pub struct BitcoinProvider {
    chain: Chain,
    api_urls: Vec<String>,
    client: reqwest::Client,
}

impl BitcoinProvider {
    pub fn new(chain: Chain, settings: ChainSettings) -> Self {
        let client = reqwest::Client::builder()
            .user_agent("aic-core/0.1")
            .timeout(Duration::from_secs(5))
            .connect_timeout(Duration::from_secs(5))
            .build()
            .expect("http client");
        Self { chain, api_urls: settings.rpc_urls, client }
    }
}

#[async_trait]
impl ChainTraits for BitcoinProvider {
    fn chain(&self) -> Chain { self.chain }

    async fn get_balance(&self, address: &str) -> Result<u128> {
        let addr = address.trim();
        if addr.is_empty() {
            return Err(CoreError::InvalidAddress("empty Bitcoin address".into()));
        }

        let mut errors = Vec::new();
        for base in &self.api_urls {
            let url = format!("{base}/address/{addr}");
            match self.client.get(&url).send().await {
                Ok(resp) => {
                    if !resp.status().is_success() {
                        errors.push(format!("{url}: HTTP {}", resp.status()));
                        continue;
                    }
                    let json: serde_json::Value = resp.json().await?;
                    let chain_stats = match json.get("chain_stats") {
                        Some(v) => v,
                        None => {
                            errors.push(format!("{url}: no chain_stats"));
                            continue;
                        }
                    };
                    let funded = chain_stats.get("funded_txo_sum").and_then(|v| v.as_u64()).unwrap_or(0);
                    let spent = chain_stats.get("spent_txo_sum").and_then(|v| v.as_u64()).unwrap_or(0);
                    let balance_sats = funded.saturating_sub(spent) as u128;
                    return Ok(balance_sats);
                }
                Err(e) => errors.push(format!("{url}: {e}")),
            }
        }
        Err(CoreError::Rpc(format!("all BTC endpoints failed: {}", errors.join(" | "))))
    }

    async fn estimate_fee(&self, _from: &str, _to: &str, _amount: u128) -> Result<u128> {
        Err(CoreError::NotImplemented("Bitcoin estimate_fee (Phase 1)"))
    }

    async fn get_nonce(&self, _address: &str) -> Result<u64> {
        Err(CoreError::NotImplemented("Bitcoin get_nonce"))
    }

    async fn get_gas_price(&self) -> Result<u128> {
        Err(CoreError::NotImplemented("Bitcoin get_gas_price"))
    }

    async fn broadcast(&self, _raw_tx_hex: &str) -> Result<String> {
        Err(CoreError::NotImplemented("Bitcoin broadcast"))
    }

    async fn health(&self) -> Result<bool> {
        let mut ok = false;
        for base in &self.api_urls {
            let url = format!("{base}/blocks/tip/height");
            if self.client.get(&url).send().await.is_ok() {
                ok = true;
                break;
            }
        }
        Ok(ok)
    }
}
