use aic_core::chain::Chain;
use aic_core::{PriceClient, ProviderFactory, Settings};

fn settings_path() -> std::path::PathBuf {
    let mut p = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.pop(); p.pop(); p.push("config"); p.push("settings.yaml"); p
}

#[tokio::main]
async fn main() {
    let settings = Settings::from_yaml(settings_path()).expect("settings");
    let factory = ProviderFactory::from_settings(&settings);
    let prices = PriceClient::new();

    let test: &[(Chain, &str, u32)] = &[
        (Chain::Ethereum, "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045", 18),
        (Chain::Polygon, "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045", 18),
        (Chain::Bsc, "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045", 18),
        (Chain::Bitcoin, "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa", 8),
        (Chain::Solana, "7cVfgArCheMR6Cs4t6vz5rfnqd56vZq4ndaBrY5xkxXy", 9),
        (Chain::Ton, "EQDk2VTvn04SUKJrW7rXahzdF8_Qi6utb0wj43InCu9vdjrR", 9),
        (Chain::Tron, "TLa2f6VPqDgRE67v1736s7bJ8Ray5wYjU7", 6),
        (Chain::Ripple, "rN7n7otQDd6FczFgLdSqtcsAUxDkw6fzRH", 6),
        (Chain::CosmosHub, "cosmos1jv65s3grqf6v6jl3dp4t6c9t9rk99cd88e2vme", 6),
        (Chain::Sui, "0x0000000000000000000000000000000000000000000000000000000000000005", 9),
    ];

    // Fetch prices in one call.
    let chains: Vec<Chain> = test.iter().map(|(c, _, _)| *c).collect();
    let price_map = prices.get_prices(&chains).await.unwrap_or_default();

    println!("{:<12} {:<20} {:<20} {:<12} {:<12}", "chain", "balance", "usd", "rub", "health");
    println!("{}", "-".repeat(80));

    for (chain, addr, decimals) in test {
        match factory.create(*chain) {
            Ok(p) => {
                let bal = p.get_balance(addr).await.unwrap_or(0);
                let human = bal as f64 / 10f64.powi(*decimals as i32);
                let price = price_map.get(chain);
                let usd = price.map(|p| human * p.usd).unwrap_or(0.0);
                let rub = price.map(|p| human * p.rub).unwrap_or(0.0);
                let health = p.health().await.unwrap_or(false);
                println!(
                    "{:<12} {:<20} ${:<11.2} ₽{:<11.2} {}",
                    chain.as_str(), format!("{human:.6}"), usd, rub, health
                );
            }
            Err(e) => println!("{:<12} error: {e}", chain.as_str()),
        }
    }
}

