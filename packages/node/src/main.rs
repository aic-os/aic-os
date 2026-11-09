mod block;
mod chain;
mod mempool;
mod rpc;
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

    /// Chain ID (по умолчанию 2026 -- год запуска AIC Chain)
    #[arg(long, default_value_t = 2026)]
    chain_id: u64,
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
    let addr = format!("0.0.0.0:{}", args.rpc_port);

    tracing::info!("Starting AIC Node");
    tracing::info!("Chain ID: {}", args.chain_id);
    tracing::info!("RPC: http://{}", addr);

    rpc::run_rpc(&addr, args.chain_id).await?;
    Ok(())
}

