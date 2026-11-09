use aic_core::chain::Chain;
use aic_core::{ProviderFactory, Settings};

#[tokio::main]
async fn main() {
    let settings = Settings::from_yaml("config/settings.yaml").expect("load settings");
    let factory = ProviderFactory::from_settings(&settings);

    let vitalik = "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045";
    let recipient = "0x000000000000000000000000000000000000dEaD";
    let amount = 1_000_000_000_000_000u128; // 0.001 ETH

    for chain in [Chain::Ethereum, Chain::Polygon, Chain::Bsc] {
        println!("--- {} ---", chain.as_str());
        match factory.create(chain) {
            Ok(provider) => {
                println!("health: {:?}", provider.health().await);
                match provider.get_balance(vitalik).await {
                    Ok(wei) => println!("balance: {:.6} native", wei as f64 / 1e18),
                    Err(e) => println!("balance error: {e}"),
                }
                match provider.estimate_fee(vitalik, recipient, amount).await {
                    Ok(fee) => println!("fee: {:.8} native", fee as f64 / 1e18),
                    Err(e) => println!("fee error: {e}"),
                }
            }
            Err(e) => println!("create error: {e}"),
        }
    }
}
