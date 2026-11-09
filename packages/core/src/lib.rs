//! AIC-OS core library.

pub mod chain;
pub mod error;
pub mod providers;
pub mod settings;
pub mod traits;
pub mod wallet;

pub use chain::{Chain, ChainType};
pub use error::{CoreError, Result};
pub use providers::ProviderFactory;
pub use settings::Settings;
pub use traits::ChainTraits;
pub use wallet::{
    Eip1559Transaction, EthAccount, LegacyTransaction, Mnemonic, SignedEip1559,
    SignedTransaction, WordCount,
};

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
