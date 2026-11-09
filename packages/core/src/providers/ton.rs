use async_trait::async_trait;
use std::time::Duration;

use crate::chain::Chain;
use crate::error::{CoreError, Result};
use crate::providers::factory::ChainSettings;
use crate::traits::ChainTraits;

pub struct TonProvider {
    chain: Chain,
    api_urls: Vec<String>,
    client: reqwest::Client,
}

impl TonProvider {
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
impl ChainTraits for TonProvider {
    fn chain(&self) -> Chain { self.chain }

    async fn get_balance(&self, address: &str) -> Result<u128> {
        let addr = address.trim();
        if addr.is_empty() {
            return Err(CoreError::InvalidAddress("empty TON address".into()));
        }

        let mut errors = Vec::new();
        for base in &self.api_urls {
            // base is like https://toncenter.com/api/v2
            let url = format!("{base}/getAddressBalance?address={addr}");
            match self.client.get(&url).send().await {
                Ok(resp) => {
                    if !resp.status().is_success() {
                        errors.push(format!("{url}: HTTP {}", resp.status()));
                        continue;
                    }
                    let json: serde_json::Value = resp.json().await?;
                    if let Some(ok) = json.get("ok").and_then(|v| v.as_bool()) {
                        if !ok {
                            errors.push(format!("{url}: ok=false"));
                            continue;
                        }
                    }
                    let result = json.get("result").and_then(|v| v.as_str());
                    match result {
                        Some(s) => {
                            let n: u128 = s.parse()
                                .map_err(|e| CoreError::Rpc(format!("TON balance parse: {e}")))?;
                            return Ok(n);
                        }
                        None => {
                            errors.push(format!("{url}: missing result"));
                            continue;
                        }
                    }
                }
                Err(e) => errors.push(format!("{url}: {e}")),
            }
        }
        Err(CoreError::Rpc(format!("all TON endpoints failed: {}", errors.join(" | "))))
    }

    async fn estimate_fee(&self, _from: &str, _to: &str, _amount: u128) -> Result<u128> {
        Err(CoreError::NotImplemented("TON estimate_fee (Phase 1)"))
    }

    async fn get_nonce(&self, _address: &str) -> Result<u64> {
        Err(CoreError::NotImplemented("TON get_nonce (seqno)"))
    }

    async fn get_gas_price(&self) -> Result<u128> {
        Err(CoreError::NotImplemented("TON get_gas_price"))
    }

    async fn broadcast(&self, _raw_tx_hex: &str) -> Result<String> {
        Err(CoreError::NotImplemented("TON broadcast (BOC base64)"))
    }

    async fn health(&self) -> Result<bool> {
        for base in &self.api_urls {
            let url = format!("{base}/getMasterchainInfo");
            if let Ok(resp) = self.client.get(&url).send().await {
                if resp.status().is_success() {
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }
}
