use std::collections::HashMap;

use aic_core::chain::Chain;
use aic_core::providers::factory::ChainSettings;
use aic_core::ProviderFactory;

#[test]
fn factory_rejects_unknown_chain() {
    let f = ProviderFactory::new();
    assert!(f.create(Chain::Ethereum).is_err());
}

#[test]
fn factory_creates_evm_provider_with_one_rpc() {
    let mut settings = HashMap::new();
    settings.insert(Chain::Ethereum, ChainSettings {
        rpc_urls: vec!["https://example.invalid".to_string()],
        chain_id: Some(1),
    });
    let f = ProviderFactory::with_settings(settings);
    let p = f.create(Chain::Ethereum).expect("evm provider");
    assert_eq!(p.chain(), Chain::Ethereum);
}

#[test]
fn factory_creates_evm_provider_with_multiple_rpc() {
    let mut settings = HashMap::new();
    settings.insert(Chain::Polygon, ChainSettings {
        rpc_urls: vec![
            "https://primary.invalid".to_string(),
            "https://fallback.invalid".to_string(),
        ],
        chain_id: Some(137),
    });
    let f = ProviderFactory::with_settings(settings);
    let p = f.create(Chain::Polygon).expect("evm provider");
    assert_eq!(p.chain(), Chain::Polygon);
}

#[test]
fn factory_rejects_not_implemented_cosmos() {
    let mut settings = HashMap::new();
    settings.insert(Chain::CosmosHub, ChainSettings {
        rpc_urls: vec!["http://localhost:26657".to_string()],
        chain_id: None,
    });
    let f = ProviderFactory::with_settings(settings);
    let err = f.create(Chain::CosmosHub).err().expect("should err");
    assert!(format!("{err}").contains("not implemented"));
}

#[test]
fn factory_creates_bitcoin_provider_now_that_utxo_is_implemented() {
    let mut settings = HashMap::new();
    settings.insert(Chain::Bitcoin, ChainSettings {
        rpc_urls: vec!["https://blockstream.info/api".to_string()],
        chain_id: None,
    });
    let f = ProviderFactory::with_settings(settings);
    let p = f.create(Chain::Bitcoin).expect("bitcoin provider");
    assert_eq!(p.chain(), Chain::Bitcoin);
}

#[test]
fn factory_from_settings_skips_chains_without_rpc() {
    let yaml = r#"
chains:
  ethereum:
    chain_id: 1
    rpc:
      - https://eth.invalid
"#;
    let s: aic_core::Settings = serde_yaml_ng::from_str(yaml).unwrap();
    let f = ProviderFactory::from_settings(&s);
    assert!(f.create(Chain::Ethereum).is_ok());
    assert!(f.create(Chain::Polygon).is_err());
}
