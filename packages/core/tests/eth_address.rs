use aic_core::wallet::{derive_account, to_eip55, Mnemonic};

// Standard Hardhat/Ganache test mnemonic -- NEVER use with real funds.
const HARDHAT_MNEMONIC: &str =
    "test test test test test test test test test test test junk";

#[test]
fn hardhat_account_0_matches_known_vectors() {
    let m = Mnemonic::from_phrase(HARDHAT_MNEMONIC).expect("parse");
    let acc = derive_account(&m, 0).expect("derive");

    assert_eq!(acc.address, "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266");
    assert_eq!(
        acc.private_key_hex(),
        "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"
    );
}

#[test]
fn hardhat_account_1_matches_known_vector() {
    let m = Mnemonic::from_phrase(HARDHAT_MNEMONIC).expect("parse");
    let acc = derive_account(&m, 1).expect("derive");
    assert_eq!(acc.address, "0x70997970C51812dc3A010C7d01b50e0d17dc79C8");
}

#[test]
fn accounts_at_different_indices_are_distinct() {
    let m = Mnemonic::from_phrase(HARDHAT_MNEMONIC).expect("parse");
    let a0 = derive_account(&m, 0).expect("a0");
    let a1 = derive_account(&m, 1).expect("a1");
    assert_ne!(a0.address, a1.address);
    assert_ne!(a0.private_key, a1.private_key);
}

#[test]
fn same_input_gives_same_account() {
    let m = Mnemonic::from_phrase(HARDHAT_MNEMONIC).expect("parse");
    let a = derive_account(&m, 5).expect("a");
    let b = derive_account(&m, 5).expect("b");
    assert_eq!(a.address, b.address);
    assert_eq!(a.private_key, b.private_key);
}

#[test]
fn pubkey_is_33_bytes_compressed() {
    let m = Mnemonic::from_phrase(HARDHAT_MNEMONIC).expect("parse");
    let acc = derive_account(&m, 0).expect("derive");
    assert_eq!(acc.public_key.len(), 33);
    // Compressed secp256k1: prefix 0x02 or 0x03.
    assert!(acc.public_key[0] == 0x02 || acc.public_key[0] == 0x03);
}

#[test]
fn eip55_spec_vector_1() {
    // From EIP-55 spec.
    let raw = hex::decode("5aaeb6053f3e94c9b9a09f33669435e7ef1beaed").unwrap();
    let mut addr = [0u8; 20];
    addr.copy_from_slice(&raw);
    assert_eq!(to_eip55(&addr), "0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed");
}

#[test]
fn eip55_spec_vector_2() {
    let raw = hex::decode("fb6916095ca1df60bb79ce92ce3ea74c37c5d359").unwrap();
    let mut addr = [0u8; 20];
    addr.copy_from_slice(&raw);
    assert_eq!(to_eip55(&addr), "0xfB6916095ca1df60bB79Ce92cE3Ea74c37c5d359");
}

#[test]
fn eip55_spec_vector_3() {
    let raw = hex::decode("dbf03b407c01e7cd3cbea99509d93f8dddc8c6fb").unwrap();
    let mut addr = [0u8; 20];
    addr.copy_from_slice(&raw);
    assert_eq!(to_eip55(&addr), "0xdbF03B407c01E7cD3CBea99509d93f8DDDC8C6FB");
}
