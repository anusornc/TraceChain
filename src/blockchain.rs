use chrono::Utc;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: String,
    pub data: String, // RDF in Turtle format
    pub previous_hash: String,
    pub hash: String,
}

impl Block {
    pub fn new(index: u64, data: String, previous_hash: String) -> Self {
        let timestamp = Utc::now().to_rfc3339();
        let mut block = Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash: String::new(),
        };
        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.index.to_le_bytes());
        hasher.update(self.timestamp.as_bytes());
        hasher.update(self.data.as_bytes());
        hasher.update(self.previous_hash.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}

pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Default for Blockchain {
    fn default() -> Self {
        Self::new()
    }
}

impl Blockchain {
    pub fn new() -> Self {
        let mut bc = Blockchain { chain: Vec::new() };
        bc.chain.push(bc.create_genesis_block());
        bc
    }

    fn create_genesis_block(&self) -> Block {
        Block::new(
            0,
            "@prefix ex: <http://example.org/> . ex:genesis ex:type \"Genesis Block\" .".into(),
            "0".into(),
        )
    }

    pub fn add_block(&mut self, data: String) {
        let prev_block = self.chain.last().unwrap();
        let new_block = Block::new(prev_block.index + 1, data, prev_block.hash.clone());
        self.chain.push(new_block);
    }

    pub fn is_valid(&self) -> bool {
        // also check genesis block
        if self.chain.is_empty() {
            return false;
        }
        if self.chain[0].hash != self.chain[0].calculate_hash() {
            return false;
        }

        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let prev = &self.chain[i - 1];
            if current.hash != current.calculate_hash() || current.previous_hash != prev.hash {
                return false;
            }
        }
        true
    }
}
