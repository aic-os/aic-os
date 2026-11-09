use aic_core::chain::Chain;
use aic_core::{ProviderFactory, Settings};

fn settings_path() -> std::path::PathBuf {
    let mut p = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.pop(); // -> packages
    p.pop(); // -> workspace root
    p.push("config");
    p.push("settings.yaml");
    p
}

#[tokio::main]
async fn main() {
    let settings = Settings::from_yaml(settings_path()).expect("settings");
    let factory = ProviderFactory::from_settings(&settings);

    let btc_addr = "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa"; // Satoshi's genesis address
    let sol_addr = "7cVfgArCheMR6Cs4t6vz5rfnqd56vZq4ndaBrY5xkxXy";
    let ton_addr = "EQDk2VTvn04SUKJrW7rXahzdF8_Qi6utb0wj43InCu9vdjrR";

    for (chain, addr) in [
        (Chain::Bitcoin, btc_addr),
        (Chain::Solana, sol_addr),
        (Chain::Ton, ton_addr),
    ] {
        println!("\n--- {} ---", chain.as_str());
        println!("address: {addr}");
        match factory.create(chain) {
            Ok(provider) => {
                match provider.get_balance(addr).await {
                    Ok(raw) => {
                        let decimals = match chain {
                            Chain::Bitcoin => 8u32,
                            Chain::Solana => 9,
                            Chain::Ton => 9,
                            _ => 0,
                        };
                        let human = raw as f64 / 10f64.powi(decimals as i32);
                        println!("balance: {raw} (raw) = {human:.9}");
                    }
                    Err(e) => println!("balance error: {e}"),
                }
                match provider.health().await {
                    Ok(ok) => println!("health: {ok}"),
                    Err(e) => println!("health error: {e}"),
                }
            }
            Err(e) => println!("create error: {e}"),
        }
    }
}
