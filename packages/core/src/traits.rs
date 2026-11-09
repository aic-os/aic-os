use async_trait::async_trait;

use crate::chain::Chain;
use crate::error::{CoreError, Result};

/// Unified async interface every provider must implement.
#[async_trait]
pub trait ChainTraits: Send + Sync {
    fn chain(&self) -> Chain;

    async fn get_balance(&self, address: &str) -> Result<u128>;
    async fn estimate_fee(&self, from: &str, to: &str, amount: u128) -> Result<u128>;
    async fn get_nonce(&self, address: &str) -> Result<u64>;
    async fn get_gas_price(&self) -> Result<u128>;
    async fn broadcast(&self, raw_tx_hex: &str) -> Result<String>;
    async fn health(&self) -> Result<bool>;

    /// ERC-20 token balance. Default: not supported by this chain.
    /// Returns amount in the token's smallest unit (e.g. 1 USDC = 1_000_000).
    async fn get_erc20_balance(&self, _contract: &str, _holder: &str) -> Result<u128> {
        Err(CoreError::NotImplemented("ERC-20 balance (non-EVM chain)"))
    }
}
