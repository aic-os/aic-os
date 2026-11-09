//! Ethereum key derivation (BIP-32 + secp256k1 + EIP-55 checksum).

use bip32::{DerivationPath, XPrv};
use k256::ecdsa::{SigningKey, VerifyingKey};
use sha3::{Digest, Keccak256};
use std::str::FromStr;
use zeroize::Zeroize;

use crate::error::{CoreError, Result};
use crate::wallet::Mnemonic;

/// Default BIP-44 path prefix for Ethereum accounts.
pub const ETH_DEFAULT_PATH: &str = "m/44'/60'/0'/0";

/// Derived Ethereum account: EIP-55 address, compressed pubkey, private key.
///
/// The private key is zeroized on drop.
pub struct EthAccount {
    pub address: String,
    pub private_key: [u8; 32],
    pub public_key: [u8; 33],
}

impl Drop for EthAccount {
    fn drop(&mut self) {
        self.private_key.zeroize();
    }
}

impl EthAccount {
    /// Hex-encoded private key (0x-prefixed). Useful for export; callers are
    /// responsible for not logging it.
    pub fn private_key_hex(&self) -> String {
        format!("0x{}", hex::encode(self.private_key))
    }
}

/// Derive an Ethereum account from a mnemonic at a given index.
/// Path: m/44'/60'/0'/0/{index}.
pub fn derive_account(mnemonic: &Mnemonic, index: u32) -> Result<EthAccount> {
    let mut seed = mnemonic.to_seed("");

    let path_str = format!("{}/{index}", ETH_DEFAULT_PATH);
    let path = DerivationPath::from_str(&path_str)
        .map_err(|e| CoreError::Wallet(format!("bad derivation path: {e}")))?;

    let xprv = XPrv::derive_from_path(seed.as_slice(), &path)
        .map_err(|e| CoreError::Wallet(format!("bip32 derivation: {e}")))?;
    seed.zeroize();

    let mut priv_bytes = xprv.private_key().to_bytes();
    let mut private_key = [0u8; 32];
    private_key.copy_from_slice(&priv_bytes);
    priv_bytes.zeroize();

    let signing_key = SigningKey::from_bytes((&private_key).into())
        .map_err(|e| CoreError::Wallet(format!("secp256k1 key: {e}")))?;

    let public_key = signing_key.verifying_key();
    let compressed = public_key.to_encoded_point(true);
    let mut public_key_arr = [0u8; 33];
    public_key_arr.copy_from_slice(compressed.as_bytes());

    let address = pubkey_to_eip55(&public_key_arr)?;

    Ok(EthAccount {
        address,
        private_key,
        public_key: public_key_arr,
    })
}

/// Convert a compressed secp256k1 pubkey to an EIP-55 checksummed address.
pub fn pubkey_to_eip55(pubkey: &[u8; 33]) -> Result<String> {
    let vk = VerifyingKey::from_sec1_bytes(pubkey)
        .map_err(|e| CoreError::Wallet(format!("verifying key: {e}")))?;
    let uncompressed = vk.to_encoded_point(false); // 65 bytes, 0x04 prefix
    let bytes = uncompressed.as_bytes();
    let without_prefix = &bytes[1..]; // 64 bytes

    let mut hasher = Keccak256::new();
    hasher.update(without_prefix);
    let hash = hasher.finalize();

    let mut addr = [0u8; 20];
    addr.copy_from_slice(&hash[12..]);
    Ok(to_eip55(&addr))
}

/// Apply EIP-55 mixed-case checksum to a 20-byte address.
pub fn to_eip55(addr: &[u8; 20]) -> String {
    let hex_addr = hex::encode(addr);
    let mut hasher = Keccak256::new();
    hasher.update(hex_addr.as_bytes());
    let hash_hex = hex::encode(hasher.finalize());

    let mut out = String::with_capacity(42);
    out.push_str("0x");
    for (i, ch) in hex_addr.chars().enumerate() {
        if ch.is_ascii_digit() {
            out.push(ch);
        } else {
            let nibble = u8::from_str_radix(&hash_hex[i..=i], 16).unwrap_or(0);
            if nibble >= 8 {
                out.push(ch.to_ascii_uppercase());
            } else {
                out.push(ch);
            }
        }
    }
    out
}
