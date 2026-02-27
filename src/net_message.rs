// src/net_message.rs
use serde::{Serialize, Deserialize};
use crate::block::Block;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum NetMessage {
    NewBlock(Block),
    RequestChain,
    ChainResponse(Vec<Block>),
}