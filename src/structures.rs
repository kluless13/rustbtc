use sha2::{Sha256, Digest};
use chrono::Utc;

// Transaction structure
#[derive(Debug, Clone)]
pub struct Transaction {
    pub inputs: Vec<TransactionInput>,
    pub outputs: Vec<TransactionOutput>,
}

#[derive(Debug, Clone)]
pub struct TransactionInput {
    pub txid: [u8; 32],
    pub vout: u32,
    pub script_sig: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct TransactionOutput {
    pub value: u64,
    pub script_pubkey: Vec<u8>,
}

// Block structure
#[derive(Debug, Clone)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
}

#[derive(Debug, Clone)]
pub struct BlockHeader {
    pub version: i32,
    pub prev_block_hash: [u8; 32],
    pub merkle_root: [u8; 32],
    pub timestamp: i64,
    pub bits: u32,
    pub nonce: u32,
}

impl Block {
    pub fn new(prev_block_hash: [u8; 32], transactions: Vec<Transaction>) -> Self {
        let merkle_root = Self::calculate_merkle_root(&transactions);
        
        Block {
            header: BlockHeader {
                version: 1,
                prev_block_hash,
                merkle_root,
                timestamp: Utc::now().timestamp(),
                bits: 0, // This should be calculated based on the current difficulty
                nonce: 0,
            },
            transactions,
        }
    }

    fn calculate_merkle_root(transactions: &[Transaction]) -> [u8; 32] {
        // This is a simplified merkle root calculation
        // In a real implementation, you'd need to handle the case of an odd number of transactions
        let mut hasher = Sha256::new();
        for tx in transactions {
            let tx_hash = Self::hash_transaction(tx);
            hasher.update(tx_hash);
        }
        hasher.finalize().into()
    }

    fn hash_transaction(transaction: &Transaction) -> [u8; 32] {
        // This is a simplified transaction hashing method
        // In a real implementation, you'd need to properly serialize the transaction
        let mut hasher = Sha256::new();
        for input in &transaction.inputs {
            hasher.update(&input.txid);
            hasher.update(&input.vout.to_le_bytes());
            hasher.update(&input.script_sig);
        }
        for output in &transaction.outputs {
            hasher.update(&output.value.to_le_bytes());
            hasher.update(&output.script_pubkey);
        }
        hasher.finalize().into()
    }
}
