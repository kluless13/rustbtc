mod structures;
mod blockchain;
mod wallet;
mod utxo;
mod merkle_tree;
mod mempool;
mod cli;

use cli::CLI;

fn main() {
    let mut cli = CLI::new();
    cli.run();
}
