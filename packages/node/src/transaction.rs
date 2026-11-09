use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};

/// Транзакция AIC Chain (упрощённая, legacy-стиль).
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
    /// Хеш подписи (EIP-155-подобный).
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

    /// Хеш транзакции (включает подпись).
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
