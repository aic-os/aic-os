use aic_core::chain::Chain;
use aic_core::{ProviderFactory, Settings};

// USDC on Ethereum mainnet (6 decimals).
const USDC: &str = "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48";
// Binance 14 hot wallet -- a known active USDC holder.
const HOLDER: &str = "0x28C6c06298d514Db089934071355E5743bf21d60";

#[tokio::main]
async fn main() {
    let settings = Settings::from_yaml("config/settings.yaml").expect("settings");
    let factory = ProviderFactory::from_settings(&settings);
    let provider = factory.create(Chain::Ethereum).expect("eth provider");

    println!("contract: {USDC}");
    println!("holder:   {HOLDER}");

    match provider.get_erc20_balance(USDC, HOLDER).await {
        Ok(raw) => {
            let human = raw as f64 / 1e6;
            println!("USDC balance: {raw} (raw, 6 decimals) = {human:.2} USDC");
        }
        Err(e) => println!("error: {e}"),
    }

    // Sanity: calling get_erc20_balance on a non-EVM chain should be rejected.
    println!();
    println!("(non-EVM providers will return NotImplemented by default)");
}

