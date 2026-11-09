use std::collections::HashMap;

use aic_core::chain::Chain;
use aic_core::providers::factory::ChainSettings;
use aic_core::wallet::{derive_account, LegacyTransaction, Mnemonic, H160, U256};
use aic_core::ProviderFactory;

const ANVIL_RPC: &str = "http://127.0.0.1:8545";
const AICID: &str = "0x5FbDB2315678afecb367f032d93F642f64180aa3";
const HARDHAT_MNEMONIC: &str =
    "test test test test test test test test test test test junk";

#[tokio::main]
async fn main() {
    let mut map = HashMap::new();
    map.insert(Chain::Ethereum, ChainSettings {
        rpc_urls: vec![ANVIL_RPC.to_string()],
        chain_id: Some(31337),
    });
    let factory = ProviderFactory::with_settings(map);
    let provider = factory.create(Chain::Ethereum).expect("provider");

    println!("AICID contract: {AICID}");

    // 1) resolve @alice
    let name = "alice";
    println!("\nResolving @{name} ...");
    match provider.resolve_name(AICID, name).await {
        Ok(Some(addr)) => {
            println!("  @{name} -> {addr}");

            // 2) reverse-resolve the resolved address
            match provider.reverse_resolve(AICID, &addr).await {
                Ok(Some(n)) => println!("  reverse: {addr} -> @{n}"),
                Ok(None) => println!("  reverse: no name bound"),
                Err(e) => println!("  reverse error: {e}"),
            }

            // 3) send 1 wei to @alice from Hardhat account 0
            let mnemonic = Mnemonic::from_phrase(HARDHAT_MNEMONIC).expect("mnemonic");
            let account = derive_account(&mnemonic, 0).expect("derive");
            println!("\nsender: {} ", account.address);
            println!("sender balance: {} wei",
                provider.get_balance(&account.address).await.unwrap_or(0));

            let nonce = provider.get_nonce(&account.address).await.expect("nonce");
            let gas_price = provider.get_gas_price().await.expect("gas_price");

            let addr_bytes = hex::decode(&addr[2..]).expect("decode addr");
            let to_h160 = H160::from_slice(&addr_bytes);

            let tx = LegacyTransaction {
                nonce,
                gas_price: U256::from(gas_price),
                gas_limit: 21_000,
                to: to_h160,
                value: U256::from(1u64),
                data: Vec::new(),
                chain_id: 31337,
            };
            let signed = tx.sign(&account).expect("sign");
            println!("\nbroadcasting 1 wei -> @{name} ({addr}) ...");
            match provider.broadcast(&signed.raw_hex()).await {
                Ok(hash) => println!("  sent! tx hash: {hash}"),
                Err(e) => println!("  broadcast error: {e}"),
            }
        }
        Ok(None) => println!("  @{name} is not registered"),
        Err(e) => println!("  resolve error: {e}"),
    }

    // 4) try to resolve an unknown name
    println!("\nResolving @bob (should be free) ...");
    match provider.resolve_name(AICID, "bob").await {
        Ok(Some(addr)) => println!("  @bob -> {addr}"),
        Ok(None) => println!("  @bob is free"),
        Err(e) => println!("  error: {e}"),
    }

    // 5) reverse-resolve unknown address
    println!("\nReverse-resolving 0x0000000000000000000000000000000000000001 (should be None) ...");
    match provider.reverse_resolve(AICID, "0x0000000000000000000000000000000000000001").await {
        Ok(Some(n)) => println!("  -> @{n}"),
        Ok(None) => println!("  no name"),
        Err(e) => println!("  error: {e}"),
    }
}

