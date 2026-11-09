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

/// Build calldata for ERC-20 `transfer(address,uint256)`.
pub fn encode_transfer_calldata(to: &str, amount: u128) -> Result<Vec<u8>, String> {
    let sel = selector("transfer(address,uint256)");
    let addr = encode_address(to)?;
    let amt = encode_u256(amount);
    let mut out = Vec::with_capacity(4 + 32 + 32);
    out.extend_from_slice(&sel);
    out.extend_from_slice(&addr);
    out.extend_from_slice(&amt);
    Ok(out)
}

/// Decode a 0x-prefixed 32-byte hex return value into u128.
pub fn decode_u128(hex_data: &str) -> Result<u128, String> {
    let s = hex_data.strip_prefix("0x").ok_or("expected 0x prefix")?;
    if s.len() != 64 {
        return Err(format!("expected 32-byte return, got {} hex chars", s.len()));
    }
    let bytes = hex::decode(s).map_err(|e| format!("hex decode: {e}"))?;
    if bytes[..16].iter().any(|&b| b != 0) {
        return Err("value exceeds u128".into());
    }
    let mut buf = [0u8; 16];
    buf.copy_from_slice(&bytes[16..]);
    Ok(u128::from_be_bytes(buf))
}

/// Decode a 32-byte ABI return value into u8 (e.g. `decimals()`).
pub fn decode_uint8(hex_data: &str) -> Result<u8, String> {
    let s = hex_data.strip_prefix("0x").ok_or("expected 0x prefix")?;
    if s.len() != 64 {
        return Err(format!("expected 32-byte return, got {} hex chars", s.len()));
    }
    let bytes = hex::decode(s).map_err(|e| format!("hex decode: {e}"))?;
    if bytes[..31].iter().any(|&b| b != 0) {
        return Err("value exceeds u8".into());
    }
    Ok(bytes[31])
}

/// Decode a standard ABI-encoded string return value (e.g. `symbol()`, `name()`).
/// Layout: [32-byte offset=0x20][32-byte length][length bytes of UTF-8, padded].
pub fn decode_string(hex_data: &str) -> Result<String, String> {
    let s = hex_data.strip_prefix("0x").ok_or("expected 0x prefix")?;
    let bytes = hex::decode(s).map_err(|e| format!("hex decode: {e}"))?;
    if bytes.len() < 64 {
        return Err(format!("ABI string needs at least 64 bytes, got {}", bytes.len()));
    }

    // First 32 bytes: offset -- expect 0x20 (32).
    if bytes[..31].iter().any(|&b| b != 0) || bytes[31] != 0x20 {
        return Err(format!(
            "unexpected ABI offset (expected 0x20): {}",
            hex::encode(&bytes[..32])
        ));
    }

    // Next 32 bytes: length.
    if bytes[32..56].iter().any(|&b| b != 0) {
        return Err("string length too large".into());
    }
    let mut lb = [0u8; 8];
    lb.copy_from_slice(&bytes[56..64]);
    let len = u64::from_be_bytes(lb) as usize;

    if bytes.len() < 64 + len {
        return Err(format!(
            "string data truncated: need {} bytes, have {}",
            64 + len,
            bytes.len()
        ));
    }
    let data = &bytes[64..64 + len];
    String::from_utf8(data.to_vec()).map_err(|e| format!("string not UTF-8: {e}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selector_balance_of() {
        assert_eq!(selector("balanceOf(address)"), [0x70, 0xa0, 0x82, 0x31]);
    }

    #[test]
    fn selector_transfer() {
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
        assert!(encoded[..12].iter().all(|&b| b == 0));
        assert_eq!(hex::encode(&encoded[12..]), "d8da6bf26964af9d7eed9e03e53415d37aa96045");
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
        assert_eq!(&e[29..], &[0x0F, 0x42, 0x40]);
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

    #[test]
    fn uint8_decode_known() {
        let hex = "0x0000000000000000000000000000000000000000000000000000000000000006";
        assert_eq!(decode_uint8(hex).unwrap(), 6);
    }

    #[test]
    fn uint8_decode_rejects_overflow() {
        let hex = "0x0000000000000000000000000000000000000000000000000000000000000106";
        assert!(decode_uint8(hex).is_err());
    }

    #[test]
    fn string_decode_short() {
        // "USDC"
        let hex = "0x\
0000000000000000000000000000000000000000000000000000000000000020\
0000000000000000000000000000000000000000000000000000000000000004\
5553444300000000000000000000000000000000000000000000000000000000";
        assert_eq!(decode_string(hex).unwrap(), "USDC");
    }

    #[test]
    fn string_decode_long() {
        // "USD Coin" (8 bytes)
        let hex = "0x\
0000000000000000000000000000000000000000000000000000000000000020\
0000000000000000000000000000000000000000000000000000000000000008\
55534420436f696e000000000000000000000000000000000000000000000000";
        assert_eq!(decode_string(hex).unwrap(), "USD Coin");
    }

    #[test]
    fn string_decode_rejects_bad_offset() {
        let hex = "0x\
0000000000000000000000000000000000000000000000000000000000000040\
0000000000000000000000000000000000000000000000000000000000000004\
5553444300000000000000000000000000000000000000000000000000000000";
        assert!(decode_string(hex).is_err());
    }

    #[test]
    fn transfer_calldata_known() {
        let to = "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045";
        let data = encode_transfer_calldata(to, 1_000_000).unwrap();
        let hex = hex::encode(&data);
        let expected = "a9059cbb\
000000000000000000000000d8da6bf26964af9d7eed9e03e53415d37aa96045\
00000000000000000000000000000000000000000000000000000000000f4240";
        assert_eq!(hex, expected);
    }
}
