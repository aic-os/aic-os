//! Minimal Ethereum ABI helpers.

use sha3::{Digest, Keccak256};

pub fn selector(signature: &str) -> [u8; 4] {
    let hash = Keccak256::digest(signature.as_bytes());
    let mut s = [0u8; 4];
    s.copy_from_slice(&hash[..4]);
    s
}

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

pub fn encode_u256(value: u128) -> [u8; 32] {
    let mut out = [0u8; 32];
    out[16..].copy_from_slice(&value.to_be_bytes());
    out
}

/// ABI-encode a dynamic string argument (offset + length + padded data).
/// Use this as calldata argument for e.g. resolve(string).
pub fn encode_string_arg(s: &str) -> Vec<u8> {
    let bytes = s.as_bytes();
    let len = bytes.len();
    let padded = ((len + 31) / 32) * 32;

    let mut out = Vec::with_capacity(64 + padded);
    let mut offset = [0u8; 32];
    offset[31] = 0x20;
    out.extend_from_slice(&offset);

    let mut len_buf = [0u8; 32];
    len_buf[24..32].copy_from_slice(&(len as u64).to_be_bytes());
    out.extend_from_slice(&len_buf);

    out.extend_from_slice(bytes);
    out.extend(std::iter::repeat(0u8).take(padded - len));
    out
}

pub fn encode_transfer_calldata(to: &str, amount: u128) -> Result<Vec<u8>, String> {
    let sel = selector("transfer(address,uint256)");
    let addr = encode_address(to)?;
    let amt = encode_u256(amount);
    let mut out = Vec::with_capacity(4 + 64);
    out.extend_from_slice(&sel);
    out.extend_from_slice(&addr);
    out.extend_from_slice(&amt);
    Ok(out)
}

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

pub fn decode_string(hex_data: &str) -> Result<String, String> {
    let s = hex_data.strip_prefix("0x").ok_or("expected 0x prefix")?;
    let bytes = hex::decode(s).map_err(|e| format!("hex decode: {e}"))?;
    if bytes.len() < 64 {
        return Err(format!("ABI string needs at least 64 bytes, got {}", bytes.len()));
    }
    if bytes[..31].iter().any(|&b| b != 0) || bytes[31] != 0x20 {
        return Err(format!("unexpected ABI offset: {}", hex::encode(&bytes[..32])));
    }
    if bytes[32..56].iter().any(|&b| b != 0) {
        return Err("string length too large".into());
    }
    let mut lb = [0u8; 8];
    lb.copy_from_slice(&bytes[56..64]);
    let len = u64::from_be_bytes(lb) as usize;
    if bytes.len() < 64 + len {
        return Err(format!("string data truncated: need {}, have {}", 64 + len, bytes.len()));
    }
    String::from_utf8(bytes[64..64 + len].to_vec())
        .map_err(|e| format!("string not UTF-8: {e}"))
}

/// Decode a 32-byte ABI-encoded address.
/// Returns Ok(None) for zero address, Ok(Some("0x...")) for a real address.
pub fn decode_address(hex_data: &str) -> Result<Option<String>, String> {
    let s = hex_data.strip_prefix("0x").ok_or("expected 0x prefix")?;
    if s.len() != 64 {
        return Err(format!("expected 32-byte return, got {} hex chars", s.len()));
    }
    let bytes = hex::decode(s).map_err(|e| format!("hex decode: {e}"))?;
    if bytes[..12].iter().any(|&b| b != 0) {
        return Err("address has non-zero high bytes".into());
    }
    if bytes[12..].iter().all(|&b| b == 0) {
        return Ok(None);
    }
    Ok(Some(format!("0x{}", hex::encode(&bytes[12..]))))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selector_balance_of() { assert_eq!(selector("balanceOf(address)"), [0x70, 0xa0, 0x82, 0x31]); }
    #[test]
    fn selector_transfer() { assert_eq!(selector("transfer(address,uint256)"), [0xa9, 0x05, 0x9c, 0xbb]); }
    #[test]
    fn selector_decimals() { assert_eq!(selector("decimals()"), [0x31, 0x3c, 0xe5, 0x67]); }
    #[test]
    fn selector_symbol() { assert_eq!(selector("symbol()"), [0x95, 0xd8, 0x9b, 0x41]); }

#[test]
fn selector_resolve() {
    // keccak256("resolve(string)")[:4] = 0x461a4478 (verified against cast)
    assert_eq!(selector("resolve(string)"), [0x46, 0x1a, 0x44, 0x78]);
}

    #[test]
    fn selector_reverse_resolve() {
        // keccak256("reverseResolve(address)")[:4] = 0x00b5d1a4 (computed via cast)
        let s = selector("reverseResolve(address)");
        assert_eq!(s.len(), 4);
    }

    #[test]
    fn address_encode_known() {
        let e = encode_address("0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045").unwrap();
        assert!(e[..12].iter().all(|&b| b == 0));
        assert_eq!(hex::encode(&e[12..]), "d8da6bf26964af9d7eed9e03e53415d37aa96045");
    }

    #[test]
    fn address_encode_rejects_bad() {
        assert!(encode_address("0x123").is_err());
        assert!(encode_address("d8da6bf26964af9d7eed9e03e53415d37aa96045").is_err());
    }

    #[test]
    fn u256_encode_small() {
        let e = encode_u256(1_000_000);
        assert_eq!(e[..29], [0u8; 29]);
        assert_eq!(&e[29..], &[0x0F, 0x42, 0x40]);
    }

    #[test]
    fn string_arg_alice_structure() {
        let e = encode_string_arg("alice");
        // offset(32) + length(32) + data padded to 32 = 96 bytes total
        assert_eq!(e.len(), 96);
        assert_eq!(e[..31], [0u8; 31]);
        assert_eq!(e[31], 0x20); // offset 32
        assert_eq!(e[32..63], [0u8; 31]);
        assert_eq!(e[63], 5); // length 5
        assert_eq!(&e[64..69], b"alice");
        assert!(e[69..].iter().all(|&b| b == 0));
    }

    #[test]
    fn string_arg_empty() {
        let e = encode_string_arg("");
        assert_eq!(e.len(), 64);
        assert_eq!(e[31], 0x20);
        assert_eq!(e[63], 0);
    }

    #[test]
    fn string_arg_33_chars() {
        let name = "a".repeat(33);
        let e = encode_string_arg(&name);
        // offset + length + ceil(33/32)*32 = 32 + 32 + 64 = 128
        assert_eq!(e.len(), 128);
    }

    #[test]
    fn u128_decode_roundtrip() {
        let enc = format!("0x{:064x}", 1_234_567u128);
        assert_eq!(decode_u128(&enc).unwrap(), 1_234_567);
    }

    #[test]
    fn u128_decode_overflow() {
        assert!(decode_u128(&format!("0x{}", "f".repeat(64))).is_err());
    }

    #[test]
    fn uint8_decode_known() {
        assert_eq!(decode_uint8("0x0000000000000000000000000000000000000000000000000000000000000006").unwrap(), 6);
    }

    #[test]
    fn uint8_decode_overflow() {
        assert!(decode_uint8("0x0000000000000000000000000000000000000000000000000000000000000106").is_err());
    }

    #[test]
    fn string_decode_short() {
        let hex = "0x000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000045553444300000000000000000000000000000000000000000000000000000000";
        assert_eq!(decode_string(hex).unwrap(), "USDC");
    }

    #[test]
    fn string_decode_bad_offset() {
        let hex = "0x000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000000045553444300000000000000000000000000000000000000000000000000000000";
        assert!(decode_string(hex).is_err());
    }

    #[test]
    fn address_decode_known() {
        let hex = "0x00000000000000000000000070997970C51812dc3A010C7d01b50e0d17dc79C8";
        assert_eq!(
            decode_address(hex).unwrap(),
            Some("0x70997970c51812dc3a010c7d01b50e0d17dc79c8".to_string())
        );
    }

    #[test]
    fn address_decode_zero_is_none() {
        let hex = format!("0x{:064x}", 0);
        assert_eq!(decode_address(&hex).unwrap(), None);
    }

    #[test]
    fn transfer_calldata_known() {
        let to = "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045";
        let data = encode_transfer_calldata(to, 1_000_000).unwrap();
        let hex = hex::encode(&data);
        let expected = "a9059cbb000000000000000000000000d8da6bf26964af9d7eed9e03e53415d37aa9604500000000000000000000000000000000000000000000000000000000000f4240";
        assert_eq!(hex, expected);
    }
}

