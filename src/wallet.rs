use secp256k1::{Secp256k1, SecretKey, PublicKey, Message};
use rand::Rng;
use sha2::{Sha256, Digest};
use bs58;

pub struct Wallet {
    private_key: SecretKey,
    public_key: PublicKey,
}

impl Wallet {
    pub fn new() -> Self {
        let secp = Secp256k1::new();
        let mut rng = rand::thread_rng();
        
        // Generate a random 32-byte array for the private key
        let mut private_key_bytes = [0u8; 32];
        rng.fill(&mut private_key_bytes);
        
        // Create SecretKey from the random bytes
        let secret_key = SecretKey::from_slice(&private_key_bytes).expect("32 bytes, within curve order");
        let public_key = PublicKey::from_secret_key(&secp, &secret_key);

        Wallet {
            private_key: secret_key,
            public_key,
        }
    }

    pub fn get_address(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(&self.public_key.serialize());
        let public_key_hash = hasher.finalize();

        let mut address = vec![0]; // Version byte
        address.extend_from_slice(&public_key_hash[..20]); // RIPEMD160 of SHA256

        let checksum = Sha256::digest(&Sha256::digest(&address));
        address.extend_from_slice(&checksum[..4]);

        bs58::encode(address).into_string()
    }

    pub fn sign_transaction(&self, transaction_hash: &[u8; 32]) -> Vec<u8> {
        let secp = Secp256k1::new();
        let message = Message::from_slice(transaction_hash).unwrap();
        let signature = secp.sign(&message, &self.private_key);
        signature.serialize_der().to_vec()
    }
}
