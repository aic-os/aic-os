use aic_core::wallet::{EthAccount, LegacyTransaction};
use primitive_types::{H160, U256};
use sha3::{Digest, Keccak256};

// EIP-155 official example:
// nonce=9, gasPrice=20 gwei, gasLimit=21000,
// to=0x3535...35, value=1e18, data='', chain_id=1
// privkey=0x4646...46
// expected signing hash: 0xdaf5a779ae972f972197303d7b574746c7ef83eadac0f2791ad23db92e4c8e53
#[test]
fn eip155_spec_signing_hash_and_v() {
    let tx = LegacyTransaction {
        nonce: 9,
        gas_price: U256::from(20_000_000_000u64),
        gas_limit: 21_000,
        to: H160::from_slice(
            &hex::decode("3535353535353535353535353535353535353535").unwrap(),
        ),
        value: U256::from(1_000_000_000_000_000_000u64),
        data: Vec::new(),
        chain_id: 1,
    };

    let signing_hash = tx.signing_hash();
    assert_eq!(
        hex::encode(signing_hash),
        "daf5a779ae972f972197303d7b574746c7ef83eadac0f2791ad23db92e4c8e53"
    );

    let mut privkey = [0u8; 32];
    for b in privkey.iter_mut() { *b = 0x46; }
    let account = EthAccount {
        address: String::new(),
        private_key: privkey,
        public_key: [0u8; 33],
    };

    let signed = tx.sign(&account).expect("sign");
    let raw_hex = signed.raw_hex();
    assert!(
        raw_hex.starts_with("0xf86c098504a817c8008252089435353535353535353535353535353535353535"),
        "got: {raw_hex}"
    );
}

#[test]
fn signed_tx_hash_matches_keccak_of_raw() {
    let tx = LegacyTransaction {
        nonce: 0,
        gas_price: U256::from(1_000_000_000u64),
        gas_limit: 21_000,
        to: H160::from_low_u64_be(0xdead),
        value: U256::from(1u64),
        data: Vec::new(),
        chain_id: 1,
    };
    let mut privkey = [0u8; 32];
    privkey[31] = 1;
    let account = EthAccount {
        address: String::new(),
        private_key: privkey,
        public_key: [0u8; 33],
    };
    let signed = tx.sign(&account).expect("sign");
    let from_raw = Keccak256::digest(&signed.raw);
    assert_eq!(&from_raw[..], &signed.hash[..]);
}
