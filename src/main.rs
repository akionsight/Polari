mod block;
mod blockchain;

use blockchain::Blockchain;
use block::SupplyChainData;
use chrono::Utc;

fn main() {
    let mut bc = Blockchain::new(2); // loads from Redis or creates genesis

    bc.add_data(SupplyChainData {
        item_id: "ECID123".to_string(),
        event_type: "manufacture".to_string(),
        location: "Factory, Gharroli, Delhi".to_string(),
        timestamp: Utc::now().to_rfc3339(),
        owner: "SupplierA".to_string(),
        document_hash: "sha256_cert_abc123".to_string(),
    });
    bc.mine_pending_data("Miner1");

    // Tamper with the chain
// bc.chain[1].data.location = "Hacked".to_string();  // uncomment to test

    println!("Chain valid: {}", bc.is_chain_valid());

        // Use get_item_trace so it’s not dead code
    let trace = bc.get_item_trace("ECID123");
    println!("Trace length for ECID123: {}", trace.len());
}
