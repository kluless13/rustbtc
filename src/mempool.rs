use std::collections::HashMap;
use crate::structures::Transaction;

pub struct Mempool {
    transactions: HashMap<[u8; 32], Transaction>,
}

impl Mempool {
    pub fn new() -> Self {
        Mempool {
            transactions: HashMap::new(),
        }
    }

    pub fn add_transaction(&mut self, transaction: Transaction) -> Result<(), &'static str> {
        let tx_hash = self.calculate_transaction_hash(&transaction);
        if self.transactions.contains_key(&tx_hash) {
            return Err("Transaction already in mempool");
        }
        self.transactions.insert(tx_hash, transaction);
        Ok(())
    }

    pub fn remove_transaction(&mut self, tx_hash: &[u8; 32]) -> Option<Transaction> {
        self.transactions.remove(tx_hash)
    }

    pub fn get_transactions(&self, limit: usize) -> Vec<Transaction> {
        self.transactions.values().take(limit).cloned().collect()
    }

    fn calculate_transaction_hash(&self, transaction: &Transaction) -> [u8; 32] {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(bincode::serialize(transaction).unwrap());
        hasher.finalize().into()
    }
}
