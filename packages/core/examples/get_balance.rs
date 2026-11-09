use std::collections::HashMap;

use aic_core::chain::Chain;
use aic_core::providers::factory::ChainSettings;
use aic_core::ProviderFactory;

#[tokio::main]
async fn main() {
    let mut settings = HashMap::new();
    settings.insert(Chain::Ethereum, ChainSettings {
        rpc_url: "https://ethereum-rpc.publicnode.com".to_string(),
        chain_id: Some(1),
    });

    let factory = ProviderFactory::with_settings(settings);
    let provider = factory.create(Chain::Ethereum).expect("provider");

    // Vitalik's public address
    let addr = "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045";

    println!("health: {:?}", provider.health().await);

    match provider.get_balance(addr).await {
        Ok(wei) => {
            let eth = wei as f64 / 1e18;
            println!("balance: {wei} wei = {eth:.6} ETH");
        }
        Err(e) => println!("error: {e}"),
    }
}
