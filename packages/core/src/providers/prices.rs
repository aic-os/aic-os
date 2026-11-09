//! CoinGecko price feed: convert token amounts to USD / RUB.

use std::collections::HashMap;
use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::chain::Chain;
use crate::error::{CoreError, Result};

const COINGECKO_BASE: &str = "https://api.coingecko.com/api/v3";

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PriceInfo { pub usd: f64, pub rub: f64 }

pub struct PriceClient {
    http: reqwest::Client,
    base_url: String,
}

impl PriceClient {
    pub fn new() -> Self {
        let http = reqwest::Client::builder()
            .user_agent("aic-core/0.1")
            .timeout(Duration::from_secs(8))
            .connect_timeout(Duration::from_secs(5))
            .build()
            .expect("http client");
        Self { http, base_url: COINGECKO_BASE.to_string() }
    }
    pub fn with_base_url(base_url: impl Into<String>) -> Self {
        let mut c = Self::new(); c.base_url = base_url.into(); c
    }

    /// CoinGecko id per chain. Returns None for chains we don't price (rare).
    pub fn coingecko_id(chain: Chain) -> Option<&'static str> {
        use Chain::*;
        Some(match chain {
            Ethereum => "ethereum",
            Bsc => "binancecoin",
            Polygon => "matic-network",
            Arbitrum | Optimism | Base | Linea | Scroll | ZkSync | Mantle | Blast
                | PolygonZkEvm | Aurora | ArbitrumNova | Zora | Mode | Mint
                | MantaPacific | Kroma | Pgn | Redstone | Ancient8 | Cyber
                | Lisk | Curtis | Frame | Rari | Opbnb => "ethereum",
            Avalanche => "avalanche-2",
            Gnosis => "xdai",
            Celo => "celo",
            Fantom => "fantom",
            Cronos => "crypto-com-chain",
            Moonbeam => "moonbeam",
            Moonriver => "moonriver",
            Filecoin => "filecoin",
            Harmony => "harmony",
            Metis => "metis-token",
            Klaytn => "klay-token",
            Astar => "astar",
            Boba => "boba-network",
            Conflux => "conflux-token",
            Evmos => "evmos",
            Canto => "canto",
            Okc => "oec-token",
            Fuse => "fuse-network-token",
            Telos => "telos",
            Dogechain => "dogechain",
            Songbird => "songbird",
            Gochain => "gochain",
            Callisto => "callisto",
            Ubic => "ubiq",
            Xinfin => "xdce-crowd-sale",
            Wanchain => "wanchain",
            ThunderCore => "thunder-token",
            Hoo => "hoo-token",
            Kcc => "kucoin-shares",
            Bitgert => "bitrise-token",
            Dexalot => "dexalot",
            ApeChain => "apecoin",
            // UTXO
            Bitcoin => "bitcoin",
            Litecoin => "litecoin",
            Dogecoin => "dogecoin",
            BitcoinCash => "bitcoin-cash",
            Dash => "dash",
            Zcash => "zcash",
            BitcoinSv => "bitcoin-cash-sv",
            DigiByte => "digibyte",
            Ravencoin => "ravencoin",
            Groestlcoin => "groestlcoin",
            // Cosmos
            CosmosHub => "cosmos",
            Osmosis => "osmosis",
            Celestia => "celestia",
            Injective => "injective-protocol",
            Kava => "kava",
            Sei => "sei-network",
            Akash => "akash-network",
            Juno => "juno-network",
            Stargaze => "stargaze",
            Kujira => "kujira",
            Stride => "stride",
            Neutron => "neutron-3",
            Archway => "archway",
            Persistence => "persistence",
            Umee => "umee",
            Comdex => "comdex",
            Crescent => "crescent-network",
            Regen => "regen",
            Sentinel => "sentinel",
            Sifchain => "sifchain",
            Chihuahua => "chihuahua-token",
            Bitsong => "bitsong",
            Desmos => "desmos",
            Emoney => "e-money",
            Medibloc => "medibloc",
            // Other
            Solana => "solana",
            Tron => "tron",
            Ton => "the-open-network",
            Sui => "sui",
            Aptos => "aptos",
            Ripple => "ripple",
            Near => "near",
            Algorand => "algorand",
            Stellar => "stellar",
            Kaspa => "kaspa",
            Monero => "monero",
            Tezos => "tezos",
            MultiversX => "elrond-erd-2",
            Hedera => "hedera-hashgraph",
            VeChain => "vechain",
            Theta => "theta-token",
            Cardano => "cardano",
            Polkadot => "polkadot",
            Meter => "meter",
            KavaEvm => "kava",
            OasisSapphire => "oasis-network",
            Elastos => "elastos",
            Syscoin => "syscoin",
            Lightlink => "lightlink",
            XLayer => "okb",
            Berachain => "berachain-bera",
            Band => "band-protocol",
            Fetch => "fetch-ai",
            Axelar => "axelar",
            Dydx => "dydx-chain",
            Saga => "saga-2",
            Nomic => "nomic",
            Terra => "terra-luna-2",
            AssetMantle => "assetmantle",
            _ => return None,
        })
    }

    pub async fn get_prices(&self, chains: &[Chain]) -> Result<HashMap<Chain, PriceInfo>> {
        if chains.is_empty() { return Ok(HashMap::new()); }
        let mut id_to_chains: HashMap<&'static str, Vec<Chain>> = HashMap::new();
        for &chain in chains {
            if let Some(id) = Self::coingecko_id(chain) {
                id_to_chains.entry(id).or_default().push(chain);
            }
        }
        if id_to_chains.is_empty() { return Ok(HashMap::new()); }
        let ids: Vec<&str> = id_to_chains.keys().copied().collect();
        let url = format!("{}/simple/price?ids={}&vs_currencies=usd,rub",
            self.base_url, ids.join(","));
        let resp = self.http.get(&url).send().await
            .map_err(|e| CoreError::Rpc(format!("coingecko http: {e}")))?;
        if !resp.status().is_success() {
            return Err(CoreError::Rpc(format!("coingecko status: {}", resp.status())));
        }
        let raw: HashMap<String, HashMap<String, f64>> = resp.json().await
            .map_err(|e| CoreError::Rpc(format!("coingecko parse: {e}")))?;
        let mut out = HashMap::new();
        for (cg_id, price_map) in raw {
            let usd = price_map.get("usd").copied().unwrap_or(0.0);
            let rub = price_map.get("rub").copied().unwrap_or(0.0);
            if let Some(cs) = id_to_chains.get(cg_id.as_str()) {
                for &c in cs { out.insert(c, PriceInfo { usd, rub }); }
            }
        }
        Ok(out)
    }
}

impl Default for PriceClient { fn default() -> Self { Self::new() } }

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn coingecko_id_known() {
        assert_eq!(PriceClient::coingecko_id(Chain::Ethereum), Some("ethereum"));
        assert_eq!(PriceClient::coingecko_id(Chain::Bitcoin), Some("bitcoin"));
        assert_eq!(PriceClient::coingecko_id(Chain::Solana), Some("solana"));
    }
    #[tokio::test] async fn prices_empty() {
        assert!(PriceClient::new().get_prices(&[]).await.unwrap().is_empty());
    }
}


