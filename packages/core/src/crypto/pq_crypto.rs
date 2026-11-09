//! Post-Quantum cryptography: Schnorr (BIP-340) + ML-DSA-65 (Dilithium3).

use k256::schnorr::{
    signature::{Signer as SchnorrSigner, Verifier as SchnorrVerifier},
    SigningKey as SchnorrSigningKey,
    VerifyingKey as SchnorrVerifyingKey,
    Signature as SchnorrSignature,
};
use ml_dsa::{
    MlDsa65, Generate, Keypair,
    SigningKey as MlDsaSigningKey,
    VerifyingKey as MlDsaVerifyingKey,
    Signature as MlDsaSignature,
    EncodedVerifyingKey, EncodedSignature,
    signature::{Signer as MlDsaSigner, Verifier as MlDsaVerifier},
    SignatureEncoding, KeyExport,
};
use rand::rngs::OsRng;

use crate::error::{CoreError, Result};

#[derive(Clone, Debug)]
pub struct HybridSignature {
    pub schnorr: Vec<u8>,
    pub dilithium: Option<Vec<u8>>,
}

#[derive(Clone, Debug)]
pub struct HybridPublicKey {
    pub schnorr: [u8; 32],
    pub dilithium: Option<Vec<u8>>,
}

pub fn generate_keypair() -> (HybridPublicKey, Vec<u8>) {
    let schnorr_sk = SchnorrSigningKey::random(&mut OsRng);
    let schnorr_pk_bytes = schnorr_sk.verifying_key().to_bytes();

    let ml_dsa_sk = MlDsaSigningKey::<MlDsa65>::generate();
    let ml_dsa_pk_bytes = ml_dsa_sk.verifying_key().to_bytes().to_vec();
    let ml_dsa_seed = ml_dsa_sk.to_seed();

    let pubkey = HybridPublicKey {
        schnorr: schnorr_pk_bytes.into(),
        dilithium: Some(ml_dsa_pk_bytes),
    };

    let mut secret = Vec::with_capacity(32 + ml_dsa_seed.len());
    secret.extend_from_slice(&schnorr_sk.to_bytes()[..]);
    secret.extend_from_slice(ml_dsa_seed.as_slice());
    (pubkey, secret)
}

pub fn sign_hybrid(
    message: &[u8],
    secret: &[u8],
    with_dilithium: bool,
) -> Result<HybridSignature> {
    if secret.len() < 32 {
        return Err(CoreError::Wallet("secret too short".into()));
    }
    let (schnorr_sk_bytes, ml_dsa_seed_bytes) = secret.split_at(32);
    let schnorr_sk = SchnorrSigningKey::from_bytes(schnorr_sk_bytes)
        .map_err(|e| CoreError::Wallet(format!("schnorr sk: {e}")))?;

    let schnorr_sig = schnorr_sk.sign(message);
    let schnorr_bytes = schnorr_sig.to_bytes().to_vec();

    let dilithium_sig = if with_dilithium {
        let seed: [u8; 32] = ml_dsa_seed_bytes
            .try_into()
            .map_err(|_| CoreError::Wallet("bad ml-dsa seed length".into()))?;
        let ml_dsa_sk = MlDsaSigningKey::<MlDsa65>::from_seed(&seed.into());
        Some(ml_dsa_sk.sign(message).to_bytes().to_vec())
    } else {
        None
    };

    Ok(HybridSignature { schnorr: schnorr_bytes, dilithium: dilithium_sig })
}

pub fn verify_hybrid(
    message: &[u8],
    signature: &HybridSignature,
    pubkey: &HybridPublicKey,
) -> Result<bool> {
    let schnorr_pk = SchnorrVerifyingKey::from_bytes(&pubkey.schnorr)
        .map_err(|e| CoreError::Wallet(format!("schnorr pk: {e}")))?;
    let schnorr_sig = SchnorrSignature::try_from(signature.schnorr.as_slice())
        .map_err(|e| CoreError::Wallet(format!("schnorr sig: {e}")))?;
    if schnorr_pk.verify(message, &schnorr_sig).is_err() {
        return Ok(false);
    }

    if let (Some(dilithium_sig_bytes), Some(dilithium_pk_bytes)) =
        (&signature.dilithium, &pubkey.dilithium)
    {
        let pk_encoded = EncodedVerifyingKey::<MlDsa65>::try_from(dilithium_pk_bytes.as_slice())
            .map_err(|_| CoreError::Wallet("bad ml-dsa pk length".into()))?;
        let ml_dsa_pk = MlDsaVerifyingKey::<MlDsa65>::decode(&pk_encoded);

        let sig_encoded = EncodedSignature::<MlDsa65>::try_from(dilithium_sig_bytes.as_slice())
            .map_err(|_| CoreError::Wallet("bad ml-dsa sig length".into()))?;
        let ml_dsa_sig = MlDsaSignature::<MlDsa65>::decode(&sig_encoded)
            .ok_or_else(|| CoreError::Wallet("sig decode failed".into()))?;

        if ml_dsa_pk.verify(message, &ml_dsa_sig).is_err() {
            return Ok(false);
        }
    }

    Ok(true)
}

/// Только Dilithium-проверка (без Schnorr). Для ноды: Schnorr проверяется отдельно
/// через legacy ECDSA verify, а Dilithium -- здесь.
pub fn verify_dilithium_only(message: &[u8], sig_bytes: &[u8], pk_bytes: &[u8]) -> Result<bool> {
    let pk_encoded = EncodedVerifyingKey::<MlDsa65>::try_from(pk_bytes)
        .map_err(|_| CoreError::Wallet("bad ml-dsa pk length".into()))?;
    let ml_dsa_pk = MlDsaVerifyingKey::<MlDsa65>::decode(&pk_encoded);

    let sig_encoded = EncodedSignature::<MlDsa65>::try_from(sig_bytes)
        .map_err(|_| CoreError::Wallet("bad ml-dsa sig length".into()))?;
    let ml_dsa_sig = MlDsaSignature::<MlDsa65>::decode(&sig_encoded)
        .ok_or_else(|| CoreError::Wallet("sig decode failed".into()))?;

    Ok(ml_dsa_pk.verify(message, &ml_dsa_sig).is_ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hybrid_schnorr_only() {
        let (pk, sk) = generate_keypair();
        let msg = b"hello AIC";
        let sig = sign_hybrid(msg, &sk, false).unwrap();
        assert!(verify_hybrid(msg, &sig, &pk).unwrap());
    }

    #[test]
    fn hybrid_full() {
        let (pk, sk) = generate_keypair();
        let msg = b"critical: transfer 1M AIC";
        let sig = sign_hybrid(msg, &sk, true).unwrap();
        assert!(verify_hybrid(msg, &sig, &pk).unwrap());
    }

    #[test]
    fn tampered_fails() {
        let (pk, sk) = generate_keypair();
        let msg = b"hello";
        let mut sig = sign_hybrid(msg, &sk, true).unwrap();
        sig.schnorr[0] ^= 0xFF;
        assert!(!verify_hybrid(msg, &sig, &pk).unwrap());
    }

    #[test]
    fn dilithium_only_roundtrip() {
        let (pk, sk) = generate_keypair();
        let msg = b"dilithium only test";
        let sig = sign_hybrid(msg, &sk, true).unwrap();
        let d_sig = sig.dilithium.unwrap();
        let d_pk = pk.dilithium.unwrap();
        assert!(verify_dilithium_only(msg, &d_sig, &d_pk).unwrap());
    }
}

