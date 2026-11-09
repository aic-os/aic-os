use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};

use revm::{
    context::{Context, TxEnv},
    context_interface::result::ExecutionResult,
    primitives::{Address, Bytes, TxKind, U256},
    ExecuteEvm, MainBuilder, MainContext,
};

use crate::block::{Block, BlockHeader};
use crate::state::State;
use crate::transaction::Transaction;

pub const GENESIS_TIMESTAMP: u64 = 1_766_000_000;
pub const DEFAULT_BLOCK_TIME_SECS: u64 = 3;
pub const BLOCK_GAS_LIMIT: u64 = 30_000_000;
pub const BASE_TX_GAS: u64 = 21_000;

pub const TREASURY_ADDR: [u8; 20] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0x0F];

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chain {
    pub chain_id: u64,
    pub blocks: Vec<Block>,
    pub state: State,
    #[serde(with = "crate::serde_helpers::map_bytes32")]
    pub receipts: HashMap<[u8; 32], TxReceipt>,
}

impl Chain {
    pub fn genesis(chain_id: u64, alloc: &[([u8; 20], u128)]) -> Self {
        let mut state = State::new();
        for (addr, balance) in alloc {
            state.credit(*addr, *balance);
        }

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

    /// Выполнить транзакцию. Если to -- контракт или data не пустая, исполняем через EVM.
    /// Иначе -- обычный перевод AIC.
    fn apply_tx(&mut self, tx: &Transaction, gas_used: &mut u64) -> (bool, u64) {
        let is_evm = tx.data.len() > 0
            || tx.to.map(|t| self.state.is_contract(&t)).unwrap_or(false);

        if is_evm {
            self.apply_evm_tx(tx, gas_used)
        } else {
            self.apply_simple_tx(tx, gas_used)
        }
    }

    fn apply_simple_tx(&mut self, tx: &Transaction, gas_used: &mut u64) -> (bool, u64) {
        let mut from_acc = self.state.get_account(&tx.from);
        let gas_fee = tx.gas_price.saturating_mul(BASE_TX_GAS as u128);
        let total_cost = tx.value.saturating_add(gas_fee);
        let valid = tx.nonce == from_acc.nonce && from_acc.balance >= total_cost;

        if valid {
            from_acc.balance = from_acc.balance.saturating_sub(total_cost);
            from_acc.nonce = from_acc.nonce.saturating_add(1);
            self.state.set_account(tx.from, from_acc);

            if let Some(to) = tx.to {
                self.state.credit(to, tx.value);
            }

            // Газ: 100% в казну (пока)
            self.state.credit(TREASURY_ADDR, gas_fee);

            *gas_used += BASE_TX_GAS;
            (true, BASE_TX_GAS)
        } else {
            (false, 0)
        }
    }

    fn apply_evm_tx(&mut self, tx: &Transaction, gas_used: &mut u64) -> (bool, u64) {
        let from_acc = self.state.get_account(&tx.from);
        eprintln!("[apply_evm_tx] from=0x{} acc.nonce={} tx.nonce={} acc.balance={}",
            hex::encode(&tx.from), from_acc.nonce, tx.nonce, from_acc.balance);
        if tx.nonce != from_acc.nonce {
            eprintln!("[apply_evm_tx] NONCE MISMATCH, reject");
            return (false, 0);
        }

        let gas_price = if tx.gas_price > 0 { tx.gas_price } else { 1_000_000_000 };
        let max_gas_cost = gas_price.saturating_mul(tx.gas_limit as u128);
        eprintln!("[apply_evm_tx] balance check: have={} need={}", from_acc.balance, tx.value.saturating_add(max_gas_cost));
        if from_acc.balance < tx.value.saturating_add(max_gas_cost) {
            eprintln!("[apply_evm_tx] BALANCE TOO LOW, reject");
            return (false, 0);
        }

        let db = self.state.to_inmemory();
        // КРИТИЧНО: наш chain_id = 2026, а не 1 (Ethereum mainnet).
        // В revm 43 CfgEnv generic по SpecId -- указываем тип явно.
        let mut cfg: revm::context::CfgEnv<revm::primitives::hardfork::SpecId> = Default::default();
        cfg.chain_id = self.chain_id;
        let mut evm = Context::mainnet().with_db(db).with_cfg(cfg).build_mainnet();

        let to_addr = match tx.to {
            Some(t) => Address::from(t),
            None => return (false, 0),
        };

        let mut tx_env = TxEnv::default();
        tx_env.caller = Address::from(tx.from);
        tx_env.gas_limit = tx.gas_limit;
        tx_env.gas_price = gas_price;
        tx_env.kind = TxKind::Call(to_addr);
        tx_env.value = U256::from(tx.value);
        tx_env.data = Bytes::from(tx.data.clone());
        tx_env.nonce = tx.nonce;
        tx_env.chain_id = Some(self.chain_id);

        let result = match evm.transact(tx_env) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("[apply_evm_tx] EVM ERROR: {:?}", e);
                return (false, 0);
            }
        };
        eprintln!("[apply_evm_tx] EVM result: {:?}, gas={}", result.result, result.result.gas_used());

        let actual_gas = result.result.gas_used();
        let success = result.result.is_success();

        // Применяем state diff (все аккаунты, которые revm изменил)
        for (addr, acc) in result.state {
            let addr_arr: [u8; 20] = addr.into();

            // Баланс и nonce
            let mut our_acc = self.state.get_account(&addr_arr);
            let new_balance = u128::try_from(acc.info.balance).unwrap_or(u128::MAX);
            let new_nonce = acc.info.nonce;
            our_acc.balance = new_balance;
            our_acc.nonce = new_nonce;
            self.state.set_account(addr_arr, our_acc);

            // Код
            if let Some(code) = &acc.info.code {
                if !code.is_empty() {
                    self.state.set_code(addr_arr, code.original_bytes().to_vec());
                }
            }

            // Storage: EvmStorageSlot.present_value -> [u8; 32]
            for (slot_u256, slot_val) in acc.storage.iter() {
                let slot_arr: [u8; 32] = slot_u256.to_be_bytes();
                let val_arr: [u8; 32] = slot_val.present_value.to_be_bytes();
                self.state.set_storage(addr_arr, slot_arr, val_arr);
            }
        }

        // Комиссия: revm уже списал с from. Добавляем газ в казну.
        // Для MVP: простая модель -- revm списал max_gas_cost, возвращаем разницу.
        // gas_used * gas_price -- реальная комиссия, уходит в казну.
        let gas_fee = gas_price.saturating_mul(actual_gas as u128);
        self.state.credit(TREASURY_ADDR, gas_fee);

        *gas_used += actual_gas;
        (success, actual_gas)
    }

    pub fn produce_block(&mut self, txs: Vec<Transaction>, timestamp: u64) -> Block {
        let mut gas_used = 0u64;
        let mut applied = Vec::new();
        if !txs.is_empty() {
            eprintln!("[produce_block] received {} tx from mempool", txs.len());
        }for tx in txs {
            if gas_used + 21_000 > BLOCK_GAS_LIMIT { break; }

            let tx_hash = tx.hash();

            if !tx.verify() {
                continue;
            }

            let bn = self.block_number() + 1;
            let (success, tx_gas) = self.apply_tx(&tx, &mut gas_used);

            self.receipts.insert(tx_hash, TxReceipt {
                tx_hash,
                block_number: bn,
                success,
                gas_used: tx_gas,
                from: tx.from,
                to: tx.to,
                value: tx.value,
            });

            if success {
                applied.push(tx);
            }
        }

        let parent_hash = self.head().hash();
        let number = self.block_number() + 1;
        let state_root = self.state.root();

        let tx_root = if applied.is_empty() {
            [0u8; 32]
        } else {
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

    pub fn save(&self, path: &std::path::Path) -> std::io::Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("serialize: {e}")))?;
        std::fs::write(path, json)
    }

    pub fn load(path: &std::path::Path) -> std::io::Result<Self> {
        let text = std::fs::read_to_string(path)?;
        let chain: Chain = serde_json::from_str(&text)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("deserialize: {e}")))?;
        Ok(chain)
    }
}




