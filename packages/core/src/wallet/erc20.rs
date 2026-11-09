//! High-level helpers to build ERC-20 transfer transactions.

use crate::chain::Chain;
use crate::error::Result;
use crate::providers::evm_abi;
use crate::wallet::eth_tx::LegacyTransaction;
use crate::wallet::EthAccount;
use crate::ChainTraits;
use primitive_types::H160;
use std::sync::Arc;

pub fn chain_id_for(chain: Chain) -> u64 {
    match chain {
        Chain::Ethereum => 1, Chain::Bsc => 56, Chain::Polygon => 137,
        Chain::Arbitrum => 42161, Chain::Optimism => 10, Chain::Base => 8453,
        Chain::Avalanche => 43114, Chain::Linea => 59144, Chain::Scroll => 534352,
        Chain::ZkSync => 324, Chain::Mantle => 5000, Chain::Blast => 81457,
        Chain::Gnosis => 100, Chain::Celo => 42220, Chain::Fantom => 250,
        Chain::Cronos => 25, Chain::Moonbeam => 1284, Chain::Aurora => 1313161554,
        Chain::Filecoin => 314, Chain::PolygonZkEvm => 1101,
        _ => 1,
    }
}

pub async fn build_transfer_tx(
    provider: &Arc<dyn ChainTraits>,
    account: &EthAccount,
    token_contract: &str,
    recipient: &str,
    amount: u128,
    gas_limit: u64,
    chain: Chain,
) -> Result<LegacyTransaction> {
    let nonce = provider.get_nonce(&account.address).await? as u64;
    let gas_price = provider.get_gas_price().await?;

    let calldata = evm_abi::encode_transfer_calldata(recipient, amount)
        .map_err(crate::CoreError::Rpc)?;

    let contract_bytes = hex::decode(token_contract.trim_start_matches("0x"))
        .map_err(|e| crate::CoreError::Rpc(format!("token address hex: {e}")))?;
    if contract_bytes.len() != 20 {
        return Err(crate::CoreError::Rpc("token contract must be 20 bytes".into()));
    }
    let to = H160::from_slice(&contract_bytes);

    Ok(LegacyTransaction {
        nonce,
        gas_price: primitive_types::U256::from(gas_price),
        gas_limit,
        to,
        value: primitive_types::U256::zero(),
        data: calldata,
        chain_id: chain_id_for(chain),
    })
}
