//! High-level helpers to build ERC-20 transfer transactions.

use crate::chain::Chain;
use crate::error::Result;
use crate::providers::evm_abi;
use crate::wallet::eth_tx::LegacyTransaction;
use crate::wallet::EthAccount;
use crate::ChainTraits;
use primitive_types::H160;
use std::sync::Arc;

/// Build an unsigned legacy transaction that calls `transfer(to, amount)`
/// on an ERC-20 contract. Value = 0 (ETH), data = ABI calldata.
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

    let contract_bytes = hex::decode(
        token_contract.trim_start_matches("0x"),
    ).map_err(|e| crate::CoreError::Rpc(format!("token address hex: {e}")))?;
    if contract_bytes.len() != 20 {
        return Err(crate::CoreError::Rpc("token contract must be 20 bytes".into()));
    }
    let to = H160::from_slice(&contract_bytes);

    let chain_id = match chain {
        Chain::Ethereum => 1u64,
        Chain::Bsc => 56,
        Chain::Polygon => 137,
        Chain::Arbitrum => 42161,
        Chain::Optimism => 10,
        Chain::Base => 8453,
        Chain::Avalanche => 43114,
        _ => 1,
    };

    Ok(LegacyTransaction {
        nonce,
        gas_price: primitive_types::U256::from(gas_price),
        gas_limit,
        to,
        value: primitive_types::U256::zero(),
        data: calldata,
        chain_id,
    })
}
