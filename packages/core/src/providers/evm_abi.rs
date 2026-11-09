//! Minimal Ethereum ABI helpers: function selectors, address/u256 encoding,
//! and decoding of simple return values.

use sha3::{Digest, Keccak256};

/// First 4 bytes of keccak256(signature) -- the standard EVM function selector.
pub fn selector(signature: &str) -> [u8; 4] {
    let hash = Keccak256::digest(signature.as_bytes());
    let mut s = [0u8; 4];
    s.copy_from_slice(&hash[..4]);
    s
}

/// ABI-encode an address argument: 32 bytes, left-padded with zeros.
/// Input: 0x-prefixed 40-hex address string.
pub fn encode_address(addr: &str) -> Result<[u8; 32], String> {
    let hex_part = addr.strip_prefix("0x").ok_or("address must be 0x-prefixed")?;
    if hex_part.len() != 40 {
        return Err(format!("address must be 40 hex chars, got {}", hex_part.len()));
    }
    let bytes = hex::decode(hex_part).map_err(|e| format!("address hex decode: {e}"))?;
    let mut out = [0u8; 32];
    out[12..].copy_from_slice(&bytes);
    Ok(out)
}

/// ABI-encode a uint256 argument: 32 bytes big-endian.
pub fn encode_u256(value: u128) -> [u8; 32] {
    let mut out = [0u8; 32];
    out[16..].copy_from_slice(&value.to_be_bytes());
    out
}

/// Decode a 0x-prefixed 32-byte hex return value into u128.
/// Errors if the value doesn't fit in u128 (> 2^128 - 1).
pub fn decode_u128(hex_data: &str) -> Result<u128, String> {
    let s = hex_data.strip_prefix("0x").ok_or("expected 0x prefix")?;
    if s.len() != 64 {
        return Err(format!("expected 32-byte return, got {} hex chars", s.len()));
    }
    let bytes = hex::decode(s).map_err(|e| format!("hex decode: {e}"))?;
    // High 16 bytes must be zero for u128 fit.
    if bytes[..16].iter().any(|&b| b != 0) {
        return Err("value exceeds u128".into());
    }
    let mut buf = [0u8; 16];
    buf.copy_from_slice(&bytes[16..]);
    Ok(u128::from_be_bytes(buf))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selector_balance_of() {
        // Well-known: balanceOf(address) = 0x70a08231
        assert_eq!(selector("balanceOf(address)"), [0x70, 0xa0, 0x82, 0x31]);
    }

    #[test]
    fn selector_transfer() {
        // Well-known: transfer(address,uint256) = 0xa9059cbb
        assert_eq!(selector("transfer(address,uint256)"), [0xa9, 0x05, 0x9c, 0xbb]);
    }

    #[test]
    fn selector_decimals() {
        assert_eq!(selector("decimals()"), [0x31, 0x3c, 0xe5, 0x67]);
    }

    #[test]
    fn selector_symbol() {
        assert_eq!(selector("symbol()"), [0x95, 0xd8, 0x9b, 0x41]);
    }

    #[test]
    fn address_encode_known() {
        let encoded = encode_address("0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045").unwrap();
        // First 12 bytes zero, then 20 bytes of address.
        assert!(encoded[..12].iter().all(|&b| b == 0));
        assert_eq!(
            hex::encode(&encoded[12..]),
            "d8da6bf26964af9d7eed9e03e53415d37aa96045"
        );
    }

    #[test]
    fn address_encode_rejects_bad_input() {
        assert!(encode_address("0x123").is_err());
        assert!(encode_address("d8da6bf26964af9d7eed9e03e53415d37aa96045").is_err());
    }

    #[test]
fn u256_encode_small_value() {
    let e = encode_u256(1_000_000);
    assert_eq!(e[..29], [0u8; 29]);
    assert_eq!(&e[29..], &[0x0F, 0x42, 0x40]); // 1_000_000 == 0x0F4240
}

    #[test]
    fn u128_decode_roundtrip() {
        let encoded = format!("0x{:064x}", 1_234_567u128);
        assert_eq!(decode_u128(&encoded).unwrap(), 1_234_567);
    }

    #[test]
    fn u128_decode_rejects_overflow() {
        let too_big = format!("0x{}", "f".repeat(64));
        assert!(decode_u128(&too_big).is_err());
    }
}

