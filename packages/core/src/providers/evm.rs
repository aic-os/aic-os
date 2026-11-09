use async_trait::async_trait;

use crate::chain::{Chain, ChainType};
use crate::error::{CoreError, Result};
use crate::providers::factory::ChainSettings;
use crate::traits::ChainTraits;

pub struct EvmProvider {
    chain: Chain,
    settings: ChainSettings,
    client: reqwest::Client,
}

impl EvmProvider {
    pub fn new(chain: Chain, settings: ChainSettings) -> Self {
        debug_assert_eq!(chain.chain_type(), ChainType::Evm);
        let client = reqwest::Client::builder()
            .user_agent("aic-core/0.1")
            .build()
            .expect("http client");
        Self { chain, settings, client }
    }

    pub fn rpc_url(&self) -> &str { &self.settings.rpc_url }
    pub fn chain_id(&self) -> Option<u64> { self.settings.chain_id }

    async fn rpc_call(&self, method: &str, params: serde_json::Value) -> Result<serde_json::Value> {
        let body = serde_json::json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
            "id": 1
        });

        let resp = self.client
            .post(&self.settings.rpc_url)
            .json(&body)
            .send()
            .await
            .map_err(|e| CoreError::Rpc(e.to_string()))?;

        let status = resp.status();
        let text = resp.text().await?;

        let json: serde_json::Value = serde_json::from_str(&text).map_err(|_| {
            let preview: String = text.chars().take(200).collect();
            CoreError::Rpc(format!(
                "non-JSON response from {} (status {}): {}",
                self.settings.rpc_url, status, preview
            ))
        })?;

        if let Some(err) = json.get("error") {
            return Err(CoreError::Rpc(err.to_string()));
        }
        json.get("result")
            .cloned()
            .ok_or_else(|| CoreError::Rpc("missing result".to_string()))
    }
}

fn parse_hex_u128(hex: &str) -> Result<u128> {
    let s = hex.trim_start_matches("0x");
    if s.is_empty() { return Ok(0); }
    u128::from_str_radix(s, 16)
        .map_err(|_| CoreError::Rpc(format!("invalid hex: {hex}")))
}

#[async_trait]
impl ChainTraits for EvmProvider {
    fn chain(&self) -> Chain { self.chain }

    async fn get_balance(&self, address: &str) -> Result<u128> {
        let addr = address.trim();
        if !addr.starts_with("0x") || addr.len() != 42 {
            return Err(CoreError::InvalidAddress(address.to_string()));
        }
        let res = self.rpc_call("eth_getBalance", serde_json::json!([addr, "latest"])).await?;
        let hex = res.as_str().ok_or_else(|| CoreError::Rpc("result is not a string".into()))?;
        parse_hex_u128(hex)
    }

    async fn estimate_fee(&self, _from: &str, _to: &str, _amount: u128) -> Result<u128> {
        Err(CoreError::NotImplemented("EvmProvider::estimate_fee (Phase 1.2)"))
    }

    async fn broadcast(&self, _raw_tx_hex: &str) -> Result<String> {
        Err(CoreError::NotImplemented("EvmProvider::broadcast (Phase 1.3)"))
    }

    async fn health(&self) -> Result<bool> {
        let res = self.rpc_call("eth_blockNumber", serde_json::json!([])).await?;
        Ok(res.as_str().is_some())
    }
}
