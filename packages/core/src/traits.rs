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

    /// Broadcast a signed transaction. Returns tx hash.
    async fn broadcast(&self, raw_tx_hex: &str) -> Result<String>;

    /// Node reachable and in sync.
    async fn health(&self) -> Result<bool>;
}
