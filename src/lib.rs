// src/lib.rs  ← FIX #4: was incorrectly labelled "net_message.rs"
pub mod blockchain;
pub mod block;

pub use blockchain::Blockchain;
pub use block::{Block, SupplyChainData};

pub mod net_message;
pub use net_message::NetMessage;
