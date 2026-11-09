use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Состояние AIC Chain: аккаунты, их балансы и nonce.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct State {
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

    /// Корневой хеш состояния (упрощённо -- хеш сериализованной карты).
    pub fn root(&self) -> [u8; 32] {
        let json = serde_json::to_vec(&self.accounts).unwrap_or_default();
        use sha3::{Digest, Keccak256};
        Keccak256::digest(&json).into()
    }
}
