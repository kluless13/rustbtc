use crate::structures::{Block, Transaction};
use sha2::{Sha256, Digest};

pub struct Blockchain {
    chain: Vec<Block>,
    current_transactions: Vec<Transaction>,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis_block = Block::new([0; 32], vec![]);
        Blockchain {
            chain: vec![genesis_block],
            current_transactions: vec![],
        }
    }

    pub fn add_block(&mut self, block: Block) -> Result<(), &'static str> {
        if self.is_valid_new_block(&block) {
            self.chain.push(block);
            Ok(())
        } else {
            Err("Invalid block")
        }
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.current_transactions.push(transaction);
    }

    fn is_valid_new_block(&self, block: &Block) -> bool {
        let previous_block = self.chain.last().unwrap();
        
        // Check if the previous block hash matches
        if block.header.prev_block_hash != self.calculate_block_hash(previous_block) {
            return false;
        }

        // Here you would typically also check:
        // 1. Block's hash meets the difficulty requirement
        // 2. Block's timestamp is valid
        // 3. All transactions in the block are valid
        
        true
    }

    fn calculate_block_hash(&self, block: &Block) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(block.serialize());
        hasher.finalize().into()
    }

    pub fn mine_block(&mut self) -> Block {
        let prev_hash = self.calculate_block_hash(self.chain.last().unwrap());
        let mut new_block = Block::new(prev_hash, self.current_transactions.clone());
        
        // Simple mining: increment nonce until hash starts with "00"
        loop {
            let hash = self.calculate_block_hash(&new_block);
            if hash[0] == 0 && hash[1] == 0 {
                break;
            }
            new_block.header.nonce += 1;
        }

        self.current_transactions.clear();
        new_block
    }
    
    pub fn get_chain_length(&self) -> usize {
        self.chain.len()
    }

    pub fn get_last_block(&self) -> Option<&Block> {
        self.chain.last()
    }
}
