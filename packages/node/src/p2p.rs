//! P2P сеть на libp2p: Kademlia (discovery) + Gossipsub (broadcast) + Request-Response (sync).

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::Duration;

use futures::StreamExt;
use libp2p::{
    gossipsub, identify, kad, noise, ping, request_response,
    swarm::{NetworkBehaviour, SwarmEvent},
    tcp, yamux, Multiaddr, PeerId, SwarmBuilder,
};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

use std::sync::Arc;
use std::sync::RwLock;

use crate::block::Block;
use crate::chain::Chain;
use crate::transaction::Transaction;

/// Поведение ноды в P2P сети.
#[derive(NetworkBehaviour)]
pub struct AicBehaviour {
    pub kademlia: kad::Behaviour<kad::store::MemoryStore>,
    pub gossipsub: gossipsub::Behaviour,
    pub sync: request_response::cbor::Behaviour<SyncRequest, SyncResponse>,
    pub identify: identify::Behaviour,
    pub ping: ping::Behaviour,
}

/// Запрос синхронизации: "дай мне блоки начиная с N".
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncRequest {
    pub from_block: u64,
    pub max_blocks: u64,
}

/// Ответ: список блоков.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncResponse {
    pub blocks: Vec<Block>,
}

/// События, которые P2P-слой отправляет в основную логику ноды.
#[derive(Debug, Clone)]
pub enum P2PEvent {
    NewBlock(Block),
    NewTransaction(Transaction),
    PeerConnected(PeerId),
    PeerDisconnected(PeerId),
}

/// Каналы для управления P2P-слоем извне.
pub struct P2PHandles {
    pub event_rx: mpsc::UnboundedReceiver<P2PEvent>,
    pub gossipsub_tx: gossipsub::IdentTopic,
    pub command_tx: mpsc::UnboundedSender<P2PCommand>,
}

/// Команды, которые основная логика отправляет в P2P-слой.
#[derive(Debug)]
pub enum P2PCommand {
    BroadcastBlock(Block),
    BroadcastTx(Transaction),
    RequestSync { peer: PeerId, from_block: u64 },
}

/// Запустить P2P-слой. Возвращает каналы для интеграции.
pub async fn start_p2p(
    listen_port: u16,
    bootnodes: Vec<String>,
    chain: Arc<RwLock<Chain>>,
) -> Result<P2PHandles, Box<dyn std::error::Error>> {
    let mut swarm = SwarmBuilder::with_new_identity()
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )?
        .with_behaviour(|key| {
            // Kademlia
            let peer_id = key.public().to_peer_id();
            let store = kad::store::MemoryStore::new(peer_id);
            let mut kademlia = kad::Behaviour::new(peer_id, store);
            kademlia.set_mode(Some(kad::Mode::Server));

            // Gossipsub
            let gossipsub_config = gossipsub::ConfigBuilder::default()
                .heartbeat_interval(Duration::from_secs(1))
                .validation_mode(gossipsub::ValidationMode::Strict)
                .build()
                .expect("valid gossipsub config");
            let mut gossipsub = gossipsub::Behaviour::new(
                gossipsub::MessageAuthenticity::Signed(key.clone()),
                gossipsub_config,
            ).expect("gossipsub");
            let block_topic = gossipsub::IdentTopic::new("aic/blocks/1");
            let tx_topic = gossipsub::IdentTopic::new("aic/txs/1");
            gossipsub.subscribe(&block_topic).ok();
            gossipsub.subscribe(&tx_topic).ok();

            // Request-Response для синхронизации
            let sync = request_response::cbor::Behaviour::<SyncRequest, SyncResponse>::new(
                [(
                    libp2p::StreamProtocol::new("/aic/sync/1"),
                    request_response::ProtocolSupport::Full,
                )],
                request_response::Config::default().with_request_timeout(Duration::from_secs(30)),
            );

            // Identify
            let identify = identify::Behaviour::new(identify::Config::new(
                "/aic/id/1".to_string(),
                key.public(),
            ));

            // Ping
            let ping = ping::Behaviour::new(ping::Config::new());

            Ok(AicBehaviour { kademlia, gossipsub, sync, identify, ping })
        })?
        .with_swarm_config(|c| c.with_idle_connection_timeout(Duration::from_secs(60)))
        .build();

    // Слушаем порт
    let listen_addr: Multiaddr = format!("/ip4/0.0.0.0/tcp/{listen_port}").parse()?;
    swarm.listen_on(listen_addr.clone())?;
    println!("[P2P] Listening on {listen_addr}");

    let local_peer_id = *swarm.local_peer_id();
    println!("[P2P] ===============================================");
    println!("[P2P] Local PeerId: {local_peer_id}");
    println!("[P2P] Share with other nodes as --bootnodes:");
    println!("[P2P]   /ip4/127.0.0.1/tcp/{listen_port}/p2p/{local_peer_id}");
    println!("[P2P] ===============================================");

    for addr_str in &bootnodes {
        if let Ok(addr) = addr_str.parse::<Multiaddr>() {
            println!("[P2P] Dialing bootnode: {addr}");
            if let Some(idx) = addr_str.rfind("/p2p/") {
                let pid_str = &addr_str[idx+5..];
                if let Ok(pid) = pid_str.parse::<PeerId>() {
                    swarm.behaviour_mut().kademlia.add_address(&pid, addr.clone());
                }
            }
            swarm.dial(addr.clone()).ok();
        }
    }

    // Каналы
    let (event_tx, event_rx) = mpsc::unbounded_channel();
    let (command_tx, mut command_rx) = mpsc::unbounded_channel();
    let block_topic = gossipsub::IdentTopic::new("aic/blocks/1");
    let tx_topic = gossipsub::IdentTopic::new("aic/txs/1");

    let block_topic_out = block_topic.clone();
    let tx_topic_out = tx_topic.clone();

    // Запускаем event loop
    tokio::spawn(async move {
        loop {
            tokio::select! {
                event = swarm.select_next_some() => {
                    match event {
                        SwarmEvent::Behaviour(AicBehaviourEvent::Gossipsub(
                            gossipsub::Event::Message { message, .. }
                        )) => {
                            let topic_hash = message.topic;
                            if topic_hash == block_topic.hash() {
                                if let Ok(block) = serde_json::from_slice::<Block>(&message.data) {
                                    eprintln!("[P2P] Gossip: new block #{}", block.header.number);
                                    let _ = event_tx.send(P2PEvent::NewBlock(block));
                                }
                            } else if topic_hash == tx_topic.hash() {
                                if let Ok(tx) = serde_json::from_slice::<Transaction>(&message.data) {
                                    eprintln!("[P2P] Gossip: new tx from 0x{}", hex::encode(&tx.from[..4]));
                                    let _ = event_tx.send(P2PEvent::NewTransaction(tx));
                                }
                            }
                        }
                        SwarmEvent::Behaviour(AicBehaviourEvent::Sync(
                            request_response::Event::Message { peer, message, .. }
                        )) => {
                            match message {
                                request_response::Message::Request { request, channel, .. } => {
                                    eprintln!("[P2P] Sync request from {peer}: from_block={}, max={}",
                                        request.from_block, request.max_blocks);
                                    // Читаем chain и отвечаем блоками
                                    let chain_guard = chain.read().unwrap();
                                    let from = request.from_block as usize;
                                    let max = request.max_blocks as usize;
                                    let blocks: Vec<Block> = chain_guard.blocks.iter()
                                        .skip(from)
                                        .take(max)
                                        .cloned()
                                        .collect();
                                    let n = blocks.len();
                                    drop(chain_guard);
                                    let response = SyncResponse { blocks };
                                    if swarm.behaviour_mut().sync.send_response(channel, response).is_ok() {
                                        eprintln!("[P2P] Sync response sent to {peer}: {} blocks", n);
                                    }
                                }
                                request_response::Message::Response { response, .. } => {
                                    eprintln!("[P2P] Sync response from {peer}: {} blocks", response.blocks.len());
                                    let mut chain_guard = chain.write().unwrap();
                                    let mut applied = 0;
                                    for block in response.blocks {
                                        if chain_guard.add_block_from_peer(block) {
                                            applied += 1;
                                        }
                                    }
                                    if applied > 0 {
                                        let chain_file = std::path::PathBuf::from("data").join("chain.json");
                                        if let Err(e) = chain_guard.save(&chain_file) {
                                            eprintln!("⚠ Save failed: {e}");
                                        }
                                        eprintln!("[P2P] Sync applied {} blocks, new head #{}",
                                            applied, chain_guard.block_number());
                                    }
                                }
                            }
                        }
                        SwarmEvent::Behaviour(AicBehaviourEvent::Kademlia(
                            kad::Event::RoutingUpdated { peer, .. }
                        )) => {
                            eprintln!("[P2P] Kademlia: routing updated, peer={peer}");
                        }
                        SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                            eprintln!("[P2P] Connected to {peer_id}");
                            let _ = event_tx.send(P2PEvent::PeerConnected(peer_id));
                            // Запрашиваем sync с нашего head+1
                            let our_head = chain.read().unwrap().block_number();
                            let req = SyncRequest { from_block: our_head + 1, max_blocks: 100 };
                            eprintln!("[P2P] Requesting sync from {peer_id}: from_block={}", our_head + 1);
                            swarm.behaviour_mut().sync.send_request(&peer_id, req);
                        }
                        SwarmEvent::ConnectionClosed { peer_id, .. } => {
                            eprintln!("[P2P] Disconnected from {peer_id}");
                            let _ = event_tx.send(P2PEvent::PeerDisconnected(peer_id));
                        }
                        _ => {}
                    }
                }
                cmd = command_rx.recv() => {
                    match cmd {
                        Some(P2PCommand::BroadcastBlock(block)) => {
                            if let Ok(bytes) = serde_json::to_vec(&block) {
                                let _ = swarm.behaviour_mut().gossipsub.publish(block_topic.clone(), bytes);
                            }
                        }
                        Some(P2PCommand::BroadcastTx(tx)) => {
                            if let Ok(bytes) = serde_json::to_vec(&tx) {
                                let _ = swarm.behaviour_mut().gossipsub.publish(tx_topic.clone(), bytes);
                            }
                        }
                        Some(P2PCommand::RequestSync { peer, from_block }) => {
                            let req = SyncRequest { from_block, max_blocks: 100 };
                            swarm.behaviour_mut().sync.send_request(&peer, req);
                        }
                        None => break,
                    }
                }
            }
        }
    });

    Ok(P2PHandles {
        event_rx,
        gossipsub_tx: block_topic_out,
        command_tx,
    })
}


