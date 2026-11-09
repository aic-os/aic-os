use async_trait::async_trait;
use std::time::Duration;

use crate::chain::Chain;
use crate::error::{CoreError, Result};
use crate::providers::factory::ChainSettings;
use crate::traits::ChainTraits;

pub struct XrpProvider {
    chain: Chain,
    rpc_urls: Vec<String>,
    client: reqwest::Client,
}

impl XrpProvider {
    pub fn new(chain: Chain, settings: ChainSettings) -> Self {
        let client = reqwest::Client::builder()
            .user_agent("aic-core/0.1")
            .timeout(Duration::from_secs(5))
            .build()
            .expect("http client");
        Self { chain, rpc_urls: settings.rpc_urls, client }
    }
}

#[async_trait]
impl ChainTraits for XrpProvider {
    fn chain(&self) -> Chain { self.chain }

    async fn get_balance(&self, address: &str) -> Result<u128> {
        let addr = address.trim();
        if addr.is_empty() || !addr.starts_with('r') {
            return Err(CoreError::InvalidAddress(format!("invalid XRP address: {addr}")));
        }

        let body = serde_json::json!({
            "method": "account_info",
            "params": [{"account": addr, "ledger_index": "validated"}]
        });

        let mut errors = Vec::new();
        for url in &self.rpc_urls {
            match self.client.post(url).json(&body).send().await {
                Ok(resp) => {
                    if !resp.status().is_success() {
                        errors.push(format!("{url}: HTTP {}", resp.status()));
                        continue;
                    }
                    let json: serde_json::Value = resp.json().await?;
                    // "actNotFound" means unfunded account -> 0 balance.
                    if let Some(err) = json.get("result").and_then(|r| r.get("error")) {
                        let code = err.as_str().unwrap_or("");
                        if code == "actNotFound" { return Ok(0); }
                        errors.push(format!("{url}: {err}"));
                        continue;
                    }
                    let drops = json
                        .get("result")
                        .and_then(|r| r.get("account_data"))
                        .and_then(|d| d.get("Balance"))
                        .and_then(|b| b.as_str());
                    match drops {
                        Some(s) => {
                            let n: u128 = s.parse().map_err(|e| CoreError::Rpc(format!("XRP parse: {e}")))?;
                            return Ok(n);
                        }
                        None => { errors.push(format!("{url}: no Balance")); continue; }
                    }
                }
                Err(e) => errors.push(format!("{url}: {e}")),
            }
        }
        Err(CoreError::Rpc(format!("all XRP endpoints failed: {}", errors.join(" | "))))
    }

    async fn estimate_fee(&self, _f: &str, _t: &str, _a: u128) -> Result<u128> { Err(CoreError::NotImplemented("XRP estimate_fee")) }
    async fn get_nonce(&self, _a: &str) -> Result<u64> { Err(CoreError::NotImplemented("XRP get_nonce")) }
    async fn get_gas_price(&self) -> Result<u128> { Err(CoreError::NotImplemented("XRP get_gas_price")) }
    async fn broadcast(&self, _r: &str) -> Result<String> { Err(CoreError::NotImplemented("XRP broadcast")) }

    async fn health(&self) -> Result<bool> {
        let body = serde_json::json!({"method":"ping","params":[{}]});
        for url in &self.rpc_urls {
            if let Ok(resp) = self.client.post(url).json(&body).send().await {
                if resp.status().is_success() { return Ok(true); }
            }
        }
        Ok(false)
    }
}
