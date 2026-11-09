use aic_core::chain::{Chain, ChainType};
use aic_core::{ChainTraits, PriceClient, ProviderFactory, Settings};

fn settings_path() -> std::path::PathBuf {
    let mut p = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.pop(); p.pop(); p.push("config"); p.push("settings.yaml"); p
}

#[tokio::main]
async fn main() {
    println!("=== AIC-OS networks registry ===");
    let all = Chain::ALL;
    let mut by_type = [0usize; 4];
    for &c in all {
        match c.chain_type() {
            ChainType::Evm => by_type[0] += 1,
            ChainType::Utxo => by_type[1] += 1,
            ChainType::Cosmos => by_type[2] += 1,
            ChainType::Other => by_type[3] += 1,
        }
    }
    println!("Total chains: {}", all.len());
    println!("  EVM: {}, UTXO: {}, Cosmos: {}, Other: {}",
        by_type[0], by_type[1], by_type[2], by_type[3]);

    let settings = Settings::from_yaml(settings_path()).expect("settings");
    let factory = ProviderFactory::from_settings(&settings);
    let prices = PriceClient::new();

    // Live test: sample 12 chains across all types
    let targets: &[(Chain, &str, u32)] = &[
        (Chain::Ethereum, "0xd8dA6BF26964aF9D7EdE9e03E53415D37aA96045", 18),
        (Chain::Polygon, "0xd8dA6BF26964aF9D7EdE9e03E53415D37aA96045", 18),
        (Chain::Arbitrum, "0xd8dA6BF26964aF9D7EdE9e03E53415D37aA96045", 18),
        (Chain::Optimism, "0xd8dA6BF26964aF9D7EdE9e03E53415D37aA96045", 18),
        (Chain::Base, "0xd8dA6BF26964aF9D7EdE9e03E53415D37aA96045", 18),
        (Chain::Bitcoin, "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa", 8),
        (Chain::Litecoin, "ltc1qg3zyz6h5znu4m6jnmfe9w4rz9f3k74pjw7kjy0", 8),
        (Chain::CosmosHub, "cosmos1jv65s3grqf6v6jl3dp4t6c9t9rk99cd88e2vme", 6),
        (Chain::Osmosis, "osmo1jv65s3grqf6v6jl3dp4t6c9t9rk99cd88e2vme", 6),
        (Chain::Solana, "7cVfgArCheMR6Cs4t6vz5rfnqd56vZq4ndaBrY5xkxXy", 9),
        (Chain::Tron, "TLa2f6VPqDgRE67v1736s7bJ8Ray5wYjU7", 6),
        (Chain::Ripple, "rN7n7otQDd6FczFgLdSqtcsAUxDkw6fzRH", 6),
    ];

    println!("\n=== live balances (sample of 12) ===");
    println!("{:<14} {:<14} {:<22} {:<14} {:<14} {:<6}",
        "chain", "type", "balance", "usd", "rub", "health");
    println!("{}", "-".repeat(86));

    let chains: Vec<Chain> = targets.iter().map(|(c, _, _)| *c).collect();
    let price_map = prices.get_prices(&chains).await.unwrap_or_default();

    for (chain, addr, decimals) in targets {
        let type_str = match chain.chain_type() {
            ChainType::Evm => "EVM",
            ChainType::Utxo => "UTXO",
            ChainType::Cosmos => "Cosmos",
            ChainType::Other => "Other",
        };
        match factory.create(*chain) {
            Ok(p) => {
                let bal = p.get_balance(addr).await.unwrap_or(0);
                let human = bal as f64 / 10f64.powi(*decimals as i32);
                let price = price_map.get(chain);
                let usd = price.map(|p| human * p.usd).unwrap_or(0.0);
                let rub = price.map(|p| human * p.rub).unwrap_or(0.0);
                let health = p.health().await.unwrap_or(false);
                println!("{:<14} {:<14} {:<22} ${:<13.4} ₽{:<13.2} {}",
                    chain.as_str(), type_str,
                    format!("{human:.8}"), usd, rub, health);
            }
            Err(e) => println!("{:<14} error: {e}", chain.as_str()),
        }
    }
}
