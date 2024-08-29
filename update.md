# Project Update: Bitcoin-like Blockchain Implementation in Rust

## Overview
This document outlines the progress and updates made to our Bitcoin-like blockchain implementation in Rust. The project aims to create a simplified version of a blockchain system, incorporating key concepts from Bitcoin and other cryptocurrencies.

## Key Components Implemented

### 1. Block and Transaction Structures
- Implemented `Block` and `Transaction` structs.
- Included essential fields such as version, timestamp, merkle root, inputs, and outputs.
- Created `BlockHeader` struct to separate block metadata from transactions.

### 2. Blockchain Core
- Developed a `Blockchain` struct to manage the chain of blocks.
- Implemented methods to add new blocks and maintain blockchain integrity.
- Created a genesis block to initialize the blockchain.

### 3. Wallet Functionality
- Implemented a `Wallet` struct for managing cryptographic keys.
- Added methods for generating private/public key pairs using secp256k1.
- Implemented Bitcoin-like address generation from public keys.
- Created functionality for signing transactions.

### 4. Mining Simulation
- Implemented a basic Proof of Work (PoW) mining algorithm.
- Added functionality to adjust mining difficulty.
- Created methods to validate mined blocks before adding them to the chain.

### 5. Serialization and Deserialization
- Implemented methods to serialize and deserialize blocks and transactions.
- Utilized the `bincode` library for efficient binary serialization.

### 6. Transaction Handling
- Developed methods to create and validate transactions.
- Implemented basic UTXO (Unspent Transaction Output) tracking.

### 7. Cryptographic Functions
- Integrated cryptographic libraries for secure operations.
- Implemented hashing functions using SHA-256.
- Added digital signature creation and verification.

## Technical Details

### Updated Dependencies
- secp256k1: For elliptic curve cryptography
- rand: For secure random number generation
- sha2: For SHA-256 hashing
- bs58: For Base58 encoding (used in address generation)
- bincode: For binary serialization
- serde: For serialization and deserialization of data structures
- chrono: For timestamp handling

### Key Struct Implementations
1. `Block`: Represents a block in the blockchain.
2. `Transaction`: Represents a transaction within a block.
3. `Blockchain`: Manages the entire blockchain.
4. `Wallet`: Handles key management and transaction signing.

### Major Functions Added
- `Blockchain::new()`: Initializes a new blockchain with a genesis block.
- `Blockchain::add_block()`: Adds a new block to the chain.
- `Blockchain::mine_block()`: Simulates the mining process to create new blocks.
- `Wallet::new()`: Creates a new wallet with a keypair.
- `Wallet::get_address()`: Generates a Bitcoin-like address from the public key.
- `Wallet::sign_transaction()`: Signs a transaction using the wallet's private key.

## Next Steps
1. Implement more robust transaction validation.
2. Add a simple peer-to-peer networking layer.
3. Improve the mining algorithm with dynamic difficulty adjustment.
4. Enhance the UTXO model for accurate balance tracking.
5. Create a user-friendly CLI or API for blockchain interaction.
6. Implement comprehensive testing, including edge cases and security scenarios.

More soon...
