mod structures;

use structures::{Block, Transaction, TransactionInput, TransactionOutput};

fn main() {
    // Create a sample transaction
    let tx = Transaction {
        inputs: vec![TransactionInput {
            txid: [0; 32],
            vout: 0,
            script_sig: vec![],
        }],
        outputs: vec![TransactionOutput {
            value: 50_000_000, // 0.5 BTC
            script_pubkey: vec![],
        }],
    };

    // Create a new block with the sample transaction
    let block = Block::new([0; 32], vec![tx]);

    println!("New block created: {:?}", block);
}
