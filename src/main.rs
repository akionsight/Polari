mod block;
mod blockchain;
mod net_message;
mod p2p;

use p2p::P2PNode;
use block::SupplyChainData;
use log::info;
use chrono::Utc;

#[tokio::main]

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    let mut node = P2PNode::new(2).await?;
    node.subscribe()?;

    // Add supply chain data
    node.blockchain.add_data(SupplyChainData {
        item_id: "ECID123".to_string(),
        event_type: "manufacture".to_string(),
        location: "Factory, Gharroli, Delhi".to_string(),
        timestamp: Utc::now().to_rfc3339(),
        owner: "SupplierA".to_string(),
        document_hash: "sha256_cert_abc123".to_string(),
    });

    // Mine pending data
    node.blockchain.mine_pending_data("Miner1");

    // FIX #1: Clone last block first to avoid simultaneous mut + immutable borrow
    let last_block = node.blockchain.last_block().clone();
    node.broadcast_block(last_block)?;

    info!("✅ Chain valid: {}", node.blockchain.is_chain_valid());
    info!("📊 Chain length: {}", node.blockchain.chain.len());

    node.run().await?;
    Ok(())
}