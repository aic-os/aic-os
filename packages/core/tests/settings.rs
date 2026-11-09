use aic_core::chain::{Chain, ChainType};
use aic_core::Settings;

fn settings_path() -> std::path::PathBuf {
    // CARGO_MANIFEST_DIR = .../packages/core
    // workspace root  = .../  -> ../../config/settings.yaml
    let mut p = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.pop(); // -> packages
    p.pop(); // -> workspace root
    p.push("config");
    p.push("settings.yaml");
    p
}

#[test]
fn chain_type_classification() {
    assert_eq!(Chain::Ethereum.chain_type(), ChainType::Evm);
    assert_eq!(Chain::Bsc.chain_type(), ChainType::Evm);
    assert_eq!(Chain::Bitcoin.chain_type(), ChainType::Utxo);
    assert_eq!(Chain::CosmosHub.chain_type(), ChainType::Cosmos);
    assert_eq!(Chain::Solana.chain_type(), ChainType::Other);
}

#[test]
fn chain_name_roundtrip() {
    for c in [Chain::Ethereum, Chain::Bitcoin, Chain::Solana, Chain::Ton, Chain::Polygon] {
        assert_eq!(Chain::from_name(c.as_str()), Some(c));
    }
}

#[test]
fn chain_alias_lookup() {
    assert_eq!(Chain::from_name("BTC"), Some(Chain::Bitcoin));
    assert_eq!(Chain::from_name("  eth "), Some(Chain::Ethereum));
    assert_eq!(Chain::from_name("MATIC"), Some(Chain::Polygon));
    assert_eq!(Chain::from_name("unknowncoin"), None);
}

#[test]
fn settings_loads_from_yaml() {
    let s = Settings::from_yaml(settings_path()).expect("load settings.yaml");
    let eth_rpc = s.rpc_urls_for(Chain::Ethereum).expect("eth rpc");
    assert!(!eth_rpc.is_empty());
    assert_eq!(s.chain_id_for(Chain::Ethereum), Some(1));
    assert_eq!(s.chain_id_for(Chain::Polygon), Some(137));
}

#[test]
fn settings_unknown_chain_returns_none() {
    let s = Settings::from_yaml(settings_path()).expect("load settings.yaml");
    assert_eq!(s.rpc_urls_for(Chain::Aptos), None);
}

#[test]
fn settings_parses_inline_yaml() {
    let yaml = r#"
chains:
  ethereum:
    chain_id: 1
    rpc:
      - https://a.example
      - https://b.example
"#;
    let s: Settings = serde_yaml_ng::from_str(yaml).expect("parse");
    let rpc = s.rpc_urls_for(Chain::Ethereum).expect("eth");
    assert_eq!(rpc.len(), 2);
    assert_eq!(rpc[0], "https://a.example");
}


