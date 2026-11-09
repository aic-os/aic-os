use aic_core::crypto::pq_crypto;
use aic_core::wallet::{derive_account, LegacyTransaction, Mnemonic, H160, U256};

const HARDHAT_MNEMONIC: &str =
    "test test test test test test test test test test test junk";

const RPC_URL: &str = "http://127.0.0.1:8545";
const AIC_CHAIN_ID: u64 = 2026;

#[tokio::main]
async fn main() {
    let mnemonic = Mnemonic::from_phrase(HARDHAT_MNEMONIC).expect("mnemonic");
    let alice = derive_account(&mnemonic, 0).expect("alice");
    let bob = derive_account(&mnemonic, 1).expect("bob");

    println!("Alice: {}", alice.address);
    println!("Bob:   {}\n", bob.address);

    let alice_addr_bytes: [u8; 20] = hex::decode(&alice.address[2..])
        .unwrap().try_into().unwrap();

    let bob_bytes = hex::decode(&bob.address[2..]).unwrap();
    let bob_h160 = H160::from_slice(&bob_bytes);

    let client = reqwest::Client::new();

    // ============================================================
    // СЦЕНАРИЙ 1: 500 AIC без PQ (мелкая) -- должно пройти
    // ============================================================
    println!("=== СЦЕНАРИЙ 1: 500 AIC без PQ (мелкая tx) ===");
    let tx1 = LegacyTransaction {
        nonce: get_nonce(&client, &alice.address).await,
        gas_price: U256::from(10_000_000_000u64),
        gas_limit: 21000,
        to: bob_h160,
        value: U256::from(500u128 * 10u128.pow(18)),  // 500 AIC
        data: Vec::new(),
        chain_id: AIC_CHAIN_ID,
    };
    let signed1 = tx1.sign(&alice).expect("sign");
    match send_tx(&client, &signed1.raw_hex(), None, None).await {
        Ok(hash) => println!("✅ Принята: {}\n", &hash[..20]),
        Err(e) => println!("❌ Отклонена: {}\n", e),
    }
    tokio::time::sleep(std::time::Duration::from_secs(4)).await;

    // ============================================================
    // СЦЕНАРИЙ 2: 5000 AIC без PQ (крупная) -- должно ОТКЛОНИТЬСЯ
    // ============================================================
    println!("=== СЦЕНАРИЙ 2: 5000 AIC БЕЗ PQ (должна отклониться) ===");
    let tx2 = LegacyTransaction {
        nonce: get_nonce(&client, &alice.address).await,
        gas_price: U256::from(10_000_000_000u64),
        gas_limit: 21000,
        to: bob_h160,
        value: U256::from(5000u128 * 10u128.pow(18)),  // 5000 AIC
        data: Vec::new(),
        chain_id: AIC_CHAIN_ID,
    };
    let signed2 = tx2.sign(&alice).expect("sign");
    match send_tx(&client, &signed2.raw_hex(), None, None).await {
        Ok(hash) => println!("⚠ НЕОЖИДАННО принята: {}\n", &hash[..20]),
        Err(e) => println!("✅ Правильно отклонена: {}\n", e),
    }
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    // ============================================================
    // СЦЕНАРИЙ 3: 5000 AIC С PQ (крупная, Dilithium) -- должно пройти
    // ============================================================
    println!("=== СЦЕНАРИЙ 3: 5000 AIC С PQ (Schnorr + Dilithium) ===");
    let (pq_pk, pq_sk) = pq_crypto::generate_keypair();
    let dilithium_pk = pq_pk.dilithium.clone().expect("dilithium pk");

    let tx3 = LegacyTransaction {
        nonce: get_nonce(&client, &alice.address).await,
        gas_price: U256::from(10_000_000_000u64),
        gas_limit: 21000,
        to: bob_h160,
        value: U256::from(5000u128 * 10u128.pow(18)),  // 5000 AIC
        data: Vec::new(),
        chain_id: AIC_CHAIN_ID,
    };

    // PQ-сообщение (то же, что считает нода)
    let pq_msg = tx3.pq_message(alice_addr_bytes);

    // Подпись: sign_hybrid -- для клиента нам нужна только Dilithium-часть
    let hybrid_sig = pq_crypto::sign_hybrid(&pq_msg, &pq_sk, true).expect("sign pq");
    let dilithium_sig = hybrid_sig.dilithium.expect("dilithium sig");

    let signed3 = tx3.sign(&alice).expect("sign");
    match send_tx(
        &client,
        &signed3.raw_hex(),
        Some(&hex::encode(&dilithium_pk)),
        Some(&hex::encode(&dilithium_sig)),
    ).await {
        Ok(hash) => println!("✅ Принята с PQ: {}\n", &hash[..20]),
        Err(e) => println!("❌ Отклонена: {}\n", e),
    }
    tokio::time::sleep(std::time::Duration::from_secs(4)).await;

    // Финальные балансы
    let bob_bal = get_balance(&client, &bob.address).await;
    let alice_bal = get_balance(&client, &alice.address).await;
    println!("=== ИТОГ ===");
    println!("Alice: {:.4} AIC", alice_bal as f64 / 1e18);
    println!("Bob:   {:.4} AIC (ожидаем 10500 AIC)", bob_bal as f64 / 1e18);
}

async fn send_tx(
    client: &reqwest::Client,
    raw_hex: &str,
    pq_pk_hex: Option<&str>,
    pq_sig_hex: Option<&str>,
) -> Result<String, String> {
    let mut params = vec![serde_json::json!(raw_hex)];
    if let Some(pk) = pq_pk_hex {
        params.push(serde_json::json!(format!("0x{}", pk)));
    } else {
        params.push(serde_json::json!(null));
    }
    if let Some(sig) = pq_sig_hex {
        params.push(serde_json::json!(format!("0x{}", sig)));
    } else {
        params.push(serde_json::json!(null));
    }

    let body = serde_json::json!({
        "jsonrpc": "2.0", "id": 1,
        "method": "aic_sendRawTransaction",
        "params": params
    });
    let resp = client.post(RPC_URL).json(&body).send().await
        .map_err(|e| e.to_string())?;
    let json: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;

    if let Some(err) = json.get("error") {
        let msg = err.get("message").and_then(|m| m.as_str()).unwrap_or("unknown");
        return Err(msg.to_string());
    }
    json.get("result").and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| "no result".to_string())
}

async fn get_nonce(client: &reqwest::Client, addr: &str) -> u64 {
    let body = serde_json::json!({
        "jsonrpc": "2.0", "id": 1, "method": "aic_getAccount", "params": [addr]
    });
    let resp = client.post(RPC_URL).json(&body).send().await.expect("post");
    let json: serde_json::Value = resp.json().await.expect("json");
    json.get("result").and_then(|r| r.get("nonce")).and_then(|n| n.as_u64()).unwrap_or(0)
}

async fn get_balance(client: &reqwest::Client, addr: &str) -> u128 {
    let body = serde_json::json!({
        "jsonrpc": "2.0", "id": 1, "method": "aic_getBalance", "params": [addr]
    });
    let resp = client.post(RPC_URL).json(&body).send().await.expect("post");
    let json: serde_json::Value = resp.json().await.expect("json");
    let hex_str = json.get("result").and_then(|v| v.as_str()).unwrap_or("0x0");
    let h = hex_str.trim_start_matches("0x");
    if h.is_empty() { 0 } else { u128::from_str_radix(h, 16).unwrap_or(0) }
}

