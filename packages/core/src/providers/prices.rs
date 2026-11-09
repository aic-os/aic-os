//! CoinGecko price feed: convert token amounts to USD / RUB.
//!
//! Uses the public CoinGecko API (rate limit 10-30 req/min without a key,
//! which is fine for wallet UI: prices are cached client-side for 60s).

use std::collections::HashMap;
use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::chain::Chain;
use crate::error::{CoreError, Result};

const COINGECKO_BASE: &str = "https://api.coingecko.com/api/v3";

/// Fiat price for a single asset.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PriceInfo {
    pub usd: f64,
    pub rub: f64,
}

/// CoinGecko price client.
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
        let mut c = Self::new();
        c.base_url = base_url.into();
        c
    }

    /// CoinGecko id for a chain's native coin.
    pub fn coingecko_id(chain: Chain) -> Option<&'static str> {
        use Chain::*;
        Some(match chain {
            Ethereum => "ethereum",
            Bsc => "binancecoin",
            Polygon => "matic-network",
            Arbitrum => "ethereum",        // native = ETH
            Optimism => "ethereum",        // native = ETH
            Base => "ethereum",            // native = ETH
            Avalanche => "avalanche-2",
            Bitcoin => "bitcoin",
            Litecoin => "litecoin",
            Dogecoin => "dogecoin",
            CosmosHub => "cosmos",
            Osmosis => "osmosis",
            Celestia => "celestia",
            Solana => "solana",
            Tron => "tron",
            Ton => "the-open-network",
            Sui => "sui",
            Aptos => "aptos",
            Ripple => "ripple",
        })
    }

    /// Fetch USD + RUB prices for the given chains in one HTTP call.
    /// Returns a map chain -> PriceInfo. Missing chains are simply absent.
    pub async fn get_prices(&self, chains: &[Chain]) -> Result<HashMap<Chain, PriceInfo>> {
        if chains.is_empty() {
            return Ok(HashMap::new());
        }

        // Build unique coingecko ids.
        let mut id_to_chains: HashMap<&'static str, Vec<Chain>> = HashMap::new();
        for &chain in chains {
            if let Some(id) = Self::coingecko_id(chain) {
                id_to_chains.entry(id).or_default().push(chain);
            }
        }

        if id_to_chains.is_empty() {
            return Ok(HashMap::new());
        }

        let ids: Vec<&str> = id_to_chains.keys().copied().collect();
        let ids_param = ids.join(",");

        let url = format!(
            "{}/simple/price?ids={}&vs_currencies=usd,rub",
            self.base_url, ids_param
        );

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
            if let Some(chains_for_id) = id_to_chains.get(cg_id.as_str()) {
                for &chain in chains_for_id {
                    out.insert(chain, PriceInfo { usd, rub });
                }
            }
        }
        Ok(out)
    }
}

impl Default for PriceClient {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coingecko_id_known() {
        assert_eq!(PriceClient::coingecko_id(Chain::Ethereum), Some("ethereum"));
        assert_eq!(PriceClient::coingecko_id(Chain::Bitcoin), Some("bitcoin"));
        assert_eq!(PriceClient::coingecko_id(Chain::Solana), Some("solana"));
        assert_eq!(PriceClient::coingecko_id(Chain::Ton), Some("the-open-network"));
    }

    #[test]
    fn coingecko_id_eth_wrapped_chains() {
        // Optimism, Arbitrum, Base all use ETH as native coin.
        for c in [Chain::Optimism, Chain::Arbitrum, Chain::Base] {
            assert_eq!(PriceClient::coingecko_id(c), Some("ethereum"));
        }
    }

    #[tokio::test]
    async fn prices_empty_chains_returns_empty() {
        let client = PriceClient::new();
        let r = client.get_prices(&[]).await.unwrap();
        assert!(r.is_empty());
    }
}
