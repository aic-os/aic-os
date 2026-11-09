use serde::{Deserialize, Serialize};

/// Broad class of a blockchain, used to pick a provider implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChainType {
    Evm,
    Utxo,
    Cosmos,
    Other,
}

/// Central registry of every blockchain supported by AIC-OS.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Chain {
    // EVM
    Ethereum,
    Bsc,
    Polygon,
    Arbitrum,
    Optimism,
    Base,
    Avalanche,
    // UTXO
    Bitcoin,
    Litecoin,
    Dogecoin,
    // Cosmos
    CosmosHub,
    Osmosis,
    Celestia,
    // Other
    Solana,
    Tron,
    Ton,
    Sui,
    Aptos,
    Ripple,
}

impl Chain {
    pub fn chain_type(self) -> ChainType {
        use Chain::*;
        match self {
            Ethereum | Bsc | Polygon | Arbitrum | Optimism | Base | Avalanche => ChainType::Evm,
            Bitcoin | Litecoin | Dogecoin => ChainType::Utxo,
            CosmosHub | Osmosis | Celestia => ChainType::Cosmos,
            Solana | Tron | Ton | Sui | Aptos | Ripple => ChainType::Other,
        }
    }

    pub fn as_str(self) -> &'static str {
        use Chain::*;
        match self {
            Ethereum => "ethereum",
            Bsc => "bsc",
            Polygon => "polygon",
            Arbitrum => "arbitrum",
            Optimism => "optimism",
            Base => "base",
            Avalanche => "avalanche",
            Bitcoin => "bitcoin",
            Litecoin => "litecoin",
            Dogecoin => "dogecoin",
            CosmosHub => "cosmos",
            Osmosis => "osmosis",
            Celestia => "celestia",
            Solana => "solana",
            Tron => "tron",
            Ton => "ton",
            Sui => "sui",
            Aptos => "aptos",
            Ripple => "ripple",
        }
    }

    pub fn from_name(s: &str) -> Option<Self> {
        use Chain::*;
        Some(match s.trim().to_lowercase().as_str() {
            "ethereum" | "eth" => Ethereum,
            "bsc" | "bnb" | "binance" => Bsc,
            "polygon" | "matic" => Polygon,
            "arbitrum" | "arb" => Arbitrum,
            "optimism" | "op" => Optimism,
            "base" => Base,
            "avalanche" | "avax" => Avalanche,
            "bitcoin" | "btc" => Bitcoin,
            "litecoin" | "ltc" => Litecoin,
            "dogecoin" | "doge" => Dogecoin,
            "cosmos" | "atom" => CosmosHub,
            "osmosis" | "osmo" => Osmosis,
            "celestia" | "tia" => Celestia,
            "solana" | "sol" => Solana,
            "tron" | "trx" => Tron,
            "ton" => Ton,
            "sui" => Sui,
            "aptos" | "apt" => Aptos,
            "ripple" | "xrp" => Ripple,
            _ => return None,
        })
    }
}
