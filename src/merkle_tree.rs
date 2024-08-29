use sha2::{Sha256, Digest};

pub struct MerkleTree {
    pub root: String,
}

impl MerkleTree {
    pub fn new(transactions: &Vec<String>) -> Self {
        let mut hashes = transactions.iter()
            .map(|tx| Self::hash(tx))
            .collect::<Vec<String>>();

        while hashes.len() > 1 {
            hashes = Self::get_parent_hashes(&hashes);
        }

        MerkleTree {
            root: hashes[0].clone(),
        }
    }

    fn get_parent_hashes(hashes: &Vec<String>) -> Vec<String> {
        let mut parent_hashes = Vec::new();

        for chunk in hashes.chunks(2) {
            if chunk.len() == 2 {
                let parent = Self::hash(&(chunk[0].clone() + &chunk[1]));
                parent_hashes.push(parent);
            } else {
                parent_hashes.push(chunk[0].clone());
            }
        }

        parent_hashes
    }

    fn hash(data: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        let result = hasher.finalize();
        hex::encode(result)
    }
}
