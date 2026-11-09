use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::block::{Block, BlockHeader};
use crate::state::State;
use crate::transaction::Transaction;

pub const GENESIS_TIMESTAMP: u64 = 1_766_000_000;
pub const DEFAULT_BLOCK_TIME_SECS: u64 = 3;
pub const BLOCK_GAS_LIMIT: u64 = 30_000_000;
pub const BASE_TX_GAS: u64 = 21_000;

/// Квитанция о транзакции.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxReceipt {
    pub tx_hash: [u8; 32],
    pub block_number: u64,
    pub success: bool,
    pub gas_used: u64,
    pub from: [u8; 20],
    pub to: Option<[u8; 20]>,
    pub value: u128,
}

/// Вся цепочка AIC: блоки, состояние, квитанции.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chain {
    pub chain_id: u64,
    pub blocks: Vec<Block>,
    pub state: State,
    #[serde(with = "crate::serde_helpers::map_bytes32")]
    pub receipts: HashMap<[u8; 32], TxReceipt>,
}

impl Chain {
    /// Создать genesis с аллокацией (адрес, баланс в wei) + деплой контрактов.
    pub fn genesis(chain_id: u64, alloc: &[([u8; 20], u128)]) -> Self {
        let mut state = State::new();
        for (addr, balance) in alloc {
            state.credit(*addr, *balance);
        }

        // Деплой Solidity-контрактов в genesis (если артефакты Foundry есть)
        let deployed = crate::genesis_contracts::deploy_genesis_contracts(&mut state);
        if !deployed.is_empty() {
            println!("--- Genesis contracts deployed ---");
            for (name, addr, size) in &deployed {
                println!("  {} => 0x{} ({} bytes)", name, hex::encode(addr), size);
            }
            println!("----------------------------------");
        } else {
            println!("--- No contracts deployed (Foundry artifacts not found) ---");
        }

        let header = BlockHeader {
            parent_hash: [0u8; 32],
            number: 0,
            timestamp: GENESIS_TIMESTAMP,
            state_root: state.root(),
            tx_root: [0u8; 32],
            gas_used: 0,
            gas_limit: BLOCK_GAS_LIMIT,
            proposer: alloc.first().map(|(a, _)| *a).unwrap_or([0u8; 20]),
            extra_data: b"AIC Chain genesis".to_vec(),
        };

        Self {
            chain_id,
            blocks: vec![Block { header, transactions: vec![] }],
            state,
            receipts: HashMap::new(),
        }
    }

    pub fn head(&self) -> &Block {
        self.blocks.last().expect("chain has genesis")
    }

    pub fn block_number(&self) -> u64 {
        self.head().number()
    }

    /// Произвести новый блок: применить txs, обновить state, добавить в chain.
    pub fn produce_block(&mut self, txs: Vec<Transaction>, timestamp: u64) -> Block {
        let mut gas_used = 0u64;
        let mut applied = Vec::new();

        for tx in txs {
            if gas_used + BASE_TX_GAS > BLOCK_GAS_LIMIT { break; }

            if !tx.verify() {
                // Невалидная подпись -- пропускаем
                continue;
            }

            let tx_hash = tx.hash();
            let mut from_acc = self.state.get_account(&tx.from);

            // Проверка nonce и баланса
            let total_cost = tx.value.saturating_add(tx.gas_price.saturating_mul(BASE_TX_GAS as u128));
            let valid = tx.nonce == from_acc.nonce && from_acc.balance >= total_cost;

            if valid {
                from_acc.balance = from_acc.balance.saturating_sub(total_cost);
                from_acc.nonce = from_acc.nonce.saturating_add(1);
                self.state.set_account(tx.from, from_acc);

                if let Some(to) = tx.to {
                    self.state.credit(to, tx.value);
                }

                // Газ: 50% burn, 50% proposer (foundation)
                // Для v0.1 -- просто burn, ничего никому.
                gas_used += BASE_TX_GAS;

                self.receipts.insert(tx_hash, TxReceipt {
                    tx_hash,
                    block_number: self.block_number() + 1,
                    success: true,
                    gas_used: BASE_TX_GAS,
                    from: tx.from,
                    to: tx.to,
                    value: tx.value,
                });
                applied.push(tx);
            } else {
                self.receipts.insert(tx_hash, TxReceipt {
                    tx_hash,
                    block_number: self.block_number() + 1,
                    success: false,
                    gas_used: 0,
                    from: tx.from,
                    to: tx.to,
                    value: tx.value,
                });
            }
        }

        let parent_hash = self.head().hash();
        let number = self.block_number() + 1;
        let state_root = self.state.root();

        // tx_root = keccak256(concat(tx hashes)) или zero для пустого
        let tx_root = if applied.is_empty() {
            [0u8; 32]
        } else {
            use sha3::{Digest, Keccak256};
            let mut buf = Vec::new();
            for tx in &applied { buf.extend_from_slice(&tx.hash()); }
            Keccak256::digest(&buf).into()
        };

        let header = BlockHeader {
            parent_hash,
            number,
            timestamp,
            state_root,
            tx_root,
            gas_used,
            gas_limit: BLOCK_GAS_LIMIT,
            proposer: [0xF0; 20],
            extra_data: vec![],
        };

        let block = Block { header, transactions: applied };
        self.blocks.push(block.clone());
        block
    }

    /// Сохранить всю цепочку в JSON-файл.
    pub fn save(&self, path: &std::path::Path) -> std::io::Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("serialize: {e}")))?;
        std::fs::write(path, json)
    }

    /// Загрузить цепочку из JSON-файла.
    pub fn load(path: &std::path::Path) -> std::io::Result<Self> {
        let text = std::fs::read_to_string(path)?;
        let chain: Chain = serde_json::from_str(&text)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("deserialize: {e}")))?;
        Ok(chain)
    }
}

