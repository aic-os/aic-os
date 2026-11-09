
use aic_core::wallet::{derive_account, LegacyTransaction, Mnemonic, H160, U256};

// Hardhat test mnemonic -- известные ключи, НИКОГДА не использовать на реальные деньги.
const HARDHAT_MNEMONIC: &str =
    "test test test test test test test test test test test junk";

const RPC_URL: &str = "http://127.0.0.1:8545";
const AIC_CHAIN_ID: u64 = 2026;

#[tokio::main]
async fn main() {
    let mnemonic = Mnemonic::from_phrase(HARDHAT_MNEMONIC).expect("mnemonic");
    let alice = derive_account(&mnemonic, 0).expect("alice");
    let bob = derive_account(&mnemonic, 1).expect("bob");

    println!("Alice: {} (отправитель)", alice.address);
    println!("Bob:   {} (получатель)", bob.address);

    let client = reqwest::Client::new();

    // Проверяем начальные балансы
    let alice_before = rpc_get_balance(&client, &alice.address).await;
    let bob_before = rpc_get_balance(&client, &bob.address).await;
    println!("\nДО:");
    println!("  Alice: {} AIC", wei_to_aic(alice_before));
    println!("  Bob:   {} AIC", wei_to_aic(bob_before));

    // Получаем nonce
    let nonce = rpc_get_nonce(&client, &alice.address).await;
    println!("\nAlice nonce: {nonce}");

    // Строим транзакцию: 1000 AIC, gas_price 1 gwei, gas_limit 21000, chain_id 2026
    let amount_aic: u128 = 1000;
    let amount_wei: u128 = amount_aic * 1_000_000_000_000_000_000;
    let gas_price_wei: u128 = 1_000_000_000; // 1 gwei
    let gas_limit: u64 = 21_000;

    let bob_bytes = hex::decode(&bob.address[2..]).expect("bob addr");
    let bob_h160 = H160::from_slice(&bob_bytes);

    let tx = LegacyTransaction {
        nonce,
        gas_price: U256::from(gas_price_wei),
        gas_limit,
        to: bob_h160,
        value: U256::from(amount_wei),
        data: Vec::new(),
        chain_id: AIC_CHAIN_ID,
    };

    let signed = tx.sign(&alice).expect("sign");
    let raw_hex = signed.raw_hex();
    println!("\nПодписано. raw: {}...", &raw_hex[..66.min(raw_hex.len())]);
    println!("tx_hash: {}", signed.hash_hex());

    // Отправляем в ноду
    let resp = client
        .post(RPC_URL)
        .json(&serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "aic_sendRawTransaction",
            "params": [raw_hex]
        }))
        .send()
        .await
        .expect("post");

    let json: serde_json::Value = resp.json().await.expect("json");
    println!("\nRPC ответ: {}", serde_json::to_string_pretty(&json).unwrap());

    if json.get("error").is_some() {
        println!("❌ Отправка отклонена");
        return;
    }

    println!("\nЖдём 5 секунд (транзакция должна попасть в блок)...");
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    let alice_after = rpc_get_balance(&client, &alice.address).await;
    let bob_after = rpc_get_balance(&client, &bob.address).await;
    println!("\nПОСЛЕ:");
    println!("  Alice: {} AIC", wei_to_aic(alice_after));
    println!("  Bob:   {} AIC", wei_to_aic(bob_after));

    let expected_gas_fee = gas_price_wei * (gas_limit as u128);
    let alice_diff = alice_before.saturating_sub(alice_after);
    let bob_diff = bob_after.saturating_sub(bob_before);
    println!("\nИзменения:");
    println!("  Alice потеряла: {} AIC (1000 отправлено + {} газа)",
        wei_to_aic(alice_diff), wei_to_aic(expected_gas_fee));
    println!("  Bob получил:    {} AIC", wei_to_aic(bob_diff));

    if bob_diff == amount_wei {
        println!("\n✅ УСПЕХ: транзакция обработана, баланс Bob'а пополнился на 1000 AIC");
    } else {
        println!("\n❌ ОШИБКА: Bob получил {}, ожидалось {}", bob_diff, amount_wei);
    }
}

fn wei_to_aic(wei: u128) -> String {
    let whole = wei / 1_000_000_000_000_000_000;
    let frac = wei % 1_000_000_000_000_000_000;
    if frac == 0 { format!("{whole}") }
    else { format!("{}.{:018}", whole, frac).trim_end_matches('0').to_string() }
}

async fn rpc_get_balance(client: &reqwest::Client, addr: &str) -> u128 {
    let resp = client.post(RPC_URL).json(&serde_json::json!({
        "jsonrpc": "2.0", "id": 1, "method": "aic_getBalance", "params": [addr]
    })).send().await.expect("getBalance");
    let json: serde_json::Value = resp.json().await.expect("json");
    let hex_str = json.get("result").and_then(|v| v.as_str()).unwrap_or("0x0");
    let h = hex_str.trim_start_matches("0x");
    if h.is_empty() { return 0; }
    u128::from_str_radix(h, 16).unwrap_or(0)
}

async fn rpc_get_nonce(client: &reqwest::Client, addr: &str) -> u64 {
    let resp = client.post(RPC_URL).json(&serde_json::json!({
        "jsonrpc": "2.0", "id": 1, "method": "aic_getAccount", "params": [addr]
    })).send().await.expect("getAccount");
    let json: serde_json::Value = resp.json().await.expect("json");
    json.get("result").and_then(|r| r.get("nonce")).and_then(|n| n.as_u64()).unwrap_or(0)
}

