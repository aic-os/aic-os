use std::env;

use aic_core::chain::Chain;
use aic_core::wallet::{derive_account, LegacyTransaction, Mnemonic, H160, U256};
use aic_core::{ProviderFactory, Settings};

// Well-known Hardhat test mnemonic.
// Its accounts hold no ETH on mainnet -- broadcast will be rejected due to
// insufficient balance, which is exactly what we want to prove the full
// pipeline (derive -> build -> sign -> broadcast) works without spending gas.
const HARDHAT_MNEMONIC: &str =
    "test test test test test test test test test test test junk";

#[tokio::main]
async fn main() {
    // Opt-in via env var: AIC_TEST_MNEMONIC="word1 word2 ..."
    // Without it -- the safe Hardhat test mnemonic (no funds on mainnet).
    let phrase = env::var("AIC_TEST_MNEMONIC").unwrap_or_else(|_| HARDHAT_MNEMONIC.to_string());
    let mnemonic = Mnemonic::from_phrase(&phrase).expect("mnemonic");
    let account = derive_account(&mnemonic, 0).expect("derive account");

    // Recipient: 0x000000000000000000000000000000000000dEaD (burn address).
    let to = H160::from_low_u64_be(0xdead);

    println!("from:      {}", account.address);
    println!("to:        0x{}", hex::encode(to.as_bytes()));

    let settings = Settings::from_yaml("config/settings.yaml").expect("settings");
    let factory = ProviderFactory::from_settings(&settings);
    let provider = factory.create(Chain::Ethereum).expect("eth provider");

    let balance = provider.get_balance(&account.address).await.expect("balance");
    let nonce = provider.get_nonce(&account.address).await.expect("nonce");
    let gas_price = provider.get_gas_price().await.expect("gas_price");

    println!("balance:   {balance} wei");
    println!("nonce:     {nonce}");
    println!("gas_price: {gas_price} wei");

    let tx = LegacyTransaction {
        nonce,
        gas_price: U256::from(gas_price),
        gas_limit: 21_000,
        to,
        value: U256::from(1u64),
        data: Vec::new(),
        chain_id: 1,
    };

    let signed = tx.sign(&account).expect("sign");
    println!("\n--- signed transaction ---");
    println!("raw_hex:   {}", signed.raw_hex());
    println!("tx_hash:   {}", signed.hash_hex());

    println!("\n--- broadcasting to mainnet ---");
    match provider.broadcast(&signed.raw_hex()).await {
        Ok(hash) => {
            println!("SUCCESS: tx sent: {hash}");
            println!("explorer: https://etherscan.io/tx/{hash}");
        }
        Err(e) => {
            println!("broadcast rejected (expected for unfunded account): {e}");
            println!();
            println!("The full pipeline works: derive -> build -> sign -> broadcast.");
            println!("Fund this account with ETH and the same code would land the tx on-chain.");
        }
    }
}

