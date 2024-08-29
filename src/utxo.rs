use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UTXO {
    pub txid: [u8; 32],
    pub vout: u32,
    pub amount: u64,
    pub script_pubkey: Vec<u8>,
}

#[derive(Debug)]
pub struct UTXOSet {
    utxos: HashMap<String, UTXO>,
}

impl UTXOSet {
    pub fn new() -> Self {
        UTXOSet {
            utxos: HashMap::new(),
        }
    }

    pub fn add_utxo(&mut self, utxo: UTXO) {
        let key = self.generate_key(&utxo.txid, utxo.vout);
        self.utxos.insert(key, utxo);
    }

    pub fn remove_utxo(&mut self, txid: &[u8; 32], vout: u32) -> Option<UTXO> {
        let key = self.generate_key(txid, vout);
        self.utxos.remove(&key)
    }

    pub fn get_utxo(&self, txid: &[u8; 32], vout: u32) -> Option<&UTXO> {
        let key = self.generate_key(txid, vout);
        self.utxos.get(&key)
    }

    pub fn get_balance(&self, address: &str) -> u64 {
        self.utxos.values()
            .filter(|utxo| self.address_from_script_pubkey(&utxo.script_pubkey) == address)
            .map(|utxo| utxo.amount)
            .sum()
    }

    fn generate_key(&self, txid: &[u8; 32], vout: u32) -> String {
        format!("{}:{}", hex::encode(txid), vout)
    }

    fn address_from_script_pubkey(&self, script_pubkey: &[u8]) -> String {
        // This is still a simplified version, but it should work for our current implementation
        String::from_utf8_lossy(script_pubkey).to_string()
    }

    pub fn print_utxos(&self) {
        for (key, utxo) in &self.utxos {
            println!("UTXO {} - Amount: {}, ScriptPubKey: {}", key, utxo.amount, self.address_from_script_pubkey(&utxo.script_pubkey));
        }
    }
}
