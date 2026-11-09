use aic_core::wallet::{derive_account, LegacyTransaction, Mnemonic, H160, U256};
use aic_core::providers::evm_abi;

const HARDHAT_MNEMONIC: &str =
    "test test test test test test test test test test test junk";

const RPC_URL: &str = "http://127.0.0.1:8545";
const AIC_CHAIN_ID: u64 = 2026;
const AICID: &str = "0x0000000000000000000000000000000000a1c1d0";

#[tokio::main]
async fn main() {
    let mnemonic = Mnemonic::from_phrase(HARDHAT_MNEMONIC).expect("mnemonic");
    let alice = derive_account(&mnemonic, 0).expect("alice");
    println!("Alice: {}", alice.address);

    let client = reqwest::Client::new();

    // 1) Проверяем resolve('alice') ДО регистрации
    println!("\n=== resolve('alice') ДО регистрации ===");
    let before = call_resolve(&client, "alice").await;
    println!("  {}", before);

    // 2) Строим calldata: register("alice", alice_address)
    let selector = evm_abi::selector("register(string,address)");
    println!("\nselector(register): 0x{}", hex::encode(selector));

    // ABI-encode: string "alice" (dynamic) + address alice (static)
    // Layout: [4-byte selector][offset_string=0x40][address_padded][length=5][alice padded]
    let mut data: Vec<u8> = Vec::new();
    data.extend_from_slice(&selector);
    // offset для string = 0x40 (64) -- после 32 байт address
    let mut offset = [0u8; 32]; offset[31] = 0x40;
    data.extend_from_slice(&offset);
    // address alice (padded)
    let alice_addr_bytes = hex::decode(&alice.address[2..]).unwrap();
    let mut addr_padded = [0u8; 32];
    addr_padded[12..].copy_from_slice(&alice_addr_bytes);
    data.extend_from_slice(&addr_padded);
    // string length = 5
    let mut len_buf = [0u8; 32]; len_buf[31] = 5;
    data.extend_from_slice(&len_buf);
    // "alice" padded
    data.extend_from_slice(b"alice");
    data.extend(std::iter::repeat(0u8).take(27));

    println!("calldata len: {} байт", data.len());

    // Проверяем баланс казны ДО
    let treasury_addr = "0x000000000000000000000000000000000000000f";
    let treasury_before = get_balance(&client, treasury_addr).await;
    println!("\nКазна ДО: {} wei ({:.4} AIC)", treasury_before, treasury_before as f64 / 1e18);

    // 3) Получаем nonce, баланс
    let nonce = get_nonce(&client, &alice.address).await;
    let balance = get_balance(&client, &alice.address).await;
    println!("\nAlice nonce: {}, balance: {} wei ({:.4} AIC)", nonce, balance, balance as f64 / 1e18);

    // 4) Строим Legacy EIP-155 транзакцию (type 0)
    // gas_price = 10 gwei, gas_limit = 200_000
    // value = 10 AIC -- оплата регистрации в казну
    let registration_fee = U256::from(10_000_000_000_000_000_000u128); // 10 AIC // 10 AIC = 10 * 1e18
    let tx = LegacyTransaction {
        nonce,
        gas_price: U256::from(10_000_000_000u64),
        gas_limit: 500_000,
        to: H160::from_slice(&hex::decode(&AICID[2..]).unwrap()),
        value: registration_fee,
        data,
        chain_id: AIC_CHAIN_ID,
    };

    let signed = tx.sign(&alice).expect("sign");
    println!("\nПодписано. raw: {}...", &signed.raw_hex()[..66]);
    println!("tx_hash: {}", signed.hash_hex());

    // 5) Отправляем в ноду
    println!("\n=== Отправка aic_sendRawTransaction ===");
    let body = serde_json::json!({
        "jsonrpc": "2.0", "id": 1,
        "method": "aic_sendRawTransaction",
        "params": [signed.raw_hex()]
    });
    let resp = client.post(RPC_URL).json(&body).send().await.expect("post");
    let json: serde_json::Value = resp.json().await.expect("json");
    println!("  {}", serde_json::to_string_pretty(&json).unwrap());

    if json.get("error").is_some() {
        println!("❌ Отправка отклонена");
        return;
    }

    // 6) Ждём блок (3 сек) + запас
    println!("\nЖдём 5 секунд...");
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    // Проверяем баланс казны ПОСЛЕ
    let treasury_after = get_balance(&client, treasury_addr).await;
    println!("\nКазна ПОСЛЕ: {} wei ({:.4} AIC)", treasury_after, treasury_after as f64 / 1e18);
    println!("Прирост: {:.4} AIC", (treasury_after.saturating_sub(treasury_before)) as f64 / 1e18);

    // 7) Проверяем resolve('alice') ПОСЛЕ
    println!("\n=== resolve('alice') ПОСЛЕ регистрации ===");
    let after = call_resolve(&client, "alice").await;
    println!("  {}", after);

    let alice_clean = alice.address.to_lowercase();
    let after_clean = after.to_lowercase();
    let alice_padded = format!("0x{}{}", "0".repeat(24), &alice_clean[2..]);

    if after_clean == alice_padded {
        println!("\n✅✅✅ РЕГИСТРАЦИЯ РАБОТАЕТ!");
        println!("   @alice -> {} (совпадает с адресом Alice)", alice.address);
        println!("   AIC-OS -- полноценная криптовалюта с работающими контрактами.");
    } else if after == "0x0000000000000000000000000000000000000000000000000000000000000000" {
        println!("\n⚠ Ещё не зарегистрировано (возможно, транзакция ещё в mempool)");
    } else {
        println!("\n⚠ Неожиданный ответ: {}", after);
    }
}

async fn call_resolve(client: &reqwest::Client, name: &str) -> String {
    let sel = evm_abi::selector("resolve(string)");
    let mut data = Vec::new();
    data.extend_from_slice(&sel);
    data.extend_from_slice(&evm_abi::encode_string_arg(name));
    let hex_str = format!("0x{}", hex::encode(&data));

    let body = serde_json::json!({
        "jsonrpc": "2.0", "id": 1,
        "method": "aic_call",
        "params": [AICID, hex_str]
    });
    let resp = client.post(RPC_URL).json(&body).send().await.expect("post");
    let json: serde_json::Value = resp.json().await.expect("json");
    json.get("result").and_then(|v| v.as_str()).unwrap_or("error").to_string()
}

async fn get_nonce(client: &reqwest::Client, addr: &str) -> u64 {
    let body = serde_json::json!({
        "jsonrpc": "2.0", "id": 1, "method": "aic_getAccount", "params": [addr]
    });
    let resp = client.post(RPC_URL).json(&body).send().await.expect("getAccount");
    let json: serde_json::Value = resp.json().await.expect("json");
    json.get("result").and_then(|r| r.get("nonce")).and_then(|n| n.as_u64()).unwrap_or(0)
}

async fn get_balance(client: &reqwest::Client, addr: &str) -> u128 {
    let body = serde_json::json!({
        "jsonrpc": "2.0", "id": 1, "method": "aic_getBalance", "params": [addr]
    });
    let resp = client.post(RPC_URL).json(&body).send().await.expect("getBalance");
    let json: serde_json::Value = resp.json().await.expect("json");
    let hex_str = json.get("result").and_then(|v| v.as_str()).unwrap_or("0x0");
    let h = hex_str.trim_start_matches("0x");
    if h.is_empty() { 0 } else { u128::from_str_radix(h, 16).unwrap_or(0) }
}







