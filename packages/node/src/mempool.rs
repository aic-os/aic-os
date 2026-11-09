use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::transaction::Transaction;

/// Мемпул -- хранит неподтверждённые транзакции.
#[derive(Debug, Clone, Default)]
pub struct Mempool {
    inner: Arc<RwLock<HashMap<[u8; 32], Transaction>>>,
}

impl Mempool {
    pub fn new() -> Self { Self::default() }

    pub async fn add(&self, tx: Transaction) -> [u8; 32] {
        let hash = tx.hash();
        let mut m = self.inner.write().await;
        m.insert(hash, tx);
        hash
    }

    pub async fn contains(&self, hash: &[u8; 32]) -> bool {
        self.inner.read().await.contains_key(hash)
    }

    pub async fn len(&self) -> usize {
        self.inner.read().await.len()
    }

    pub async fn drain(&self, max: usize) -> Vec<Transaction> {
        let mut m = self.inner.write().await;
        let keys: Vec<[u8; 32]> = m.keys().take(max).copied().collect();
        let mut out = Vec::with_capacity(keys.len());
        for k in keys {
            if let Some(tx) = m.remove(&k) {
                out.push(tx);
            }
        }
        out
    }
}
