use async_trait::async_trait;

use crate::chain::Chain;
use crate::error::{CoreError, Result};

#[async_trait]
pub trait ChainTraits: Send + Sync {
    fn chain(&self) -> Chain;

    async fn get_balance(&self, address: &str) -> Result<u128>;
    async fn estimate_fee(&self, from: &str, to: &str, amount: u128) -> Result<u128>;
    async fn get_nonce(&self, address: &str) -> Result<u64>;
    async fn get_gas_price(&self) -> Result<u128>;
    async fn broadcast(&self, raw_tx_hex: &str) -> Result<String>;
    async fn health(&self) -> Result<bool>;

    async fn get_erc20_balance(&self, _c: &str, _h: &str) -> Result<u128> {
        Err(CoreError::NotImplemented("ERC-20 balance (non-EVM)"))
    }
    async fn get_erc20_decimals(&self, _c: &str) -> Result<u8> {
        Err(CoreError::NotImplemented("ERC-20 decimals (non-EVM)"))
    }
    async fn get_erc20_symbol(&self, _c: &str) -> Result<String> {
        Err(CoreError::NotImplemented("ERC-20 symbol (non-EVM)"))
    }
    async fn get_erc20_name(&self, _c: &str) -> Result<String> {
        Err(CoreError::NotImplemented("ERC-20 name (non-EVM)"))
    }
    async fn get_base_fee(&self) -> Result<u128> {
        Err(CoreError::NotImplemented("base fee (non-EVM)"))
    }
    async fn get_max_priority_fee(&self) -> Result<u128> {
        Err(CoreError::NotImplemented("priority fee (non-EVM)"))
    }

    /// AICID: resolve `@name` -> address. Ok(None) if name is free.
    async fn resolve_name(&self, _contract: &str, _name: &str) -> Result<Option<String>> {
        Err(CoreError::NotImplemented("AIC-ID resolve (non-EVM)"))
    }
    /// AICID: reverse-resolve address -> name. Ok(None) if no name bound.
    async fn reverse_resolve(&self, _contract: &str, _address: &str) -> Result<Option<String>> {
        Err(CoreError::NotImplemented("AIC-ID reverse-resolve (non-EVM)"))
    }
}
