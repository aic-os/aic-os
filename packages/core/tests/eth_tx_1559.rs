use aic_core::wallet::{
    derive_account, pubkey_to_eip55, Eip1559Transaction, Mnemonic, H160, U256,
};
use k256::ecdsa::{RecoveryId, Signature, VerifyingKey};
use sha3::{Digest, Keccak256};

const HARDHAT: &str = "test test test test test test test test test test test junk";

fn sample_tx() -> Eip1559Transaction {
    Eip1559Transaction {
        chain_id: 1,
        nonce: 0,
        max_priority_fee_per_gas: U256::from(1_500_000_000u64),
        max_fee_per_gas: U256::from(30_000_000_000u64),
        gas_limit: 21_000,
        to: H160::from_low_u64_be(0xdead),
        value: U256::from(1_000_000_000_000_000u64),
        data: Vec::new(),
    }
}

#[test]
fn eip1559_sign_and_recover() {
    let m = Mnemonic::from_phrase(HARDHAT).expect("mnemonic");
    let account = derive_account(&m, 0).expect("derive");
    let tx = sample_tx();
    let hash = tx.signing_hash();
    let signed = tx.sign(&account).expect("sign");

    let mut sig_bytes = [0u8; 64];
    sig_bytes[..32].copy_from_slice(&signed.r);
    sig_bytes[32..].copy_from_slice(&signed.s);
    let sig = Signature::from_slice(&sig_bytes).expect("sig");
    let rec_id = RecoveryId::from_byte(signed.y_parity).expect("rec_id");
    let vk = VerifyingKey::recover_from_prehash(&hash, &sig, rec_id).expect("recover");
    let compressed = vk.to_encoded_point(true);
    let mut pk = [0u8; 33];
    pk.copy_from_slice(compressed.as_bytes());
    let recovered = pubkey_to_eip55(&pk).expect("addr");
    assert_eq!(recovered, account.address);
}

#[test]
fn eip1559_raw_has_02_prefix() {
    let m = Mnemonic::from_phrase(HARDHAT).unwrap();
    let account = derive_account(&m, 0).unwrap();
    let signed = sample_tx().sign(&account).unwrap();
    assert_eq!(signed.raw[0], 0x02);
}

#[test]
fn eip1559_hash_is_keccak_of_raw() {
    let m = Mnemonic::from_phrase(HARDHAT).unwrap();
    let account = derive_account(&m, 0).unwrap();
    let signed = sample_tx().sign(&account).unwrap();
    let expected = Keccak256::digest(&signed.raw);
    assert_eq!(&expected[..], &signed.hash[..]);
}

#[test]
fn eip1559_y_parity_is_0_or_1() {
    let m = Mnemonic::from_phrase(HARDHAT).unwrap();
    let account = derive_account(&m, 0).unwrap();
    let signed = sample_tx().sign(&account).unwrap();
    assert!(signed.y_parity == 0 || signed.y_parity == 1);
}

#[test]
fn eip1559_deterministic_for_same_input() {
    let m = Mnemonic::from_phrase(HARDHAT).unwrap();
    let account = derive_account(&m, 0).unwrap();
    let tx = sample_tx();
    let a = tx.sign(&account).unwrap();
    let b = tx.sign(&account).unwrap();
    assert_eq!(a.raw, b.raw);
    assert_eq!(a.hash, b.hash);
}
