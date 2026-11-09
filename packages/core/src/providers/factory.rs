use std::collections::HashMap;
use std::sync::Arc;

use crate::chain::{Chain, ChainType};
use crate::error::{CoreError, Result};
use crate::providers::bitcoin::BitcoinProvider;
use crate::providers::evm::EvmProvider;
use crate::providers::solana::SolanaProvider;
use crate::providers::ton::TonProvider;
use crate::settings::Settings;
use crate::traits::ChainTraits;

#[derive(Debug, Clone)]
pub struct ChainSettings {
    pub rpc_urls: Vec<String>,
    pub chain_id: Option<u64>,
}

#[derive(Debug, Clone, Default)]
pub struct ProviderFactory {
    settings: HashMap<Chain, ChainSettings>,
}

impl ProviderFactory {
    pub fn new() -> Self { Self::default() }

    pub fn with_settings(settings: HashMap<Chain, ChainSettings>) -> Self {
        Self { settings }
    }

    pub fn from_settings(settings: &Settings) -> Self {
        let mut map = HashMap::new();
        let all = [
            Chain::Ethereum, Chain::Bsc, Chain::Polygon, Chain::Arbitrum,
            Chain::Optimism, Chain::Base, Chain::Avalanche,
            Chain::Bitcoin, Chain::Litecoin, Chain::Dogecoin,
            Chain::Solana, Chain::Ton,
        ];
        for chain in all {
            if let Some(rpc_urls) = settings.rpc_urls_for(chain) {
                if !rpc_urls.is_empty() {
                    map.insert(chain, ChainSettings {
                        rpc_urls,
                        chain_id: settings.chain_id_for(chain),
                    });
                }
            }
        }
        Self { settings: map }
    }

    pub fn register(&mut self, chain: Chain, settings: ChainSettings) {
        self.settings.insert(chain, settings);
    }

    pub fn create(&self, chain: Chain) -> Result<Arc<dyn ChainTraits>> {
        let settings = self.settings
            .get(&chain)
            .ok_or_else(|| CoreError::ChainNotSupported(chain.as_str().to_string()))?
            .clone();

        match chain.chain_type() {
            ChainType::Evm => Ok(Arc::new(EvmProvider::new(chain, settings))),
            ChainType::Utxo => Ok(Arc::new(BitcoinProvider::new(chain, settings))),
            ChainType::Other => match chain {
                Chain::Solana => Ok(Arc::new(SolanaProvider::new(chain, settings))),
                Chain::Ton => Ok(Arc::new(TonProvider::new(chain, settings))),
                _ => Err(CoreError::NotImplemented("Other-chain provider")),
            },
            ChainType::Cosmos => Err(CoreError::NotImplemented("Cosmos provider (Phase 1.3)")),
        }
    }
}
