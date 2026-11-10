use aic_core::wallet::{derive_account, LegacyTransaction, Mnemonic, H160, U256};
use aic_core::providers::evm_abi;

const HARDHAT_MNEMONIC: &str = "test test test test test test test test test test test junk";
const RPC_URL: &str = "http://127.0.0.1:8545";
const CHAIN_ID: u64 = 2026;
const BOUNTY: &str = "0x0000000000000000000000000000000000b0b000";

#[tokio::main]
async fn main() {
    let mnemonic = Mnemonic::from_phrase(HARDHAT_MNEMONIC).expect("mnemonic");
    let alice = derive_account(&mnemonic, 0).expect("alice");
    println!("Alice: {}", alice.address);

    let client = reqwest::Client::new();
    let bounty_h160 = H160::from_slice(&hex::decode(&BOUNTY[2..]).unwrap());
    let alice_h160 = H160::from_slice(&hex::decode(&alice.address[2..]).unwrap());

    // ========== TX 1: fund() with 3M AIC ==========
    println!("\n=== TX 1: fund() 300 000 AIC ===");
    let nonce = get_nonce(&client, &alice.address).await;
    let fund_sel = evm_abi::selector("fund()");
    let tx1 = LegacyTransaction {
        nonce,
        gas_price: U256::from(10_000_000_000u64),
        gas_limit: 100_000,
        to: bounty_h160,
        value: U256::from(300_000u128 * 10u128.pow(18)),
        data: fund_sel.to_vec(),
        chain_id: CHAIN_ID,
    };
    let signed1 = tx1.sign(&alice).expect("sign");
    let h1 = broadcast(&client, &signed1.raw_hex()).await;
    println!("  tx: {}", h1);
    tokio::time::sleep(std::time::Duration::from_secs(4)).await;

    // ========== TX 2: setPQReward(2M AIC) ==========
    println!("\n=== TX 2: setPQReward(200 000 AIC) ===");
    let nonce = get_nonce(&client, &alice.address).await;
    let pq_sel = evm_abi::selector("setPQReward(uint256)");
    let mut data2 = Vec::new();
    data2.extend_from_slice(&pq_sel);
    data2.extend_from_slice(&evm_abi::encode_u256(200_000u128 * 10u128.pow(18)));
    let tx2 = LegacyTransaction {
        nonce,
        gas_price: U256::from(10_000_000_000u64),
        gas_limit: 100_000,
        to: bounty_h160,
        value: U256::zero(),
        data: data2,
        chain_id: CHAIN_ID,
    };
    let signed2 = tx2.sign(&alice).expect("sign");
    let h2 = broadcast(&client, &signed2.raw_hex()).await;
    println!("  tx: {}", h2);
    tokio::time::sleep(std::time::Duration::from_secs(4)).await;

    // ========== TX 3: setClassicChallenge(Alice, 1M AIC, message) ==========
    println!("\n=== TX 3: setClassicChallenge(Alice, 100 000 AIC) ===");
    let nonce = get_nonce(&client, &alice.address).await;
    let cc_sel = evm_abi::selector("setClassicChallenge(address,uint256,string)");
    let message = "QuantumBounty v1: Break secp256k1 with a quantum computer.";
    let mut data3 = Vec::new();
    data3.extend_from_slice(&cc_sel);
    // arg0: address (static, 32 bytes padded)
    let mut addr_pad = [0u8; 32];
    addr_pad[12..].copy_from_slice(alice_h160.as_bytes());
    data3.extend_from_slice(&addr_pad);
    // arg1: uint256 reward = 100 000 AIC
    data3.extend_from_slice(&evm_abi::encode_u256(100_000u128 * 10u128.pow(18)));
    // arg2: offset to string = 0x60 (after 3 static 32-byte words)
    let mut off = [0u8; 32];
    off[31] = 0x60;
    data3.extend_from_slice(&off);
    // string length + data (padded)
    let msg_bytes = message.as_bytes();
    let mut len_buf = [0u8; 32];
    len_buf[24..32].copy_from_slice(&(msg_bytes.len() as u64).to_be_bytes());
    data3.extend_from_slice(&len_buf);
    data3.extend_from_slice(msg_bytes);
    let pad = (32 - (msg_bytes.len() % 32)) % 32;
    data3.extend(std::iter::repeat(0u8).take(pad));

    let tx3 = LegacyTransaction {
        nonce,
        gas_price: U256::from(10_000_000_000u64),
        gas_limit: 200_000,
        to: bounty_h160,
        value: U256::zero(),
        data: data3,
        chain_id: CHAIN_ID,
    };
    let signed3 = tx3.sign(&alice).expect("sign");
    let h3 = broadcast(&client, &signed3.raw_hex()).await;
    println!("  tx: {}", h3);
    tokio::time::sleep(std::time::Duration::from_secs(4)).await;

    // ========== Финал ==========
    println!("\n=== Итог: aic_quantumBountyInfo ===");
    let body = serde_json::json!({"jsonrpc":"2.0","id":1,"method":"aic_quantumBountyInfo","params":[]});
    let resp = client.post(RPC_URL).json(&body).send().await.expect("post");
    let r: serde_json::Value = resp.json().await.expect("json");
    println!("{}", serde_json::to_string_pretty(&r).unwrap());
}

async fn broadcast(client: &reqwest::Client, raw: &str) -> String {
    let body = serde_json::json!({
        "jsonrpc":"2.0","id":1,"method":"aic_sendRawTransaction","params":[raw]
    });
    let resp = client.post(RPC_URL).json(&body).send().await.expect("post");
    let j: serde_json::Value = resp.json().await.expect("json");
    if let Some(err) = j.get("error") {
        format!("ERROR: {}", err)
    } else {
        j.get("result").and_then(|v| v.as_str()).unwrap_or("?").to_string()
    }
}

async fn get_nonce(client: &reqwest::Client, addr: &str) -> u64 {
    let body = serde_json::json!({
        "jsonrpc":"2.0","id":1,"method":"aic_getAccount","params":[addr]
    });
    let resp = client.post(RPC_URL).json(&body).send().await.expect("post");
    let j: serde_json::Value = resp.json().await.expect("json");
    j.get("result").and_then(|r| r.get("nonce")).and_then(|n| n.as_u64()).unwrap_or(0)
}

