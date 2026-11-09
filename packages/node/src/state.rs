use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct State {
    #[serde(with = "crate::serde_helpers::map_bytes20")]
    accounts: HashMap<[u8; 20], Account>,
    /// Байткод контрактов: адрес -> bytecode
    #[serde(with = "crate::serde_helpers::map_bytes20", default)]
    contracts: HashMap<[u8; 20], Vec<u8>>,
    /// Storage контрактов: адрес -> (slot -> value)
    #[serde(with = "crate::serde_helpers::map_storage", default, skip_serializing_if = "HashMap::is_empty")]
    storage: HashMap<[u8; 20], HashMap<[u8; 32], [u8; 32]>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Account {
    pub balance: u128,
    pub nonce: u64,
}

impl State {
    pub fn new() -> Self { Self::default() }

    pub fn get_account(&self, addr: &[u8; 20]) -> Account {
        self.accounts.get(addr).cloned().unwrap_or_default()
    }

    pub fn set_account(&mut self, addr: [u8; 20], acc: Account) {
        self.accounts.insert(addr, acc);
    }

    pub fn credit(&mut self, addr: [u8; 20], amount: u128) {
        let mut acc = self.get_account(&addr);
        acc.balance = acc.balance.saturating_add(amount);
        self.set_account(addr, acc);
    }

    pub fn debit(&mut self, addr: [u8; 20], amount: u128) -> bool {
        let mut acc = self.get_account(&addr);
        if acc.balance < amount { return false; }
        acc.balance -= amount;
        self.set_account(addr, acc);
        true
    }

    // ===== Контракты =====

    pub fn set_code(&mut self, addr: [u8; 20], code: Vec<u8>) {
        if !code.is_empty() {
            self.contracts.insert(addr, code);
        }
    }

    pub fn get_code(&self, addr: &[u8; 20]) -> Option<&Vec<u8>> {
        self.contracts.get(addr)
    }

    pub fn is_contract(&self, addr: &[u8; 20]) -> bool {
        self.contracts.get(addr).map(|c| !c.is_empty()).unwrap_or(false)
    }

    pub fn set_storage(&mut self, addr: [u8; 20], slot: [u8; 32], value: [u8; 32]) {
        self.storage.entry(addr).or_default().insert(slot, value);
    }

    pub fn get_storage(&self, addr: &[u8; 20], slot: &[u8; 32]) -> [u8; 32] {
        self.storage.get(addr).and_then(|m| m.get(slot)).copied().unwrap_or([0u8; 32])
    }

    // ===== Конвертация в revm::InMemoryDB =====

    pub fn to_inmemory(&self) -> revm::database::InMemoryDB {
        use revm::state::{AccountInfo, Bytecode};
        use revm::primitives::{Address, U256, B256};
        use revm::database::InMemoryDB;
        use revm::DatabaseRef;

        let mut db = InMemoryDB::default();
        for (addr_arr, acc) in &self.accounts {
            let addr = Address::from_slice(addr_arr);
            let mut info = AccountInfo::default();
            info.balance = U256::from(acc.balance);
            info.nonce = acc.nonce;

            if let Some(code_bytes) = self.contracts.get(addr_arr) {
                if !code_bytes.is_empty() {
                    let bytecode = Bytecode::new_raw(code_bytes.clone().into());
                    info.code_hash = bytecode.hash_slow();
                    info.code = Some(bytecode);
                }
            }
            db.insert_account_info(addr, info);

            if let Some(storage_map) = self.storage.get(addr_arr) {
                for (slot, value) in storage_map {
                    let k = U256::from_be_bytes(*slot);
                    let v = U256::from_be_bytes(*value);
                    let _ = db.insert_account_storage(addr, k, v);
                }
            }
        }
        // Важно: пустые аккаунты, у которых есть только код (контракты без баланса), тоже нужны.
        for (addr_arr, code_bytes) in &self.contracts {
            if !self.accounts.contains_key(addr_arr) && !code_bytes.is_empty() {
                let addr = Address::from_slice(addr_arr);
                let bytecode = Bytecode::new_raw(code_bytes.clone().into());
                let info = AccountInfo {
                    balance: U256::ZERO,
                    nonce: 0,
                    code_hash: bytecode.hash_slow(),
                    code: Some(bytecode),
                    ..Default::default()
                };
                db.insert_account_info(addr, info);
                if let Some(storage_map) = self.storage.get(addr_arr) {
                    for (slot, value) in storage_map {
                        let k = U256::from_be_bytes(*slot);
                        let v = U256::from_be_bytes(*value);
                        let _ = db.insert_account_storage(addr, k, v);
                    }
                }
            }
        }
        let _ = B256::ZERO; // keep import
        let _ = db.basic_ref(Address::ZERO); // keep trait import used
        db
    }

    pub fn root(&self) -> [u8; 32] {
        let mut items: Vec<(&[u8; 20], &Account)> = self.accounts.iter().collect();
        items.sort_by_key(|(k, _)| **k);
        let mut buf = Vec::new();
        for (addr, acc) in items {
            buf.extend_from_slice(addr.as_slice());
            buf.extend_from_slice(&acc.balance.to_be_bytes());
            buf.extend_from_slice(&acc.nonce.to_be_bytes());
        }
        // Код контрактов и storage тоже влияют на root
        let mut contracts_vec: Vec<(&[u8; 20], &Vec<u8>)> = self.contracts.iter().collect();
        contracts_vec.sort_by_key(|(k, _)| **k);
        for (addr, code) in contracts_vec {
            buf.extend_from_slice(addr.as_slice());
            buf.extend_from_slice(code);
        }
        use sha3::{Digest, Keccak256};
        Keccak256::digest(&buf).into()
    }
}



