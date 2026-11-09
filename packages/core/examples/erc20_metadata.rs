use aic_core::chain::Chain;
use aic_core::{ProviderFactory, Settings};

const USDC: &str = "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48";

#[tokio::main]
async fn main() {
    let settings = Settings::from_yaml("config/settings.yaml").expect("settings");
    let factory = ProviderFactory::from_settings(&settings);
    let provider = factory.create(Chain::Ethereum).expect("eth provider");

    println!("contract: {USDC}");
    let name = provider.get_erc20_name(USDC).await.expect("name");
    let symbol = provider.get_erc20_symbol(USDC).await.expect("symbol");
    let decimals = provider.get_erc20_decimals(USDC).await.expect("decimals");
    println!("name:     {name}");
    println!("symbol:   {symbol}");
    println!("decimals: {decimals}");
}

