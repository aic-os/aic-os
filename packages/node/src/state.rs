use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct State {
    #[serde(with = "crate::serde_helpers::map_bytes20")]
    accounts: HashMap<[u8; 20], Account>,
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

    pub fn root(&self) -> [u8; 32] {
        // Детерминированная сортировка для стабильного хеша
        let mut items: Vec<(&[u8; 20], &Account)> = self.accounts.iter().collect();
        items.sort_by_key(|(k, _)| **k);
        let mut buf = Vec::new();
        for (addr, acc) in items {
            buf.extend_from_slice(addr.as_slice());
            buf.extend_from_slice(&acc.balance.to_be_bytes());
            buf.extend_from_slice(&acc.nonce.to_be_bytes());
        }
        use sha3::{Digest, Keccak256};
        Keccak256::digest(&buf).into()
    }
}

