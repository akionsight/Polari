// src/block.rs
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use hex;
use std::fmt;
use chrono::Utc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupplyChainData {
    pub item_id: String,
    pub event_type: String,  
    pub location: String,
    pub timestamp: String,
    pub owner: String,
    pub document_hash: String,  
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: String,
    pub proof_of_work: u64,
    pub previous_hash: String,
    pub data: SupplyChainData,
    pub hash: String,
}

impl Block {

    pub fn new(index: u64, data: SupplyChainData, previous_hash: String) -> Self {
        let timestamp = Utc::now().to_rfc3339();
        let mut block = Block {
            index,
            timestamp,
            proof_of_work: 0,
            previous_hash,
            data,
            hash: String::new(),
        };
        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> String {
        let input = serde_json::to_string(&BlockForHashing {
            index: self.index,
            timestamp: self.timestamp.clone(),
            proof_of_work: self.proof_of_work,
            previous_hash: self.previous_hash.clone(),
            data: self.data.clone(),
        }).unwrap();
        
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        hex::encode(hasher.finalize())
    }

    pub fn mine_block(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);
        while !self.hash.starts_with(&target) {
            self.proof_of_work += 1;
            self.hash = self.calculate_hash();
        }
    }

    pub fn is_valid_block(&self, previous_block: &Block, difficulty: usize) -> bool {
        // 1. Index should be previous index + 1
        if self.index != previous_block.index + 1 {
            return false;
        }

        // 2. previous_hash must match
        if self.previous_hash != previous_block.hash {
            return false;
        }

        // 3. Hash must be correct for this block's content
        let recalculated_hash = self.calculate_hash();
        if self.hash != recalculated_hash {
            return false;
        }

        // 4. Hash must satisfy difficulty (proof-of-work)
        let target = "0".repeat(difficulty);
        if !self.hash.starts_with(&target) {
            return false;
        }

        true
    }

}

// Helper struct for hashing (excludes hash field)
#[derive(Serialize)]
struct BlockForHashing {
    index: u64,
    timestamp: String,
    proof_of_work: u64,
    previous_hash: String,
    data: SupplyChainData,
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hash_short = if self.hash.len() >= 16 {
            &self.hash[..16]
        } else {
            &self.hash
        };
        let prev_short = if self.previous_hash.len() >= 16 {
            &self.previous_hash[..16]
        } else {
            &self.previous_hash
        };
        
        write!(f, "Block #{} [{}]\n", self.index, self.data.item_id)?;
        write!(f, "  Event: {}\n", self.data.event_type)?;
        write!(f, "  Location: {}\n", self.data.location)?;
        write!(f, "  Owner: {}\n", self.data.owner)?;
        write!(f, "  Hash: {}\n", hash_short)?;
        write!(f, "  Prev Hash: {}\n", prev_short)?;
        write!(f, "  PoW: {}\n", self.proof_of_work)?;
        write!(f, "---")
    }
}
