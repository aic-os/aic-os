mod block;
mod chain;
mod evm;
mod genesis_contracts;
mod mempool;
mod p2p;
mod rpc;
mod serde_helpers;
mod state;
mod transaction;

use std::sync::Arc;
use std::sync::RwLock;

use clap::Parser;

/// AIC Chain Node
#[derive(Parser, Debug)]
#[command(name = "aic-node", version, about = "AIC Chain full node")]
struct Args {
    #[arg(long, default_value = "8545")]
    rpc_port: u16,

    #[arg(long, default_value = "30333")]
    p2p_port: u16,

    #[arg(long, default_value_t = 2026)]
    chain_id: u64,

    #[arg(long, value_delimiter = ',')]
    bootnodes: Vec<String>,

    #[arg(long, default_value_t = false)]
    no_mine: bool,
}

const AIC: u128 = 1_000_000_000_000_000_000;

fn build_alloc(chain_id: u64) -> Vec<([u8; 20], u128)> {
    let _ = chain_id;
    let total: u128 = 71_000_000_000u128 * AIC;
    let treasury: [u8; 20] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0x0F];
    let airdrop: [u8; 20] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0x0A];
    let staking: [u8; 20] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0x05];
    let team: [u8; 20] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0x01];
    let dev: [u8; 20] = [
        0xf3, 0x9F, 0xd6, 0xe5, 0x1a, 0xad, 0x88, 0xF6,
        0xF4, 0xce, 0x6a, 0xB8, 0x82, 0x72, 0x79, 0xcf,
        0xff, 0xb9, 0x22, 0x66,
    ];
    let dev_balance: u128 = 1_000_000u128 * AIC;
    vec![
        (treasury, total / 100 * 40),
        (airdrop,  total / 100 * 30),
        (staking,  total / 100 * 20),
        (team,     total / 100 * 10),
        (dev,      dev_balance),
    ]
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let args = Args::parse();
    let rpc_addr = format!("0.0.0.0:{}", args.rpc_port);

    tracing::info!("Starting AIC Node");
    tracing::info!("Chain ID: {}", args.chain_id);
    tracing::info!("RPC: http://{}", rpc_addr);
    tracing::info!("P2P: port {}", args.p2p_port);
    tracing::info!("Mining: {}", if args.no_mine { "disabled" } else { "enabled" });

    // === Chain: создаём/загружаем ОДИН РАЗ ===
    let alloc = build_alloc(args.chain_id);
    std::fs::create_dir_all("data").ok();
    let chain_path = std::path::PathBuf::from("data").join("chain.json");
    let chain = chain::init_or_genesis(args.chain_id, &alloc, &chain_path)?;
    let chain_arc = Arc::new(RwLock::new(chain));

    // === P2P (получает shared chain) ===
    let p2p_handles = p2p::start_p2p(args.p2p_port, args.bootnodes, chain_arc.clone()).await?;

    // === RPC (получает shared chain + P2P handles) ===
    rpc::run_rpc(&rpc_addr, args.chain_id, p2p_handles, args.no_mine, chain_arc).await?;
    Ok(())
}
