mod structures;
mod blockchain;
mod wallet;

use structures::{Transaction, TransactionInput, TransactionOutput, Block};
use blockchain::Blockchain;
use wallet::Wallet;
use sha2::{Sha256, Digest};

fn main() {
    let mut blockchain = Blockchain::new();
    let alice_wallet = Wallet::new();
    let bob_wallet = Wallet::new();

    println!("Alice's wallet address: {}", alice_wallet.get_address());
    println!("Bob's wallet address: {}", bob_wallet.get_address());

    // Create a sample transaction from Alice to Bob
    let tx = Transaction {
        inputs: vec![TransactionInput {
            txid: [0; 32],
            vout: 0,
            script_sig: vec![],
        }],
        outputs: vec![TransactionOutput {
            value: 50_000_000, // 0.5 BTC
            script_pubkey: bob_wallet.get_address().into_bytes(),
        }],
    };

    // Sign the transaction
    let tx_hash = calculate_transaction_hash(&tx);
    let signature = alice_wallet.sign_transaction(&tx_hash);
    let mut signed_tx = tx.clone();
    signed_tx.inputs[0].script_sig = signature;

    // Add transaction to the blockchain
    blockchain.add_transaction(signed_tx.clone());

    // Mine a new block
    let new_block = blockchain.mine_block();
    println!("Mined a new block with nonce: {}", new_block.header.nonce);

    // Add the mined block to the blockchain
    match blockchain.add_block(new_block) {
        Ok(_) => println!("New block added successfully"),
        Err(e) => println!("Failed to add block: {}", e),
    }

    println!("Blockchain created with {} blocks", blockchain.get_chain_length());

    // Demonstrate block serialization and deserialization
    if let Some(last_block) = blockchain.get_last_block() {
        let serialized_block = last_block.serialize();
        println!("Serialized Block (first 20 bytes): {:?}", &serialized_block[..20]);

        let deserialized_block = Block::deserialize(&serialized_block).unwrap();
        println!("Deserialized Block: {:?}", deserialized_block);

        assert_eq!(last_block.header.nonce, deserialized_block.header.nonce);
        assert_eq!(last_block.transactions.len(), deserialized_block.transactions.len());
    }

    // Demonstrate transaction serialization and deserialization
    let serialized_tx = signed_tx.serialize();
    println!("Serialized Transaction (first 20 bytes): {:?}", &serialized_tx[..20]);

    let deserialized_tx = Transaction::deserialize(&serialized_tx).unwrap();
    println!("Deserialized Transaction: {:?}", deserialized_tx);

    assert_eq!(signed_tx.inputs.len(), deserialized_tx.inputs.len());
    assert_eq!(signed_tx.outputs.len(), deserialized_tx.outputs.len());

    println!("Block and transaction serialization and deserialization successful!");
}

fn calculate_transaction_hash(tx: &Transaction) -> [u8; 32] {
    let mut hasher = Sha256::new();
    for input in &tx.inputs {
        hasher.update(&input.txid);
        hasher.update(&input.vout.to_le_bytes());
    }
    for output in &tx.outputs {
        hasher.update(&output.value.to_le_bytes());
        hasher.update(&output.script_pubkey);
    }
    hasher.finalize().into()
}
