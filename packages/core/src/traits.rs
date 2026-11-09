use async_trait::async_trait;

use crate::chain::Chain;
use crate::error::Result;

/// Unified async interface every provider must implement.
#[async_trait]
pub trait ChainTraits: Send + Sync {
    fn chain(&self) -> Chain;

    /// Native coin balance of the given address, in the smallest unit.
    async fn get_balance(&self, address: &str) -> Result<u128>;

    /// Rough network fee estimate for a simple transfer.
    async fn estimate_fee(&self, from: &str, to: &str, amount: u128) -> Result<u128>;

    /// Transaction sequence number (nonce) for the given address.
    /// For EVM -- pending nonce; for account-based chains -- current sequence.
    async fn get_nonce(&self, address: &str) -> Result<u64>;

    /// Current network gas price in smallest unit (wei for EVM).
    async fn get_gas_price(&self) -> Result<u128>;

    /// Broadcast a signed transaction. Returns tx hash.
    async fn broadcast(&self, raw_tx_hex: &str) -> Result<String>;

    /// Node reachable and in sync.
    async fn health(&self) -> Result<bool>;
}
