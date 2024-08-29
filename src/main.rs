mod structures;
mod blockchain;
mod wallet;
mod utxo;
mod merkle_tree;

use structures::{Transaction, TransactionInput, TransactionOutput};
use blockchain::Blockchain;
use wallet::Wallet;

fn main() {
    let mut blockchain = Blockchain::new();
    let alice_wallet = Wallet::new();
    let bob_wallet = Wallet::new();

    println!("Alice's wallet address: {}", alice_wallet.get_address());
    println!("Bob's wallet address: {}", bob_wallet.get_address());

    println!("Initial balances:");
    println!("Alice's balance: {}", blockchain.get_balance(&alice_wallet.get_address()));
    println!("Bob's balance: {}", blockchain.get_balance(&bob_wallet.get_address()));

    let genesis_tx = blockchain.chain[0].transactions[0].clone();
    let alice_tx = Transaction {
        inputs: vec![TransactionInput {
            txid: blockchain.calculate_transaction_hash(&genesis_tx),
            vout: 0,
            script_sig: alice_wallet.sign_transaction(&blockchain.calculate_transaction_hash(&genesis_tx)),
        }],
        outputs: vec![TransactionOutput {
            value: 25_000_000_000, // 250 BTC
            script_pubkey: alice_wallet.get_address().as_bytes().to_vec(),
        }],
    };

    println!("\nAdding transaction from genesis to Alice");
    blockchain.add_transaction(&alice_tx).unwrap();

    let new_block = blockchain.mine_block();
    blockchain.add_block(new_block).unwrap();

    println!("\nBalances after genesis -> Alice transaction:");
    println!("Alice's balance: {}", blockchain.get_balance(&alice_wallet.get_address()));
    println!("Bob's balance: {}", blockchain.get_balance(&bob_wallet.get_address()));

    let bob_tx = Transaction {
        inputs: vec![TransactionInput {
            txid: blockchain.calculate_transaction_hash(&alice_tx),
            vout: 0,
            script_sig: alice_wallet.sign_transaction(&blockchain.calculate_transaction_hash(&alice_tx)),
        }],
        outputs: vec![
            TransactionOutput {
                value: 10_000_000_000, // 100 BTC to Bob
                script_pubkey: bob_wallet.get_address().as_bytes().to_vec(),
            },
            TransactionOutput {
                value: 14_999_000_000, // 149.99 BTC back to Alice (change)
                script_pubkey: alice_wallet.get_address().as_bytes().to_vec(),
            },
        ],
    };

    println!("\nAdding transaction from Alice to Bob");
    blockchain.add_transaction(&bob_tx).unwrap();

    let new_block = blockchain.mine_block();
    blockchain.add_block(new_block).unwrap();

    println!("\nFinal balances:");
    println!("Alice's balance: {}", blockchain.get_balance(&alice_wallet.get_address()));
    println!("Bob's balance: {}", blockchain.get_balance(&bob_wallet.get_address()));

    println!("\nFinal UTXO set:");
    blockchain.utxo_set.print_utxos();

    println!("\nBlockchain information:");
    println!("Number of blocks: {}", blockchain.chain.len());
    for (i, block) in blockchain.chain.iter().enumerate() {
        println!("Block {}:", i);
        println!("  Previous hash: {:?}", block.header.prev_block_hash);
        println!("  Merkle root: {:?}", block.header.merkle_root);
        println!("  Nonce: {}", block.header.nonce);
        println!("  Difficulty: {}", block.header.difficulty);
        println!("  Transactions: {}", block.transactions.len());
    }
}
