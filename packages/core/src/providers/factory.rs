use std::collections::HashMap;
use std::sync::Arc;

use crate::chain::{Chain, ChainType};
use crate::error::{CoreError, Result};
use crate::providers::evm::EvmProvider;
use crate::traits::ChainTraits;

/// Per-chain RPC settings (later -- loaded from Settings.yaml).
#[derive(Debug, Clone)]
pub struct ChainSettings {
    pub rpc_url: String,
    pub chain_id: Option<u64>,
}

/// Central factory: given a chain and its settings, returns a working provider.
#[derive(Debug, Clone, Default)]
pub struct ProviderFactory {
    settings: HashMap<Chain, ChainSettings>,
}

impl ProviderFactory {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_settings(settings: HashMap<Chain, ChainSettings>) -> Self {
        Self { settings }
    }

    pub fn register(&mut self, chain: Chain, settings: ChainSettings) {
        self.settings.insert(chain, settings);
    }

    pub fn create(&self, chain: Chain) -> Result<Arc<dyn ChainTraits>> {
        let settings = self
            .settings
            .get(&chain)
            .ok_or_else(|| CoreError::ChainNotSupported(chain.as_str().to_string()))?
            .clone();

        match chain.chain_type() {
            ChainType::Evm => Ok(Arc::new(EvmProvider::new(chain, settings))),
            ChainType::Utxo => Err(CoreError::NotImplemented("UTXO provider (Phase 1.2)")),
            ChainType::Cosmos => Err(CoreError::NotImplemented("Cosmos provider (Phase 1.3)")),
            ChainType::Other => Err(CoreError::NotImplemented("Other-chain provider (Phase 1.4)")),
        }
    }
}
