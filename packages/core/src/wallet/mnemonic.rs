use bip39::{Language, Mnemonic as Bip39Mnemonic};
use rand::RngCore;
use zeroize::Zeroize;

use crate::error::{CoreError, Result};

/// Supported BIP-39 word counts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WordCount {
    Words12,
    Words24,
}

impl WordCount {
    pub fn as_usize(self) -> usize {
        match self {
            WordCount::Words12 => 12,
            WordCount::Words24 => 24,
        }
    }
}

/// BIP-39 mnemonic phrase. Wraps the `bip39` crate to keep the API stable.
///
/// Memory hygiene: when dropped, the internal BIP-39 object is not explicitly
/// zeroized here because `bip39::Mnemonic` does not expose zeroize on all
/// versions; we will add explicit zeroization in the key-derivation layer
/// where the seed leaves this struct.
pub struct Mnemonic {
    inner: Bip39Mnemonic,
}

impl Mnemonic {
    /// Generate a fresh mnemonic using OS-provided entropy.
    pub fn generate(words: WordCount) -> Result<Self> {
        // BIP-39 entropy: 128 bit (12 words) or 256 bit (24 words).
        let entropy_bytes = match words {
            WordCount::Words12 => 16,
            WordCount::Words24 => 32,
        };

        let mut entropy = vec![0u8; entropy_bytes];
        rand::thread_rng().fill_bytes(&mut entropy);

        let result = Bip39Mnemonic::from_entropy_in(Language::English, &entropy);
        entropy.zeroize();

        let inner = result.map_err(|e| CoreError::Wallet(format!("generate mnemonic: {e}")))?;
        Ok(Self { inner })
    }

    /// Parse and validate a phrase (BIP-39 checksum included).
    pub fn from_phrase(phrase: &str) -> Result<Self> {
        let normalized = normalize(phrase);
        let inner = Bip39Mnemonic::parse_in_normalized(Language::English, &normalized)
            .map_err(|e| CoreError::Wallet(format!("invalid mnemonic: {e}")))?;
        Ok(Self { inner })
    }

    /// Check the phrase without constructing a Mnemonic.
    pub fn validate(phrase: &str) -> bool {
        let normalized = normalize(phrase);
        Bip39Mnemonic::parse_in_normalized(Language::English, &normalized).is_ok()
    }

    /// The canonical phrase, space-separated, English words.
    pub fn phrase(&self) -> String {
        self.inner.to_string()
    }

    /// The number of words in the phrase.
    pub fn word_count(&self) -> usize {
        self.inner.word_count()
    }

    /// Derive the 64-byte BIP-39 seed (PBKDF2-HMAC-SHA512) for a passphrase.
    /// Empty string is the standard "no passphrase" case.
    pub fn to_seed(&self, passphrase: &str) -> [u8; 64] {
        self.inner.to_seed(passphrase)
    }
}

/// BIP-39 phrases are case-insensitive, tolerate extra whitespace.
fn normalize(phrase: &str) -> String {
    phrase
        .split_whitespace()
        .map(|w| w.to_lowercase())
        .collect::<Vec<_>>()
        .join(" ")
}
