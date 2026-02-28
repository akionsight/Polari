// mod block;
// mod blockchain;
// mod net_message;
// mod p2p;

// use p2p::P2PNode;
// use block::SupplyChainData;
// use log::info;
// use chrono::Utc;

// #[tokio::main]

// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     pretty_env_logger::init();

//     let mut node = P2PNode::new(2).await?;
//     node.subscribe()?;

//     // Add supply chain data
//     node.blockchain.add_data(SupplyChainData {
//         item_id: "ECID123".to_string(),
//         event_type: "manufacture".to_string(),
//         location: "Factory, Pune, India".to_string(),
//         timestamp: Utc::now().to_rfc3339(),
//         owner: "Omron".to_string(),
//         document_hash: "sha256_cert_abc123".to_string(),
//     });

//     // Mine pending data
//     node.blockchain.mine_pending_data("Miner1");

//     // FIX #1: Clone last block first to avoid simultaneous mut + immutable borrow
//     let last_block = node.blockchain.last_block().clone();
//     node.broadcast_block(last_block)?;

//     info!("✅ Chain valid: {}", node.blockchain.is_chain_valid());
//     info!("📊 Chain length: {}", node.blockchain.chain.len());

//     node.run().await?;
//     Ok(())
// }

mod block;
mod blockchain;
mod net_message;
mod p2p;

use block::SupplyChainData;
use chrono::Utc;
use log::info;
use std::env;
use rand::Rng;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    let args: Vec<String> = env::args().collect();

    // Mode 1: CLI add event (no P2P)
    //
    // Example:
    // cargo run -- add ECID123 manufacture "Factory, Delhi" SupplierA some_doc_hash
    if args.len() >= 2 && args[1] == "add" {
        if args.len() < 6 {
            eprintln!("Usage: cargo run -- add <item_id> <event_type> <location> <owner> [document_hash]");
            std::process::exit(1);
        }

        let item_id = args[2].clone();
        let event_type = args[3].clone();
        let location = args[4].clone();
        let owner = args[5].clone();
        let document_hash = if args.len() >= 7 {
    args[6].clone()
} else {
    let mut rng = rand::thread_rng();
    let suffix: u16 = rng.gen();
    format!("auto_{}_{}", Utc::now().timestamp(), suffix)
};

        let mut bc = blockchain::Blockchain::new(2);

        bc.add_data(SupplyChainData {
            item_id: item_id.clone(),
            event_type,
            location,
            timestamp: Utc::now().to_rfc3339(),
            owner,
            document_hash,
        });

        bc.mine_pending_data("CLI-Miner");
        println!("Added new block. Chain length: {}", bc.chain.len());
        println!("Chain valid: {}", bc.is_chain_valid());
        return Ok(());
    }

    // Mode 2: Run full P2P node (what we had earlier)
    let mut node = p2p::P2PNode::new(2).await?;
    node.subscribe()?;

    info!("P2P node started. Chain length: {}", node.blockchain.chain.len());
    node.run().await?;
    Ok(())
}