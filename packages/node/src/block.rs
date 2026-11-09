use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};

use crate::transaction::Transaction;

/// Блок AIC Chain. Содержит заголовок и список транзакций.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
}

/// Заголовок блока. Содержит всё, что нужно для проверки и связи с предыдущим блоком.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockHeader {
    pub parent_hash: [u8; 32],
    pub number: u64,
    pub timestamp: u64,
    pub state_root: [u8; 32],
    pub tx_root: [u8; 32],
    pub gas_used: u64,
    pub gas_limit: u64,
    pub proposer: [u8; 20],
    pub extra_data: Vec<u8>,
}

impl BlockHeader {
    /// Хеш заголовка: keccak256(rlp([parent_hash, number, timestamp, state_root, tx_root, gas_used, gas_limit, proposer, extra_data]))
    pub fn hash(&self) -> [u8; 32] {
        let mut stream = rlp::RlpStream::new_list(9);
        stream.append(&self.parent_hash.as_slice());
        stream.append(&self.number);
        stream.append(&self.timestamp);
        stream.append(&self.state_root.as_slice());
        stream.append(&self.tx_root.as_slice());
        stream.append(&self.gas_used);
        stream.append(&self.gas_limit);
        stream.append(&self.proposer.as_slice());
        stream.append(&self.extra_data);
        let encoded = stream.out();
        Keccak256::digest(&encoded).into()
    }
}

impl Block {
    pub fn hash(&self) -> [u8; 32] {
        self.header.hash()
    }

    pub fn number(&self) -> u64 {
        self.header.number
    }
}
