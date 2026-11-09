use aic_core::chain::Chain;
use aic_core::{ProviderFactory, Settings};

#[tokio::main]
async fn main() {
    let settings = Settings::from_yaml("config/settings.yaml").expect("load settings");
    let factory = ProviderFactory::from_settings(&settings);
    let provider = factory.create(Chain::Ethereum).expect("eth provider");

    println!("--- broadcast with invalid hex (should fail cleanly, no gas spent) ---");
    match provider.broadcast("0xdeadbeef").await {
        Ok(hash) => println!("unexpected success: {hash}"),
        Err(e) => println!("expected error: {e}"),
    }

    println!("\n--- broadcast with malformed input ---");
    match provider.broadcast("not-hex").await {
        Ok(hash) => println!("unexpected success: {hash}"),
        Err(e) => println!("expected error: {e}"),
    }
}
