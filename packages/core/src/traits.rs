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

    async fn get_erc20_balance(&self, _contract: &str, _holder: &str) -> Result<u128> {
        Err(CoreError::NotImplemented("ERC-20 balance (non-EVM chain)"))
    }
    async fn get_erc20_decimals(&self, _contract: &str) -> Result<u8> {
        Err(CoreError::NotImplemented("ERC-20 decimals (non-EVM chain)"))
    }
    async fn get_erc20_symbol(&self, _contract: &str) -> Result<String> {
        Err(CoreError::NotImplemented("ERC-20 symbol (non-EVM chain)"))
    }
    async fn get_erc20_name(&self, _contract: &str) -> Result<String> {
        Err(CoreError::NotImplemented("ERC-20 name (non-EVM chain)"))
    }
}
