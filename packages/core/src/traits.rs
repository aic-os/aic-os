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

    /// EIP-1559 base fee per gas (latest block). EVM only.
    async fn get_base_fee(&self) -> Result<u128> {
        Err(CoreError::NotImplemented("base fee (non-EVM)"))
    }
    /// EIP-1559 recommended priority fee (tip). EVM only.
    async fn get_max_priority_fee(&self) -> Result<u128> {
        Err(CoreError::NotImplemented("priority fee (non-EVM)"))
    }
}
