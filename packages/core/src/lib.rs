//! AIC-OS core library (scaffold).

pub mod chain;
pub mod error;
pub mod providers;
pub mod traits;

pub use chain::{Chain, ChainType};
pub use error::{CoreError, Result};
pub use providers::ProviderFactory;
pub use traits::ChainTraits;

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
