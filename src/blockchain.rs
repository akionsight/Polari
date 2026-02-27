// src/blockchain.rs
use crate::block::{Block, SupplyChainData};
use std::fmt;
use redis::{Client, Commands};
use bincode;

#[derive(Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
    pub pending_data: Vec<SupplyChainData>,
    redis_client: Client,
}

impl Blockchain {
    pub fn new(difficulty: usize) -> Self {
        let client = Client::open("redis://127.0.0.1/").expect("failed to connect to Redis");
        let mut con = client.get_connection().expect("failed to get Redis connection");

        let mut chain: Vec<Block> = Vec::new();

        let last_index: Option<u64> = redis::cmd("GET")
            .arg("block:last_index")
            .query(&mut con)
            .ok();

        if let Some(last) = last_index {
            // Load all blocks from Redis
            for i in 0..=last {
                let key = format!("block:{}", i);
                let data: Vec<u8> = con.get(key).expect("failed to get block");
                let block: Block = bincode::deserialize(&data).expect("deserialize block");
                chain.push(block);
            }
        } else {
            // No existing chain: create and persist genesis block
            let genesis_data = SupplyChainData {
                item_id: "GENESIS".to_string(),
                event_type: "genesis".to_string(),
                location: "Origin".to_string(),
                timestamp: chrono::Utc::now().to_rfc3339(),
                owner: "System".to_string(),
                document_hash: "genesis_hash".to_string(),
            };
            let mut genesis_block = Block::new(0, genesis_data, String::from("0"));
            genesis_block.mine_block(difficulty);

            let key = "block:0";
            let value = bincode::serialize(&genesis_block).unwrap();
            let _: () = con.set(key, value).unwrap();
            let _: () = con.set("block:last_index", 0u64).unwrap();

            chain.push(genesis_block);
        }

        chain.sort_by_key(|b| b.index);

        Blockchain {
            chain,
            difficulty,
            pending_data: Vec::new(),
            redis_client: client,
        }
    }

    // FIX #3 (partial): Made pub so p2p.rs can persist peer blocks to Redis
    pub fn redis_conn(&self) -> redis::Connection {
        self.redis_client
            .get_connection()
            .expect("failed to get Redis connection")
    }

    pub fn add_data(&mut self, data: SupplyChainData) {
        self.pending_data.push(data);
    }

    pub fn mine_pending_data(&mut self, miner_address: &str) {
        if self.pending_data.is_empty() {
            return;
        }

        let data = self.pending_data[0].clone();
        let previous_block = self.last_block();

        let mut block = Block::new(
            previous_block.index + 1,
            SupplyChainData {
                item_id: data.item_id.clone(),
                event_type: data.event_type.clone(),
                location: data.location.clone(),
                timestamp: data.timestamp.clone(),
                owner: miner_address.to_string(),
                document_hash: data.document_hash.clone(),
            },
            previous_block.hash.clone(),
        );

        block.mine_block(self.difficulty);

        // Persist to Redis
        let mut con = self.redis_conn();
        let key = format!("block:{}", block.index);
        let value = bincode::serialize(&block).unwrap();
        let _: () = con.set(key, value).unwrap();
        let _: () = con.set("block:last_index", block.index).unwrap();

        self.chain.push(block);
        self.pending_data.clear();
    }

    pub fn last_block(&self) -> &Block {
        self.chain.last().unwrap()
    }

    pub fn is_chain_valid(&self) -> bool {
        if self.chain.len() <= 1 {
            return true;
        }

        for i in 1..self.chain.len() {
            let previous = &self.chain[i - 1];
            let current = &self.chain[i];

            if !current.is_valid_block(previous, self.difficulty) {
                return false;
            }
        }

        true
    }

    pub fn get_item_trace(&self, item_id: &str) -> Vec<&Block> {
        self.chain
            .iter()
            .filter(|b| b.data.item_id == item_id)
            .collect()
    }
}

impl fmt::Display for Blockchain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, block) in self.chain.iter().enumerate() {
            writeln!(f, "\n=== Block {} ===", i)?;
            block.fmt(f)?;
        }
        Ok(())
    }
}