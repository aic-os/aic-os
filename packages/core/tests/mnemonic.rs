use aic_core::wallet::{Mnemonic, WordCount};

// BIP-39 spec test vector #1: 128 bit entropy, all zeros.
const VECTOR_12: &str =
    "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";

// BIP-39 spec test vector #2: 256 bit entropy, all zeros.
const VECTOR_24: &str = "abandon abandon abandon abandon abandon abandon abandon abandon \
abandon abandon abandon abandon abandon abandon abandon abandon \
abandon abandon abandon abandon abandon abandon abandon art";

#[test]
fn validate_known_12() {
    assert!(Mnemonic::validate(VECTOR_12));
}

#[test]
fn validate_known_24() {
    assert!(Mnemonic::validate(VECTOR_24));
}

#[test]
fn validate_rejects_bad_checksum() {
    // Replace "about" with "abandon" -> checksum fails.
    let bad = VECTOR_12.replace("about", "abandon");
    assert!(!Mnemonic::validate(&bad));
}

#[test]
fn validate_rejects_garbage() {
    assert!(!Mnemonic::validate(""));
    assert!(!Mnemonic::validate("hello world"));
    assert!(!Mnemonic::validate("not a real mnemonic phrase at all"));
}

#[test]
fn from_phrase_roundtrip() {
    let m = Mnemonic::from_phrase(VECTOR_12).expect("parse");
    assert_eq!(m.word_count(), 12);
    assert_eq!(m.phrase(), VECTOR_12);
}

#[test]
fn from_phrase_normalizes_whitespace_and_case() {
    let noisy = "  ABANDON  Abandon abandon abandon abandon abandon abandon abandon \
                 abandon abandon abandon  ABOUT  ";
    let m = Mnemonic::from_phrase(noisy).expect("parse noisy");
    assert_eq!(m.phrase(), VECTOR_12);
}

#[test]
fn generate_12_has_12_words_and_validates() {
    let m = Mnemonic::generate(WordCount::Words12).expect("gen");
    assert_eq!(m.word_count(), 12);
    assert!(Mnemonic::validate(&m.phrase()));
}

#[test]
fn generate_24_has_24_words_and_validates() {
    let m = Mnemonic::generate(WordCount::Words24).expect("gen");
    assert_eq!(m.word_count(), 24);
    assert!(Mnemonic::validate(&m.phrase()));
}

#[test]
fn generate_produces_distinct_phrases() {
    let a = Mnemonic::generate(WordCount::Words12).unwrap();
    let b = Mnemonic::generate(WordCount::Words12).unwrap();
    assert_ne!(a.phrase(), b.phrase());
}

#[test]
fn seed_is_64_bytes_and_deterministic() {
    let m = Mnemonic::from_phrase(VECTOR_12).unwrap();
    let s1 = m.to_seed("");
    let s2 = m.to_seed("");
    assert_eq!(s1.len(), 64);
    assert_eq!(s1, s2);
}

#[test]
fn different_passphrase_gives_different_seed() {
    let m = Mnemonic::from_phrase(VECTOR_12).unwrap();
    assert_ne!(m.to_seed(""), m.to_seed("TREZOR"));
}

#[test]
fn known_seed_for_vector_with_passphrase_trezor() {
    // BIP-39 spec test vector: seed for the 12-word "abandon ... about"
    // with passphrase "TREZOR" (hex).
    let expected_hex = "c55257c360c07c72029aebc1b53c05ed0362ada38ead3e3e9efa3708e5349553\
1f09a6987599d18264c1e1c92f2cf141630c7a3c4ab7c81b2f001698e7463b04";
    let m = Mnemonic::from_phrase(VECTOR_12).unwrap();
    let seed = m.to_seed("TREZOR");
    let actual_hex: String = seed.iter().map(|b| format!("{b:02x}")).collect();
    assert_eq!(actual_hex, expected_hex);
}
