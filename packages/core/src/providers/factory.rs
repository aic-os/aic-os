use std::collections::HashMap;
use std::sync::Arc;

use crate::chain::{Chain, ChainType};
use crate::error::{CoreError, Result};
use crate::providers::algorand::AlgorandProvider;
use crate::providers::aptos::AptosProvider;
use crate::providers::bitcoin::BitcoinProvider;
use crate::providers::cardano::CardanoProvider;
use crate::providers::cosmos::CosmosProvider;
use crate::providers::evm::EvmProvider;
use crate::providers::hedera::HederaProvider;
use crate::providers::kaspa::KaspaProvider;
use crate::providers::monero::MoneroProvider;
use crate::providers::multiversx::MultiversXProvider;
use crate::providers::near::NearProvider;
use crate::providers::polkadot::PolkadotProvider;
use crate::providers::solana::SolanaProvider;
use crate::providers::stellar::StellarProvider;
use crate::providers::sui::SuiProvider;
use crate::providers::tezos::TezosProvider;
use crate::providers::theta::ThetaProvider;
use crate::providers::ton::TonProvider;
use crate::providers::tron::TronProvider;
use crate::providers::vechain::VeChainProvider;
use crate::providers::xrp::XrpProvider;
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
    pub fn with_settings(settings: HashMap<Chain, ChainSettings>) -> Self { Self { settings } }

    pub fn from_settings(settings: &Settings) -> Self {
        let mut map = HashMap::new();
        for &chain in Chain::ALL {
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
        let s = self.settings
            .get(&chain)
            .ok_or_else(|| CoreError::ChainNotSupported(chain.as_str().to_string()))?
            .clone();

        match chain.chain_type() {
            ChainType::Evm => Ok(Arc::new(EvmProvider::new(chain, s))),
            ChainType::Utxo => Ok(Arc::new(BitcoinProvider::new(chain, s))),
            ChainType::Cosmos => Ok(Arc::new(CosmosProvider::new(chain, s))),
            ChainType::Other => match chain {
                Chain::Solana => Ok(Arc::new(SolanaProvider::new(chain, s))),
                Chain::Ton => Ok(Arc::new(TonProvider::new(chain, s))),
                Chain::Tron => Ok(Arc::new(TronProvider::new(chain, s))),
                Chain::Sui => Ok(Arc::new(SuiProvider::new(chain, s))),
                Chain::Ripple => Ok(Arc::new(XrpProvider::new(chain, s))),
                Chain::Aptos => Ok(Arc::new(AptosProvider::new(chain, s))),
                Chain::Near => Ok(Arc::new(NearProvider::new(chain, s))),
                Chain::Algorand => Ok(Arc::new(AlgorandProvider::new(chain, s))),
                Chain::Stellar => Ok(Arc::new(StellarProvider::new(chain, s))),
                Chain::Kaspa => Ok(Arc::new(KaspaProvider::new(chain, s))),
                Chain::Monero => Ok(Arc::new(MoneroProvider::new(chain, s))),
                Chain::Tezos => Ok(Arc::new(TezosProvider::new(chain, s))),
                Chain::MultiversX => Ok(Arc::new(MultiversXProvider::new(chain, s))),
                Chain::Hedera => Ok(Arc::new(HederaProvider::new(chain, s))),
                Chain::VeChain => Ok(Arc::new(VeChainProvider::new(chain, s))),
                Chain::Theta => Ok(Arc::new(ThetaProvider::new(chain, s))),
                Chain::Cardano => Ok(Arc::new(CardanoProvider::new(chain, s))),
                Chain::Polkadot => Ok(Arc::new(PolkadotProvider::new(chain, s))),
                _ => Err(CoreError::NotImplemented("Other-chain provider")),
            },
        }
    }
}
