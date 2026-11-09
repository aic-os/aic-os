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
fn factory_creates_all_chains_from_full_settings() {
    // With a complete settings.yaml every Chain::ALL should produce a provider.
    let path = {
        let mut p = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        p.pop(); p.pop(); p.push("config"); p.push("settings.yaml"); p
    };
    let s = aic_core::Settings::from_yaml(path).expect("settings");
    let f = ProviderFactory::from_settings(&s);
    let mut created = 0usize;
    for &c in aic_core::chain::Chain::ALL {
        if f.create(c).is_ok() { created += 1; }
    }
    assert!(created >= 100, "expected >=100 chains wired, got {created}");
}

#[test]
fn factory_creates_bitcoin_provider() {
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
fn factory_creates_solana_provider() {
    let mut settings = HashMap::new();
    settings.insert(Chain::Solana, ChainSettings {
        rpc_urls: vec!["https://api.mainnet-beta.solana.com".to_string()],
        chain_id: None,
    });
    let f = ProviderFactory::with_settings(settings);
    let p = f.create(Chain::Solana).expect("solana provider");
    assert_eq!(p.chain(), Chain::Solana);
}

#[test]
fn factory_creates_cosmos_provider() {
    let mut settings = HashMap::new();
    settings.insert(Chain::CosmosHub, ChainSettings {
        rpc_urls: vec!["https://cosmos-rest.publicnode.com".to_string()],
        chain_id: None,
    });
    let f = ProviderFactory::with_settings(settings);
    let p = f.create(Chain::CosmosHub).expect("cosmos provider");
    assert_eq!(p.chain(), Chain::CosmosHub);
}

#[test]
fn factory_creates_tron_provider() {
    let mut settings = HashMap::new();
    settings.insert(Chain::Tron, ChainSettings {
        rpc_urls: vec!["https://api.trongrid.io".to_string()],
        chain_id: None,
    });
    let f = ProviderFactory::with_settings(settings);
    let p = f.create(Chain::Tron).expect("tron provider");
    assert_eq!(p.chain(), Chain::Tron);
}

#[test]
fn factory_creates_ripple_provider() {
    let mut settings = HashMap::new();
    settings.insert(Chain::Ripple, ChainSettings {
        rpc_urls: vec!["https://xrplcluster.com".to_string()],
        chain_id: None,
    });
    let f = ProviderFactory::with_settings(settings);
    let p = f.create(Chain::Ripple).expect("ripple provider");
    assert_eq!(p.chain(), Chain::Ripple);
}

#[test]
fn factory_creates_sui_provider() {
    let mut settings = HashMap::new();
    settings.insert(Chain::Sui, ChainSettings {
        rpc_urls: vec!["https://fullnode.mainnet.sui.io:443".to_string()],
        chain_id: None,
    });
    let f = ProviderFactory::with_settings(settings);
    let p = f.create(Chain::Sui).expect("sui provider");
    assert_eq!(p.chain(), Chain::Sui);
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

