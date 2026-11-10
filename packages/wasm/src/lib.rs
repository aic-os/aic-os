//! AIC WASM -- подпись транзакций в браузере. Самодостаточный, без aic-core.
use wasm_bindgen::prelude::*;
use serde::Serialize;
use sha3::{Digest, Keccak256};
use k256::ecdsa::{SigningKey, RecoveryId, Signature, VerifyingKey};
use primitive_types::{H160, U256};
use rlp::RlpStream;

fn selector(sig: &str) -> [u8; 4] {
    let hash = Keccak256::digest(sig.as_bytes());
    [hash[0], hash[1], hash[2], hash[3]]
}

fn parse_privkey(hex_str: &str) -> Result<[u8; 32], JsError> {
    let clean = hex_str.trim_start_matches("0x");
    let bytes = hex::decode(clean).map_err(|e| JsError::new(&format!("bad hex: {e}")))?;
    if bytes.len() != 32 {
        return Err(JsError::new(&format!("privkey must be 32 bytes, got {}", bytes.len())));
    }
    let mut arr = [0u8; 32];
    arr.copy_from_slice(&bytes);
    Ok(arr)
}

fn pubkey_to_eip55(pubkey_33: &[u8; 33]) -> Result<String, JsError> {
    let vk = VerifyingKey::from_sec1_bytes(pubkey_33)
        .map_err(|e| JsError::new(&format!("vk: {e}")))?;
    let uncompressed = vk.to_encoded_point(false);
    let bytes = uncompressed.as_bytes();
    let h = Keccak256::digest(&bytes[1..]);
    let mut addr = [0u8; 20];
    addr.copy_from_slice(&h[12..]);
    Ok(to_eip55(&addr))
}

fn to_eip55(addr: &[u8; 20]) -> String {
    let hex_addr = hex::encode(addr);
    let mut hasher = Keccak256::new();
    hasher.update(hex_addr.as_bytes());
    let hash_hex = hex::encode(hasher.finalize());
    let mut out = String::with_capacity(42);
    out.push_str("0x");
    for (i, ch) in hex_addr.chars().enumerate() {
        if ch.is_ascii_digit() {
            out.push(ch);
        } else {
            let nibble = u8::from_str_radix(&hash_hex[i..=i], 16).unwrap_or(0);
            if nibble >= 8 { out.push(ch.to_ascii_uppercase()); }
            else { out.push(ch); }
        }
    }
    out
}

#[wasm_bindgen]
pub fn generate_mnemonic_12() -> Result<String, JsError> {
    use bip39::{Language, Mnemonic};
    use rand::RngCore;
    let mut entropy = vec![0u8; 16];
    rand::thread_rng().fill_bytes(&mut entropy);
    let m = Mnemonic::from_entropy_in(Language::English, &entropy)
        .map_err(|e| JsError::new(&format!("mnemonic: {e}")))?;
    Ok(m.to_string())
}

#[derive(Serialize)]
struct WalletInfo {
    address: String,
    private_key_hex: String,
    mnemonic: String,
}

#[wasm_bindgen]
pub fn wallet_from_mnemonic(mnemonic: &str, index: u32) -> Result<JsValue, JsError> {
    use bip39::{Language, Mnemonic};
    use bip32::{DerivationPath, XPrv};
    use std::str::FromStr;

    let m = Mnemonic::parse_in_normalized(Language::English, mnemonic)
        .map_err(|e| JsError::new(&format!("mnemonic: {e}")))?;
    let seed = m.to_seed("");
    let path_str = format!("m/44'\''/60'\''/0'\''/0/{index}");
    let path = DerivationPath::from_str(&path_str)
        .map_err(|e| JsError::new(&format!("path: {e}")))?;
    let xprv = XPrv::derive_from_path(seed.as_slice(), &path)
        .map_err(|e| JsError::new(&format!("derive: {e}")))?;

    let privkey = xprv.private_key().to_bytes();
    let mut pk_arr = [0u8; 32];
    pk_arr.copy_from_slice(&privkey);

    let sk = SigningKey::from_bytes((&pk_arr).into())
        .map_err(|e| JsError::new(&format!("sk: {e}")))?;
    let vk = sk.verifying_key();
    let compressed = vk.to_encoded_point(true);
    let mut pubkey_arr = [0u8; 33];
    pubkey_arr.copy_from_slice(compressed.as_bytes());
    let address = pubkey_to_eip55(&pubkey_arr)?;

    let info = WalletInfo {
        address,
        private_key_hex: format!("0x{}", hex::encode(&pk_arr)),
        mnemonic: m.to_string(),
    };
    serde_wasm_bindgen::to_value(&info).map_err(|e| JsError::new(&e.to_string()))
}

#[wasm_bindgen]
pub fn address_from_privkey(privkey_hex: &str) -> Result<String, JsError> {
    let pk = parse_privkey(privkey_hex)?;
    let sk = SigningKey::from_bytes((&pk).into())
        .map_err(|e| JsError::new(&format!("sk: {e}")))?;
    let vk = sk.verifying_key();
    let compressed = vk.to_encoded_point(true);
    let mut pubkey_arr = [0u8; 33];
    pubkey_arr.copy_from_slice(compressed.as_bytes());
    pubkey_to_eip55(&pubkey_arr)
}

// ============ ABI-encode ============

#[wasm_bindgen]
pub fn encode_aicid_register(name: &str, owner_addr: &str) -> Result<String, JsError> {
    let sel = selector("register(string,address)");
    let name_bytes = name.as_bytes();
    let len = name_bytes.len();
    let padded = ((len + 31) / 32) * 32;

    let owner_clean = owner_addr.trim_start_matches("0x");
    let owner_bytes = hex::decode(owner_clean).map_err(|e| JsError::new(&e.to_string()))?;
    if owner_bytes.len() != 20 {
        return Err(JsError::new("owner must be 20-byte address"));
    }

    let mut out = Vec::with_capacity(4 + 32 * 3 + padded);
    out.extend_from_slice(&sel);
    let mut off = [0u8; 32];
    off[31] = 0x40;
    out.extend_from_slice(&off);
    let mut addr_pad = [0u8; 32];
    addr_pad[12..].copy_from_slice(&owner_bytes);
    out.extend_from_slice(&addr_pad);
    let mut l = [0u8; 32];
    l[24..32].copy_from_slice(&(len as u64).to_be_bytes());
    out.extend_from_slice(&l);
    out.extend_from_slice(name_bytes);
    out.extend(std::iter::repeat(0u8).take(padded - len));

    Ok(format!("0x{}", hex::encode(&out)))
}

#[wasm_bindgen]
pub fn encode_resolve_call(name: &str) -> Result<String, JsError> {
    let sel = selector("resolve(string)");
    let name_bytes = name.as_bytes();
    let len = name_bytes.len();
    let padded = ((len + 31) / 32) * 32;

    let mut out = Vec::with_capacity(4 + 64 + padded);
    out.extend_from_slice(&sel);
    let mut off = [0u8; 32];
    off[31] = 0x20;
    out.extend_from_slice(&off);
    let mut l = [0u8; 32];
    l[24..32].copy_from_slice(&(len as u64).to_be_bytes());
    out.extend_from_slice(&l);
    out.extend_from_slice(name_bytes);
    out.extend(std::iter::repeat(0u8).take(padded - len));
    Ok(format!("0x{}", hex::encode(&out)))
}

// ============ Подпись транзакции ============

fn be_trim(v: &[u8; 32]) -> &[u8] {
    let start = v.iter().position(|&b| b != 0).unwrap_or(32);
    &v[start..]
}

#[derive(Serialize)]
struct SignedTxInfo {
    from: String,
    raw_hex: String,
    tx_hash: String,
}

#[wasm_bindgen]
pub fn sign_legacy_tx(
    privkey_hex: &str,
    to_hex: &str,
    value_hex: &str,
    nonce: f64,
    gas_price_hex: &str,
    gas_limit: f64,
    data_hex: &str,
    chain_id: f64,
) -> Result<JsValue, JsError> {
    let pk = parse_privkey(privkey_hex)?;
    let sk = SigningKey::from_bytes((&pk).into())
        .map_err(|e| JsError::new(&format!("sk: {e}")))?;
    let vk = sk.verifying_key();
    let compressed = vk.to_encoded_point(true);
    let mut pubkey_arr = [0u8; 33];
    pubkey_arr.copy_from_slice(compressed.as_bytes());
    let from_address = pubkey_to_eip55(&pubkey_arr)?;

    let nonce_u64 = nonce as u64;
    let gas_limit_u64 = gas_limit as u64;
    let chain_id_u64 = chain_id as u64;

    let to_clean = to_hex.trim_start_matches("0x");
    let to_bytes = hex::decode(to_clean).map_err(|e| JsError::new(&format!("to: {e}")))?;
    if to_bytes.len() != 20 {
        return Err(JsError::new("to must be 20-byte address"));
    }
    let to_h160 = H160::from_slice(&to_bytes);

    let v_clean = value_hex.trim_start_matches("0x");
    let value = if v_clean.is_empty() { 0u128 } else {
        u128::from_str_radix(v_clean, 16).map_err(|e| JsError::new(&format!("value: {e}")))?
    };

    let gp_clean = gas_price_hex.trim_start_matches("0x");
    let gas_price = if gp_clean.is_empty() { 1_000_000_000u128 } else {
        u128::from_str_radix(gp_clean, 16).map_err(|e| JsError::new(&format!("gas_price: {e}")))?
    };

    let data_clean = data_hex.trim_start_matches("0x");
    let data = if data_clean.is_empty() { Vec::new() } else {
        hex::decode(data_clean).map_err(|e| JsError::new(&format!("data: {e}")))?
    };

    let value_u256 = U256::from(value);
    let gp_u256 = U256::from(gas_price);

    let mut s = RlpStream::new_list(9);
    s.append(&nonce_u64);
    s.append(&gp_u256);
    s.append(&gas_limit_u64);
    s.append(&to_h160);
    s.append(&value_u256);
    s.append(&data);
    s.append(&chain_id_u64);
    s.append(&0u8);
    s.append(&0u8);
    let signing_hash: [u8; 32] = Keccak256::digest(&s.out()).into();

    let (sig, rec_id) = sk.sign_prehash_recoverable(&signing_hash)
        .map_err(|e| JsError::new(&format!("sign: {e}")))?;

    let r_bytes = sig.r().to_bytes();
    let s_bytes = sig.s().to_bytes();
    let r = U256::from_big_endian(&r_bytes);
    let s_val = U256::from_big_endian(&s_bytes);
    let v = u64::from(rec_id.to_byte()) + chain_id_u64 * 2 + 35;

    let mut st = RlpStream::new_list(9);
    st.append(&nonce_u64);
    st.append(&gp_u256);
    st.append(&gas_limit_u64);
    st.append(&to_h160);
    st.append(&value_u256);
    st.append(&data);
    st.append(&v);
    st.append(&r);
    st.append(&s_val);
    let raw = st.out().to_vec();

    let tx_hash: [u8; 32] = Keccak256::digest(&raw).into();

    let info = SignedTxInfo {
        from: from_address,
        raw_hex: format!("0x{}", hex::encode(&raw)),
        tx_hash: format!("0x{}", hex::encode(&tx_hash)),
    };
    serde_wasm_bindgen::to_value(&info).map_err(|e| JsError::new(&e.to_string()))
}
#[wasm_bindgen]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
#[wasm_bindgen]
pub fn encode_timecapsule_create(recipient: &str, unlock_at: u64) -> Result<String, JsError> {
    let sel = selector("create(address,uint64)");
    let recip_clean = recipient.trim_start_matches("0x");
    let recip_bytes = hex::decode(recip_clean).map_err(|e| JsError::new(&e.to_string()))?;
    if recip_bytes.len() != 20 {
        return Err(JsError::new("recipient must be 20-byte address"));
    }
    let mut out = Vec::with_capacity(4 + 32 + 32);
    out.extend_from_slice(&sel);
    let mut addr_pad = [0u8; 32];
    addr_pad[12..].copy_from_slice(&recip_bytes);
    out.extend_from_slice(&addr_pad);
    let mut unlock_buf = [0u8; 32];
    unlock_buf[24..32].copy_from_slice(&unlock_at.to_be_bytes());
    out.extend_from_slice(&unlock_buf);
    Ok(format!("0x{}", hex::encode(&out)))
}
