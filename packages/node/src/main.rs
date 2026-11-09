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

use clap::Parser;

/// AIC Chain Node
#[derive(Parser, Debug)]
#[command(name = "aic-node", version, about = "AIC Chain full node")]
struct Args {
    /// Порт HTTP RPC (по умолчанию 8545)
    #[arg(long, default_value = "8545")]
    rpc_port: u16,

    /// Порт P2P (по умолчанию 30333)
    #[arg(long, default_value = "30333")]
    p2p_port: u16,

    /// Chain ID (по умолчанию 2026)
    #[arg(long, default_value_t = 2026)]
    chain_id: u64,

    /// Bootnode адреса (Multiaddr через запятую)
    #[arg(long, value_delimiter = ',')]
    bootnodes: Vec<String>,

    /// Не майнить блоки (sync-only режим)
    #[arg(long, default_value_t = false)]
    no_mine: bool,
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
    if !args.bootnodes.is_empty() {
        tracing::info!("Bootnodes: {:?}", args.bootnodes);
    }

    let p2p_handles = p2p::start_p2p(args.p2p_port, args.bootnodes).await?;
    rpc::run_rpc(&rpc_addr, args.chain_id, p2p_handles, args.no_mine).await?;
    Ok(())
}
