use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};

fn parse_addr(s: &str) -> Option<[u8; 20]> {
    let b = hex::decode(s.trim_start_matches("0x")).ok()?;
    if b.len() != 20 { return None; }
    let mut a = [0u8; 20]; a.copy_from_slice(&b); Some(a)
}

fn parse_u128(s: &str) -> Option<u128> {
    if let Some(h) = s.strip_prefix("0x") { u128::from_str_radix(h, 16).ok() }
    else { s.parse().ok() }
}

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
    pub signature: Vec<u8>,
}

impl Transaction {
    pub fn from_json(v: &serde_json::Value) -> Option<Self> {
        let nonce = v.get("nonce")?.as_u64()?;
        let from = parse_addr(v.get("from")?.as_str()?)?;
        let to = v.get("to").and_then(|t| t.as_str()).and_then(parse_addr);
        let value = parse_u128(v.get("value")?.as_str()?)?;
        let gas_price = parse_u128(v.get("gas_price")?.as_str()?)?;
        let gas_limit = v.get("gas_limit")?.as_u64()?;
        let data = hex::decode(v.get("data")?.as_str()?.trim_start_matches("0x")).ok()?;
        let chain_id = v.get("chain_id")?.as_u64()?;
        let signature = hex::decode(v.get("signature")?.as_str()?.trim_start_matches("0x")).ok()?;
        Some(Transaction { nonce, from, to, value, gas_price, gas_limit, data, chain_id, signature })
    }

    pub fn signing_hash(&self) -> [u8; 32] {
        let mut stream = rlp::RlpStream::new_list(9);
        stream.append(&self.nonce);
        stream.append(&self.from.as_slice());
        stream.append(&self.to.map(|t| t.to_vec()));
        stream.append(&self.value.to_be_bytes().as_slice());
        stream.append(&self.gas_price.to_be_bytes().as_slice());
        stream.append(&self.gas_limit);
        stream.append(&self.data);
        stream.append(&self.chain_id);
        stream.append(&0u8);
        stream.append(&0u8);
        let encoded = stream.out();
        Keccak256::digest(&encoded).into()
    }

    pub fn hash(&self) -> [u8; 32] {
        let mut stream = rlp::RlpStream::new_list(10);
        stream.append(&self.nonce);
        stream.append(&self.from.as_slice());
        stream.append(&self.to.map(|t| t.to_vec()));
        stream.append(&self.value.to_be_bytes().as_slice());
        stream.append(&self.gas_price.to_be_bytes().as_slice());
        stream.append(&self.gas_limit);
        stream.append(&self.data);
        stream.append(&self.chain_id);
        stream.append(&self.signature.as_slice());
        let encoded = stream.out();
        Keccak256::digest(&encoded).into()
    }
}
