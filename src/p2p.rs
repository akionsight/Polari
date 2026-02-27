// src/p2p.rs
use libp2p::{
    gossipsub, identity, mdns, noise,
    swarm::{NetworkBehaviour, SwarmEvent, Config as SwarmConfig},
    tcp, yamux, PeerId, Swarm, Transport,  // Transport trait must be in scope to call .upgrade()
};
use futures::StreamExt;
use std::error::Error;
use log::{info, warn};
use bincode;
use redis::Commands;
use crate::blockchain::Blockchain;
use crate::block::Block;
use crate::net_message::NetMessage;
use serde_json;

// FIX: NetworkBehaviour must come from libp2p::swarm, not libp2p root.
// FIX: #[behaviour(out_event)] is deprecated in 0.52 — the derive now auto-generates
//      a `BehaviourEvent` enum with variants named after each field (Gossipsub, Mdns).
// FIX: mdns::tokio::Behaviour and tcp::tokio require "tokio" feature in Cargo.toml.
// FIX: No hand-written EventType enum or From impls needed anymore.
#[derive(NetworkBehaviour)]
struct Behaviour {
    gossipsub: gossipsub::Behaviour,
    mdns:      mdns::tokio::Behaviour,
}

pub struct P2PNode {
    pub swarm: Swarm<Behaviour>,
    pub blockchain: Blockchain,
    pub blocks_topic: gossipsub::IdentTopic,
}

impl P2PNode {
    pub async fn new(difficulty: usize) -> Result<Self, Box<dyn Error>> {
        let local_keys = identity::Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(local_keys.public());
        info!("Local peer id: {}", local_peer_id);

        // FIX: tcp::tokio::Transport requires "tokio" feature (added to Cargo.toml)
        let transport = tcp::tokio::Transport::default()
            .upgrade(libp2p::core::upgrade::Version::V1Lazy)
            .authenticate(noise::Config::new(&local_keys)?)
            .multiplex(yamux::Config::default())
            .boxed();

        let gossipsub_config = gossipsub::ConfigBuilder::default()
            .heartbeat_interval(std::time::Duration::from_secs(10))
            .build()
            .expect("Valid gossipsub config");

        let gossipsub = gossipsub::Behaviour::new(
            gossipsub::MessageAuthenticity::Signed(local_keys),
            gossipsub_config,
        )?;

        // FIX: mdns::tokio::Behaviour, not mdns::Behaviour<PeerId>
        let mdns = mdns::tokio::Behaviour::new(mdns::Config::default(), local_peer_id)?;

        let behaviour = Behaviour { gossipsub, mdns };

        // FIX: Swarm::new now requires a 4th argument — SwarmConfig
        let mut swarm = Swarm::new(
            transport,
            behaviour,
            local_peer_id,
            SwarmConfig::with_tokio_executor(),
        );

        swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

        let blocks_topic = gossipsub::IdentTopic::new("supplychain/blocks");
        let blockchain = Blockchain::new(difficulty);

        Ok(P2PNode {
            swarm,
            blockchain,
            blocks_topic,
        })
    }

    pub fn subscribe(&mut self) -> Result<(), Box<dyn Error>> {
        self.swarm
            .behaviour_mut()
            .gossipsub
            .subscribe(&self.blocks_topic)?;
        Ok(())
    }

    pub fn broadcast_block(&mut self, block: Block) -> Result<(), Box<dyn Error>> {
        let msg  = NetMessage::NewBlock(block);
        let data = serde_json::to_vec(&msg)?;
        match self.swarm
            .behaviour_mut()
            .gossipsub
            .publish(self.blocks_topic.clone(), data)
        {
            Ok(_) => info!("✅ Broadcasted block #{}", self.blockchain.chain.len()),
            // Not enough peers yet — block is already persisted to Redis, so this is fine.
            // It will be re-broadcast once peers connect (see run() loop below).
            Err(gossipsub::PublishError::InsufficientPeers) => {
                warn!("⚠️  No peers yet — block saved to Redis, will broadcast on peer connect");
            }
            Err(e) => return Err(Box::new(e)),
        }
        Ok(())
    }

    pub fn handle_message(&mut self, msg: NetMessage) {
        match msg {
            NetMessage::NewBlock(block) => {
                let prev_block = self.blockchain.last_block();
                if block.index == prev_block.index + 1
                    && block.previous_hash == prev_block.hash
                    && block.hash == block.calculate_hash()
                {
                    // FIX #3: Persist peer-received blocks to Redis, not just in-memory
                    let mut con = self.blockchain.redis_conn();
                    let key = format!("block:{}", block.index);
                    let value = bincode::serialize(&block).unwrap();
                    let _: () = con.set(&key, value).unwrap();
                    let _: () = con.set("block:last_index", block.index).unwrap();

                    self.blockchain.chain.push(block);
                    info!("✅ Added peer block #{}", self.blockchain.chain.len());
                } else {
                    warn!("❌ Invalid block received");
                }
            }
            NetMessage::RequestChain => {
                let chain = self.blockchain.chain.clone();
                let msg = NetMessage::ChainResponse(chain);
                let data = serde_json::to_vec(&msg).unwrap();
                let _ = self
                    .swarm
                    .behaviour_mut()
                    .gossipsub
                    .publish(self.blocks_topic.clone(), data);
            }
            NetMessage::ChainResponse(chain) => {
                for block in chain {
                    let prev_block = self.blockchain.last_block();
                    if block.index == prev_block.index + 1
                        && block.previous_hash == prev_block.hash
                    {
                        // FIX #3: Also persist synced chain blocks to Redis
                        let mut con = self.blockchain.redis_conn();
                        let key = format!("block:{}", block.index);
                        let value = bincode::serialize(&block).unwrap();
                        let _: () = con.set(&key, value).unwrap();
                        let _: () = con.set("block:last_index", block.index).unwrap();

                        self.blockchain.chain.push(block);
                    } else {
                        break;
                    }
                }
                info!(
                    "✅ Synced chain, now length: {}",
                    self.blockchain.chain.len()
                );
            }
        }
    }

    pub async fn run(mut self) -> Result<(), Box<dyn Error>> {
        info!("🚀 P2P Node Started - Chain length: {}", self.blockchain.chain.len());
        loop {
            // FIX: Match on auto-generated BehaviourEvent (replaces the old EventType).
            //      Variant names match struct field names in title case: Gossipsub, Mdns.
            match self.swarm.select_next_some().await {
                SwarmEvent::Behaviour(BehaviourEvent::Gossipsub(
                    gossipsub::Event::Message { message, .. },
                )) => {
                    if let Ok(msg) = serde_json::from_slice::<NetMessage>(&message.data) {
                        self.handle_message(msg);
                    }
                }
                SwarmEvent::Behaviour(BehaviourEvent::Mdns(
                    mdns::Event::Discovered(peers),
                )) => {
                    for (peer, _) in peers {
                        info!("🔍 Discovered peer: {}", peer);
                        self.swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer);
                    }
                }
                SwarmEvent::NewListenAddr { address, .. } => {
                    info!("📡 Listening on {}", address);
                }
                SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                    info!("🔗 Connected to {}", peer_id);
                    // Re-broadcast our latest block now that we have a peer to reach
                    let block = self.blockchain.last_block().clone();
                    let msg  = NetMessage::NewBlock(block);
                    let data = serde_json::to_vec(&msg).unwrap();
                    match self.swarm
                        .behaviour_mut()
                        .gossipsub
                        .publish(self.blocks_topic.clone(), data)
                    {
                        Ok(_) => info!("✅ Re-broadcast latest block to new peer"),
                        Err(gossipsub::PublishError::Duplicate) => {} // peer already saw it, fine
                        Err(e) => warn!("⚠️  Re-broadcast failed: {}", e),
                    }
                }
                _ => {}
            }
        }
    }
}