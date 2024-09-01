use crate::structures::{Block, Transaction, TransactionInput, TransactionOutput};
use crate::utxo::{UTXO, UTXOSet};
use crate::mempool::Mempool;
use sha2::{Sha256, Digest};
use std::time::Instant;

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub utxo_set: UTXOSet,
    pub mempool: Mempool,
}

impl Blockchain {
    pub fn new() -> Self {
        let mut blockchain = Blockchain {
            chain: vec![],
            utxo_set: UTXOSet::new(),
            mempool: Mempool::new(),
        };
        blockchain.create_genesis_block();
        blockchain
    }

    fn create_genesis_block(&mut self) {
        let genesis_transaction = Transaction {
            inputs: vec![],
            outputs: vec![TransactionOutput {
                value: 50_000_000_000, // 500 BTC in satoshis
                script_pubkey: b"Genesis Address".to_vec(),
            }],
        };
        let genesis_block = Block::new([0; 32], vec![genesis_transaction.clone()], 0);
        self.chain.push(genesis_block);

        let genesis_utxo = UTXO {
            txid: self.calculate_transaction_hash(&genesis_transaction),
            vout: 0,
            amount: 50_000_000_000,
            script_pubkey: b"Genesis Address".to_vec(),
        };
        self.utxo_set.add_utxo(genesis_utxo);
    }

    pub fn add_block(&mut self, block: Block) -> Result<(), &'static str> {
        if self.is_valid_new_block(&block) {
            println!("Adding new block to blockchain");
            self.update_utxo_set(&block);
            self.chain.push(block);
            Ok(())
        } else {
            Err("Invalid block")
        }
    }

    pub fn add_transaction(&mut self, transaction: &Transaction) -> Result<(), &'static str> {
        if self.is_valid_transaction(transaction) {
            self.mempool.add_transaction(transaction.clone())
        } else {
            Err("Invalid transaction")
        }
    }

    pub fn is_valid_transaction(&self, transaction: &Transaction) -> bool {
        let mut input_sum = 0;
        let output_sum: u64 = transaction.outputs.iter().map(|output| output.value).sum();

        for input in &transaction.inputs {
            if let Some(utxo) = self.utxo_set.get_utxo(&input.txid, input.vout) {
                input_sum += utxo.amount;
            } else {
                return false; // Input refers to a non-existent or already spent output
            }
        }

        input_sum >= output_sum
    }

    fn update_utxo_set(&mut self, block: &Block) {
        println!("Updating UTXO set for new block");
        for tx in &block.transactions {
            println!("Processing transaction: {:?}", self.calculate_transaction_hash(tx));
            for input in &tx.inputs {
                println!("Removing UTXO: {:?}:{}", input.txid, input.vout);
                self.utxo_set.remove_utxo(&input.txid, input.vout);
            }

            let txid = self.calculate_transaction_hash(tx);
            for (vout, output) in tx.outputs.iter().enumerate() {
                let utxo = UTXO {
                    txid,
                    vout: vout as u32,
                    amount: output.value,
                    script_pubkey: output.script_pubkey.clone(),
                };
                println!("Adding UTXO: {:?}:{} with amount {}", txid, vout, output.value);
                self.utxo_set.add_utxo(utxo);
            }
        }
    }

    fn is_valid_new_block(&self, block: &Block) -> bool {
        let previous_block = self.chain.last().unwrap();
        
        if block.header.prev_block_hash != self.calculate_block_hash(previous_block) {
            return false;
        }

        // Here you would typically also check:
        // 1. Block's hash meets the difficulty requirement
        // 2. Block's timestamp is valid
        // 3. All transactions in the block are valid
        
        true
    }

    pub fn calculate_block_hash(&self, block: &Block) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(block.serialize());
        hasher.finalize().into()
    }

    pub fn calculate_transaction_hash(&self, transaction: &Transaction) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(transaction.serialize());
        hasher.finalize().into()
    }

    pub fn mine_block(&mut self) -> Block {
        println!("Mining new block");
        let start_time = Instant::now();
        let prev_hash = self.calculate_block_hash(self.chain.last().unwrap());
        let transactions = self.mempool.get_transactions(10); // Get up to 10 transactions
        let difficulty = self.adjust_difficulty();
        let mut new_block = Block::new(prev_hash, transactions.clone(), difficulty);
        
        let mut attempts = 0;
        loop {
            let hash = self.calculate_block_hash(&new_block);
            if self.calculate_hash_meets_difficulty(&hash, difficulty) {
                break;
            }
            new_block.header.nonce += 1;
            attempts += 1;
        }

        let mining_time = start_time.elapsed();
        println!("New block mined with {} transactions", new_block.transactions.len());
        println!("Mining took {:?} and {} attempts", mining_time, attempts);
        println!("Mining speed: {:.2} hashes per second", attempts as f64 / mining_time.as_secs_f64());
        
        // Remove mined transactions from mempool
        for tx in &transactions {
            let tx_hash = self.calculate_transaction_hash(tx);
            self.mempool.remove_transaction(&tx_hash);
        }

        new_block
    }

    pub fn adjust_difficulty(&self) -> u32 {
        // For simplicity, we're using a fixed difficulty
        // In a real implementation, this would adjust based on mining times
        1
    }

    pub fn calculate_hash_meets_difficulty(&self, hash: &[u8; 32], difficulty: u32) -> bool {
        let target = u32::MAX >> difficulty;
        let value = u32::from_be_bytes([hash[0], hash[1], hash[2], hash[3]]);
        value < target
    }

    pub fn get_balance(&self, address: &str) -> u64 {
        self.utxo_set.get_balance(address)
    }
}
