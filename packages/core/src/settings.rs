use std::collections::HashMap;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::chain::Chain;
use crate::error::{CoreError, Result};

/// Per-chain RPC list. Order matters: first = primary, rest = fallback.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainConfig {
    #[serde(default)]
    pub chain_id: Option<u64>,
    pub rpc: Vec<String>,
}

/// Top-level settings, loaded from config/settings.yaml.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    #[serde(default)]
    pub chains: HashMap<String, ChainConfig>,
}

impl Settings {
    pub fn from_yaml(path: impl AsRef<Path>) -> Result<Self> {
        let content = std::fs::read_to_string(path.as_ref())?;
        let settings: Settings = serde_yaml_ng::from_str(&content)
            .map_err(|e| CoreError::Settings(e.to_string()))?;
        Ok(settings)
    }

    pub fn rpc_urls_for(&self, chain: Chain) -> Option<Vec<String>> {
        self.chains.get(chain.as_str()).map(|c| c.rpc.clone())
    }

    pub fn chain_id_for(&self, chain: Chain) -> Option<u64> {
        self.chains.get(chain.as_str()).and_then(|c| c.chain_id)
    }
}
