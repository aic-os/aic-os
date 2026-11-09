use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};
use k256::ecdsa::{RecoveryId, Signature, VerifyingKey};

use aic_core::crypto::pq_crypto;

fn be_bytes_to_u128(b: &[u8]) -> u128 {
    if b.len() > 16 { return 0; }
    let mut buf = [0u8; 16];
    buf[16 - b.len()..].copy_from_slice(b);
    u128::from_be_bytes(buf)
}

/// Порог: транзакции дороже 1000 AIC требуют PQ-подписи.
pub const PQ_REQUIRED_THRESHOLD: u128 = 1_000_000_000_000_000_000_000; // 1000 AIC

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub nonce: u64,
    pub from: [u8; 20],
    pub to: Option<[u8; 20]>,
    pub value: u128,
    pub gas_price: u128,
    pub gas_limit: u64,
    pub data: Vec<u8>,
    pub chain_id: u64,
    #[serde(default)]
    pub signature: Vec<u8>,
    // PQ (Dilithium) -- для крупных транзакций
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pq_pubkey: Option<Vec<u8>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pq_signature: Option<Vec<u8>>,
}

impl Transaction {
    /// RLP-парсинг legacy tx. PQ-поля = None (заполняются отдельно из RPC).
    pub fn from_raw_hex(hex_str: &str) -> Option<Self> {
        let bytes = hex::decode(hex_str.trim_start_matches("0x")).ok()?;
        let rlp = rlp::Rlp::new(&bytes);
        if rlp.item_count().ok()? != 9 { return None; }

        let nonce: u64 = rlp.val_at(0).ok()?;
        let gas_price_b: Vec<u8> = rlp.val_at(1).ok()?;
        let gas_price = be_bytes_to_u128(&gas_price_b);
        let gas_limit: u64 = rlp.val_at(2).ok()?;
        let to_b: Vec<u8> = rlp.val_at(3).ok()?;
        let to = if to_b.is_empty() { None }
                 else if to_b.len() == 20 { let mut a=[0u8;20]; a.copy_from_slice(&to_b); Some(a) }
                 else { return None; };
        let value_b: Vec<u8> = rlp.val_at(4).ok()?;
        let value = be_bytes_to_u128(&value_b);
        let data: Vec<u8> = rlp.val_at(5).ok()?;
        let v_b: Vec<u8> = rlp.val_at(6).ok()?;
        let v = be_bytes_to_u128(&v_b);
        let r_b: Vec<u8> = rlp.val_at(7).ok()?;
        let s_b: Vec<u8> = rlp.val_at(8).ok()?;

        if v < 35 { return None; }
        let rec_id = ((v - 35) % 2) as u8;
        let chain_id = ((v - 35 - rec_id as u128) / 2) as u64;

        if r_b.len() > 32 || s_b.len() > 32 { return None; }
        let mut r = [0u8; 32]; r[32 - r_b.len()..].copy_from_slice(&r_b);
        let mut s = [0u8; 32]; s[32 - s_b.len()..].copy_from_slice(&s_b);

        let mut signature = Vec::with_capacity(65);
        signature.extend_from_slice(&r);
        signature.extend_from_slice(&s);
        signature.push(rec_id);

        let tmp = Transaction {
            nonce, from: [0u8; 20], to, value, gas_price, gas_limit, data, chain_id, signature,
            pq_pubkey: None, pq_signature: None,
        };
        let hash = tmp.signing_hash();
        let from = recover_from(&hash, &tmp.signature)?;
        Some(Transaction { from, ..tmp })
    }

    pub fn signing_hash(&self) -> [u8; 32] {
        let mut stream = rlp::RlpStream::new_list(9);
        stream.append(&self.nonce);
        let gp = self.gas_price.to_be_bytes();
        let gp_trim = gp.iter().position(|&b| b != 0).map(|i| &gp[i..]).unwrap_or(&[]);
        stream.append(&gp_trim);
        stream.append(&self.gas_limit);
        match self.to { Some(t) => { stream.append(&t.as_slice()); }, None => { stream.append_empty_data(); } }
        let v = self.value.to_be_bytes();
        let v_trim = v.iter().position(|&b| b != 0).map(|i| &v[i..]).unwrap_or(&[]);
        stream.append(&v_trim);
        stream.append(&self.data);
        stream.append(&self.chain_id);
        stream.append(&0u8);
        stream.append(&0u8);
        Keccak256::digest(&stream.out()).into()
    }

    pub fn hash(&self) -> [u8; 32] {
        let mut buf = self.signing_hash().to_vec();
        buf.extend_from_slice(&self.signature);
        if let Some(pk) = &self.pq_pubkey { buf.extend_from_slice(pk); }
        if let Some(sig) = &self.pq_signature { buf.extend_from_slice(sig); }
        Keccak256::digest(&buf).into()
    }

    pub fn verify(&self) -> bool {
        let hash = self.signing_hash();
        recover_from(&hash, &self.signature).map(|a| a == self.from).unwrap_or(false)
    }

    /// PQ-сообщение: keccak256(rlp([b"AIC-PQ-v1", chain_id, nonce, from, to, value, data]))
    pub fn pq_message(&self) -> [u8; 32] {
        let mut stream = rlp::RlpStream::new_list(7);
        stream.append(&b"AIC-PQ-v1".as_slice());
        stream.append(&self.chain_id);
        stream.append(&self.nonce);
        stream.append(&self.from.as_slice());
        match self.to { Some(t) => { stream.append(&t.as_slice()); }, None => { stream.append_empty_data(); } }
        let v = self.value.to_be_bytes();
        let v_trim = v.iter().position(|&b| b != 0).map(|i| &v[i..]).unwrap_or(&[]);
        stream.append(&v_trim);
        stream.append(&self.data);
        Keccak256::digest(&stream.out()).into()
    }

    /// Проверка PQ-подписи. Возвращает true, если:
    ///   - PQ нет и value < threshold; или
    ///   - PQ нет и value >= threshold -- ВЕРНЁТ false (требуется PQ, но нет).
    ///   - PQ есть -- проверяем Dilithium; если невалидна -- false.
    pub fn verify_pq(&self) -> bool {
        let needs_pq = self.value >= PQ_REQUIRED_THRESHOLD;
        match (&self.pq_pubkey, &self.pq_signature) {
            (Some(pk), Some(sig)) => {
                let msg = self.pq_message();
                pq_crypto::verify_dilithium_only(&msg, sig, pk).unwrap_or(false)
            }
            (None, None) => !needs_pq,  // ок только если не крупная
            _ => false,  // один есть, другой нет -- невалидно
        }
    }
}

fn recover_from(signing_hash: &[u8; 32], signature_65: &[u8]) -> Option<[u8; 20]> {
    if signature_65.len() != 65 { return None; }
    let mut sig_bytes = [0u8; 64];
    sig_bytes.copy_from_slice(&signature_65[..64]);
    let sig = Signature::from_slice(&sig_bytes).ok()?;
    let rec_id = RecoveryId::from_byte(signature_65[64])?;
    let vk = VerifyingKey::recover_from_prehash(signing_hash, &sig, rec_id).ok()?;
    let point = vk.to_encoded_point(false);
    let uncompressed = &point.as_bytes()[1..];
    let h = Keccak256::digest(uncompressed);
    let mut addr = [0u8; 20];
    addr.copy_from_slice(&h[12..]);
    Some(addr)
}

