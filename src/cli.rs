use crate::blockchain::Blockchain;
use crate::wallet::Wallet;
use crate::structures::{Transaction, TransactionInput, TransactionOutput};
use std::io::{self, Write};

pub struct CLI {
    blockchain: Blockchain,
    wallets: Vec<Wallet>,
}

impl CLI {
    pub fn new() -> Self {
        CLI {
            blockchain: Blockchain::new(),
            wallets: vec![],
        }
    }

    pub fn run(&mut self) {
        loop {
            print_menu();
            let choice = get_user_input("Enter your choice: ");
            match choice.as_str() {
                "1" => self.create_wallet(),
                "2" => self.list_wallets(),
                "3" => self.send_transaction(),
                "4" => self.mine_block(),
                "5" => self.print_blockchain_info(),
                "6" => self.print_wallet_balance(),
                "7" => break,
                _ => println!("Invalid choice. Please try again."),
            }
            println!();
        }
    }

    fn create_wallet(&mut self) {
        let wallet = Wallet::new();
        println!("New wallet created with address: {}", wallet.get_address());
        self.wallets.push(wallet);
    }

    fn list_wallets(&self) {
        for (i, wallet) in self.wallets.iter().enumerate() {
            println!("Wallet {}: {}", i + 1, wallet.get_address());
        }
    }

    fn send_transaction(&mut self) {
        if self.wallets.is_empty() {
            println!("No wallets available. Please create a wallet first.");
            return;
        }

        self.list_wallets();
        let sender_index = get_user_input("Enter sender wallet index: ").parse::<usize>().unwrap() - 1;
        let recipient_index = get_user_input("Enter recipient wallet index: ").parse::<usize>().unwrap() - 1;
        let amount_btc = get_user_input("Enter amount to send in BTC: ").parse::<f64>().unwrap();
        let amount_satoshis = (amount_btc * 100_000_000.0) as u64; // Convert BTC to satoshis

        let sender = &self.wallets[sender_index];
        let recipient = &self.wallets[recipient_index];

        // This is a simplified transaction creation. In a real implementation,
        // you'd need to properly handle UTXOs and change.
        let tx = Transaction {
            inputs: vec![TransactionInput {
                txid: [0; 32], // This should be a real previous transaction
                vout: 0,
                script_sig: sender.sign_transaction(&[0; 32]), // This should sign the actual transaction data
            }],
            outputs: vec![TransactionOutput {
                value: amount_satoshis,
                script_pubkey: recipient.get_address().as_bytes().to_vec(),
            }],
        };

        match self.blockchain.add_transaction(&tx) {
            Ok(_) => println!("Transaction added to mempool."),
            Err(e) => println!("Failed to add transaction: {}", e),
        }
    }

    fn mine_block(&mut self) {
        let new_block = self.blockchain.mine_block();
        match self.blockchain.add_block(new_block) {
            Ok(_) => println!("New block mined and added to the blockchain."),
            Err(e) => println!("Failed to add block: {}", e),
        }
    }

    fn print_blockchain_info(&self) {
        println!("Blockchain information:");
        println!("Number of blocks: {}", self.blockchain.chain.len());
        for (i, block) in self.blockchain.chain.iter().enumerate() {
            println!("Block {}:", i);
            println!("  Transactions: {}", block.transactions.len());
            println!("  Previous hash: {:?}", block.header.prev_block_hash);
            println!("  Merkle root: {:?}", block.header.merkle_root);
            println!("  Nonce: {}", block.header.nonce);
            println!("  Difficulty: {}", block.header.difficulty);
        }
    }

    fn print_wallet_balance(&self) {
        if self.wallets.is_empty() {
            println!("No wallets available. Please create a wallet first.");
            return;
        }

        self.list_wallets();
        let wallet_index = get_user_input("Enter wallet index: ").parse::<usize>().unwrap() - 1;
        let wallet = &self.wallets[wallet_index];
        let balance_satoshis = self.blockchain.get_balance(&wallet.get_address());
        let balance_btc = balance_satoshis as f64 / 100_000_000.0;
        println!("Balance of wallet {}: {} BTC ({} satoshis)", wallet.get_address(), balance_btc, balance_satoshis);
    }
}

fn print_menu() {
    println!("1. Create new wallet");
    println!("2. List wallets");
    println!("3. Send transaction");
    println!("4. Mine block");
    println!("5. Print blockchain info");
    println!("6. Print wallet balance");
    println!("7. Exit");
}

fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
