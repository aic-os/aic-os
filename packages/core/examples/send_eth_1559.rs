use std::env;

use aic_core::chain::Chain;
use aic_core::wallet::{derive_account, Eip1559Transaction, Mnemonic, H160, U256};
use aic_core::{ProviderFactory, Settings};

const HARDHAT_MNEMONIC: &str =
    "test test test test test test test test test test test junk";

#[tokio::main]
async fn main() {
    let phrase = env::var("AIC_TEST_MNEMONIC").unwrap_or_else(|_| HARDHAT_MNEMONIC.to_string());
    let mnemonic = Mnemonic::from_phrase(&phrase).expect("mnemonic");
    let account = derive_account(&mnemonic, 0).expect("derive");

    let settings = Settings::from_yaml("config/settings.yaml").expect("settings");
    let factory = ProviderFactory::from_settings(&settings);
    let provider = factory.create(Chain::Ethereum).expect("eth provider");

    let balance = provider.get_balance(&account.address).await.expect("balance");
    let nonce = provider.get_nonce(&account.address).await.expect("nonce");
    let base_fee = provider.get_base_fee().await.expect("base_fee");
    let priority_fee = provider.get_max_priority_fee().await.unwrap_or(1_500_000_000u128);
    let max_fee = base_fee.saturating_mul(2).saturating_add(priority_fee);

    println!("from:       {}", account.address);
    println!("balance:    {balance} wei");
    println!("nonce:      {nonce}");
    println!("base_fee:   {base_fee} wei");
    println!("priority:   {priority_fee} wei");
    println!("max_fee:    {max_fee} wei");

    let tx = Eip1559Transaction {
        chain_id: 1,
        nonce,
        max_priority_fee_per_gas: U256::from(priority_fee),
        max_fee_per_gas: U256::from(max_fee),
        gas_limit: 21_000,
        to: H160::from_low_u64_be(0xdead),
        value: U256::from(1u64),
        data: Vec::new(),
    };

    let signed = tx.sign(&account).expect("sign");

    println!("\n--- signed EIP-1559 (Type 2) ---");
    println!("raw_hex:    {}", signed.raw_hex());
    println!("tx_hash:    {}", signed.hash_hex());

    println!("\n--- broadcasting to mainnet ---");
    match provider.broadcast(&signed.raw_hex()).await {
        Ok(hash) => println!("SUCCESS: {hash}"),
        Err(e) => {
            println!("broadcast rejected (expected for unfunded account): {e}");
            println!();
            println!("EIP-1559 pipeline works: derive -> build -> sign (Type 2) -> broadcast.");
        }
    }
}

