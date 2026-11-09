use async_trait::async_trait;
use std::time::Duration;

use crate::chain::Chain;
use crate::error::{CoreError, Result};
use crate::providers::factory::ChainSettings;
use crate::traits::ChainTraits;

pub struct CosmosProvider {
    chain: Chain,
    api_urls: Vec<String>,
    client: reqwest::Client,
}

impl CosmosProvider {
    pub fn new(chain: Chain, settings: ChainSettings) -> Self {
        let client = reqwest::Client::builder()
            .user_agent("aic-core/0.1")
            .timeout(Duration::from_secs(5))
            .build()
            .expect("http client");
        Self { chain, api_urls: settings.rpc_urls, client }
    }

    fn denom_for(chain: Chain) -> &'static str {
        match chain {
            Chain::CosmosHub => "uatom",
            Chain::Osmosis => "uosmo",
            Chain::Celestia => "utia",
            Chain::Injective => "inj",
            Chain::Kava => "ukava",
            Chain::Sei => "usei",
            Chain::Akash => "uakt",
            Chain::Juno => "ujuno",
            Chain::Stargaze => "ustars",
            Chain::Kujira => "ukuji",
            Chain::Stride => "ustrd",
            Chain::Neutron => "untrn",
            Chain::Archway => "aarchway",
            Chain::Persistence => "uxprt",
            Chain::Umee => "uumee",
            Chain::Comdex => "ucmdx",
            Chain::Crescent => "ucre",
            Chain::Regen => "uregen",
            Chain::Sentinel => "udvpn",
            Chain::Sifchain => "rowan",
            Chain::Chihuahua => "uhuahua",
            Chain::Bitsong => "ubtsg",
            Chain::Desmos => "udsm",
            Chain::Emoney => "ungm",
            Chain::Medibloc => "umed",
            Chain::Band => "uband",
            Chain::Fetch => "afet",
            Chain::Axelar => "uaxl",
            Chain::Dydx => "adydx",
            Chain::Saga => "usaga",
            Chain::Nomic => "unom",
            Chain::Terra => "uluna",
            Chain::AssetMantle => "umntl",
            Chain::Cheqd => "ncheq",
            Chain::Lava => "ulava",
            Chain::Nois => "unois",
            _ => "uatom",
        }
    }
}

#[async_trait]
impl ChainTraits for CosmosProvider {
    fn chain(&self) -> Chain { self.chain }

    async fn get_balance(&self, address: &str) -> Result<u128> {
        let addr = address.trim();
        if addr.is_empty() {
            return Err(CoreError::InvalidAddress("empty Cosmos address".into()));
        }
        let denom = Self::denom_for(self.chain);

        let mut errors = Vec::new();
        for base in &self.api_urls {
            let url = format!("{base}/cosmos/bank/v1beta1/balances/{addr}");
            match self.client.get(&url).send().await {
                Ok(resp) => {
                    if !resp.status().is_success() {
                        errors.push(format!("{url}: HTTP {}", resp.status()));
                        continue;
                    }
                    let json: serde_json::Value = resp.json().await?;
                    let balances = json.get("balances").and_then(|v| v.as_array());
                    match balances {
                        Some(arr) => {
                            for coin in arr {
                                if coin.get("denom").and_then(|d| d.as_str()) == Some(denom) {
                                    if let Some(amount) = coin.get("amount").and_then(|a| a.as_str()) {
                                        let n: u128 = amount.parse()
                                            .map_err(|e| CoreError::Rpc(format!("cosmos parse: {e}")))?;
                                        return Ok(n);
                                    }
                                }
                            }
                            return Ok(0);
                        }
                        None => { errors.push(format!("{url}: no balances")); continue; }
                    }
                }
                Err(e) => errors.push(format!("{url}: {e}")),
            }
        }
        Err(CoreError::Rpc(format!("all Cosmos endpoints failed: {}", errors.join(" | "))))
    }

    async fn estimate_fee(&self, _f: &str, _t: &str, _a: u128) -> Result<u128> { Err(CoreError::NotImplemented("Cosmos estimate_fee")) }
    async fn get_nonce(&self, _a: &str) -> Result<u64> { Err(CoreError::NotImplemented("Cosmos get_nonce")) }
    async fn get_gas_price(&self) -> Result<u128> { Err(CoreError::NotImplemented("Cosmos get_gas_price")) }
    async fn broadcast(&self, _r: &str) -> Result<String> { Err(CoreError::NotImplemented("Cosmos broadcast")) }

    async fn health(&self) -> Result<bool> {
        for base in &self.api_urls {
            let url = format!("{base}/cosmos/base/tendermint/v1beta1/node_info");
            if let Ok(resp) = self.client.get(&url).send().await {
                if resp.status().is_success() { return Ok(true); }
            }
        }
        Ok(false)
    }
}


