use serde::{Serialize, Deserialize};
use chrono::Utc;
use crate::merkle_tree::MerkleTree;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub inputs: Vec<TransactionInput>,
    pub outputs: Vec<TransactionOutput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionInput {
    pub txid: [u8; 32],
    pub vout: u32,
    pub script_sig: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionOutput {
    pub value: u64,
    pub script_pubkey: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockHeader {
    pub version: i32,
    pub prev_block_hash: [u8; 32],
    pub merkle_root: [u8; 32],
    pub timestamp: i64,
    pub difficulty: u32,
    pub nonce: u32,
}

impl Block {
    pub fn new(prev_block_hash: [u8; 32], transactions: Vec<Transaction>, difficulty: u32) -> Self {
        let transaction_hashes: Vec<String> = transactions
            .iter()
            .map(|tx| hex::encode(calculate_hash(tx)))
            .collect();

        let merkle_tree = MerkleTree::new(&transaction_hashes);
        let merkle_root = hex::decode(merkle_tree.root).unwrap();

        Block {
            header: BlockHeader {
                version: 1,
                prev_block_hash,
                merkle_root: merkle_root.try_into().unwrap(),
                timestamp: Utc::now().timestamp(),
                difficulty,
                nonce: 0,
            },
            transactions,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }

    pub fn deserialize(data: &[u8]) -> Result<Self, bincode::Error> {
        bincode::deserialize(data)
    }
}

impl Transaction {
    pub fn serialize(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }

    pub fn deserialize(data: &[u8]) -> Result<Self, bincode::Error> {
        bincode::deserialize(data)
    }
}

pub fn calculate_hash<T: Serialize>(item: &T) -> Vec<u8> {
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(bincode::serialize(item).unwrap());
    hasher.finalize().to_vec()
}
