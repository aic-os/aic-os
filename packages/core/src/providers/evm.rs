use async_trait::async_trait;
use std::time::Duration;

use crate::chain::{Chain, ChainType};
use crate::error::{CoreError, Result};
use crate::providers::factory::ChainSettings;
use crate::traits::ChainTraits;

const RPC_TIMEOUT_SECS: u64 = 5;

pub struct EvmProvider {
    chain: Chain,
    chain_id: Option<u64>,
    rpc_urls: Vec<String>,
    client: reqwest::Client,
}

impl EvmProvider {
    pub fn new(chain: Chain, settings: ChainSettings) -> Self {
        debug_assert_eq!(chain.chain_type(), ChainType::Evm);
        let client = reqwest::Client::builder()
            .user_agent("aic-core/0.1")
            .timeout(Duration::from_secs(RPC_TIMEOUT_SECS))
            .connect_timeout(Duration::from_secs(RPC_TIMEOUT_SECS))
            .build()
            .expect("http client");
        Self {
            chain,
            chain_id: settings.chain_id,
            rpc_urls: settings.rpc_urls,
            client,
        }
    }

    pub fn rpc_urls(&self) -> &[String] { &self.rpc_urls }
    pub fn chain_id(&self) -> Option<u64> { self.chain_id }

    async fn rpc_call(&self, method: &str, params: serde_json::Value) -> Result<serde_json::Value> {
        if self.rpc_urls.is_empty() {
            return Err(CoreError::Rpc("no rpc urls configured".into()));
        }

        let body = serde_json::json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
            "id": 1
        });

        let mut errors = Vec::new();
        for url in &self.rpc_urls {
            match self.rpc_call_one(url, &body).await {
                Ok(v) => return Ok(v),
                Err(e) => errors.push(format!("{url}: {e}")),
            }
        }
        Err(CoreError::Rpc(format!(
            "all rpc endpoints failed for {method}: {}",
            errors.join(" | ")
        )))
    }

    async fn rpc_call_one(
        &self,
        url: &str,
        body: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        let resp = self.client
            .post(url)
            .json(body)
            .send()
            .await
            .map_err(|e| CoreError::Rpc(e.to_string()))?;

        let status = resp.status();
        let text = resp.text().await.map_err(|e| CoreError::Rpc(e.to_string()))?;

        let json: serde_json::Value = serde_json::from_str(&text).map_err(|_| {
            let preview: String = text.chars().take(160).collect();
            CoreError::Rpc(format!("non-JSON (status {status}): {preview}"))
        })?;

        if let Some(err) = json.get("error") {
            return Err(CoreError::Rpc(err.to_string()));
        }
        json.get("result")
            .cloned()
            .ok_or_else(|| CoreError::Rpc("missing result".into()))
    }
}

fn is_evm_address(s: &str) -> bool {
    s.len() == 42 && s.starts_with("0x") && s[2..].chars().all(|c| c.is_ascii_hexdigit())
}

fn is_hex_string(s: &str) -> bool {
    s.len() > 2 && s.starts_with("0x") && s[2..].chars().all(|c| c.is_ascii_hexdigit())
}

fn parse_hex_u128(hex: &str) -> Result<u128> {
    let s = hex.trim_start_matches("0x");
    if s.is_empty() { return Ok(0); }
    u128::from_str_radix(s, 16)
        .map_err(|_| CoreError::Rpc(format!("invalid hex: {hex}")))
}

fn hex_result_u128(v: &serde_json::Value) -> Result<u128> {
    let s = v.as_str().ok_or_else(|| CoreError::Rpc("result is not a string".into()))?;
    parse_hex_u128(s)
}

#[async_trait]
impl ChainTraits for EvmProvider {
    fn chain(&self) -> Chain { self.chain }

    async fn get_balance(&self, address: &str) -> Result<u128> {
        let addr = address.trim();
        if !is_evm_address(addr) {
            return Err(CoreError::InvalidAddress(address.to_string()));
        }
        let res = self.rpc_call("eth_getBalance", serde_json::json!([addr, "latest"])).await?;
        hex_result_u128(&res)
    }

    async fn estimate_fee(&self, from: &str, to: &str, amount: u128) -> Result<u128> {
        let from = from.trim();
        let to = to.trim();
        if !is_evm_address(from) { return Err(CoreError::InvalidAddress(from.to_string())); }
        if !is_evm_address(to) { return Err(CoreError::InvalidAddress(to.to_string())); }

        let gas_price_res = self.rpc_call("eth_gasPrice", serde_json::json!([])).await?;
        let gas_price = hex_result_u128(&gas_price_res)?;

        let tx = serde_json::json!({
            "from": from,
            "to": to,
            "value": format!("0x{:x}", amount),
        });
        let gas_res = self.rpc_call("eth_estimateGas", serde_json::json!([tx])).await?;
        let gas = hex_result_u128(&gas_res)?;

        gas_price.checked_mul(gas)
            .ok_or_else(|| CoreError::Rpc("fee overflow (u128)".into()))
    }

    async fn broadcast(&self, raw_tx_hex: &str) -> Result<String> {
        let raw = raw_tx_hex.trim();
        if !is_hex_string(raw) {
            return Err(CoreError::Rpc(format!(
                "invalid raw tx hex (expected 0x-prefixed even-length hex, got {} chars)",
                raw.len()
            )));
        }
        if (raw.len() - 2) % 2 != 0 {
            return Err(CoreError::Rpc(
                "invalid raw tx hex: odd number of hex chars".into(),
            ));
        }

        let res = self.rpc_call("eth_sendRawTransaction", serde_json::json!([raw])).await?;
        let hash = res.as_str()
            .ok_or_else(|| CoreError::Rpc("eth_sendRawTransaction: result is not a string".into()))?;
        Ok(hash.to_string())
    }

    async fn health(&self) -> Result<bool> {
        let res = self.rpc_call("eth_blockNumber", serde_json::json!([])).await?;
        Ok(res.as_str().is_some())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_validation_accepts_valid() {
        assert!(is_hex_string("0xdeadbeef"));
        assert!(is_hex_string("0x00"));
        assert!(is_hex_string("0x12345")); // hex chars, even if odd length; length check is separate
    }

    #[test]
    fn hex_validation_rejects_invalid() {
        assert!(!is_hex_string(""));
        assert!(!is_hex_string("0x"));
        assert!(!is_hex_string("deadbeef"));
        assert!(!is_hex_string("0xzz"));
    }

    #[test]
    fn address_validation() {
        assert!(is_evm_address("0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045"));
        assert!(!is_evm_address("0x123"));
        assert!(!is_evm_address("not_an_address"));
    }
}
