//! Legacy Ethereum transaction (Type 0) construction, EIP-155 signing, and
//! RLP serialization ready for `eth_sendRawTransaction`.

use k256::ecdsa::{RecoveryId, Signature, SigningKey};
use primitive_types::{H160, U256};
use rlp::RlpStream;
use sha3::{Digest, Keccak256};

use crate::error::{CoreError, Result};
use crate::wallet::eth::EthAccount;

/// EIP-155 requires this offset in the `v` field.
const EIP155_OFFSET: u64 = 35;

/// A legacy Ethereum transaction (Type 0), ready to be signed.
#[derive(Debug, Clone)]
pub struct LegacyTransaction {
    pub nonce: u64,
    pub gas_price: U256,
    pub gas_limit: u64,
    pub to: H160,
    pub value: U256,
    pub data: Vec<u8>,
    pub chain_id: u64,
}

/// A signed legacy transaction: the 9 RLP fields are ready to be broadcast.
#[derive(Debug, Clone)]
pub struct SignedTransaction {
    pub raw: Vec<u8>,
    pub hash: [u8; 32],
}

impl LegacyTransaction {
    /// EIP-155 signing hash: keccak256(rlp([nonce, gasPrice, gasLimit, to,
    /// value, data, chain_id, 0, 0])).
    pub fn signing_hash(&self) -> [u8; 32] {
        let mut s = RlpStream::new_list(9);
        s.append(&self.nonce);
        s.append(&self.gas_price);
        s.append(&self.gas_limit);
        s.append(&self.to);
        s.append(&self.value);
        s.append(&self.data);
        s.append(&self.chain_id);
        s.append(&0u8);
        s.append(&0u8);
        let encoded = s.out();
        Keccak256::digest(&encoded).into()
    }

    /// Sign with the account's private key, producing a broadcastable raw tx.
    pub fn sign(&self, account: &EthAccount) -> Result<SignedTransaction> {
        let hash = self.signing_hash();
        let signing_key = SigningKey::from_bytes((&account.private_key).into())
            .map_err(|e| CoreError::Wallet(format!("signing key: {e}")))?;

        let (sig, rec_id): (Signature, RecoveryId) = signing_key
            .sign_prehash_recoverable(&hash)
            .map_err(|e| CoreError::Wallet(format!("sign prehash: {e}")))?;

        let r_bytes = sig.r().to_bytes();
        let s_bytes = sig.s().to_bytes();
        let r = U256::from_big_endian(&r_bytes);
        let s = U256::from_big_endian(&s_bytes);

        // EIP-155: v = recovery_id + chain_id * 2 + 35
        let v = u64::from(rec_id.to_byte()) + self.chain_id * 2 + EIP155_OFFSET;

        let raw = self.rlp_signed(v, r, s);
        let hash: [u8; 32] = Keccak256::digest(&raw).into();
        Ok(SignedTransaction { raw, hash })
    }

    fn rlp_signed(&self, v: u64, r: U256, s: U256) -> Vec<u8> {
        let mut st = RlpStream::new_list(9);
        st.append(&self.nonce);
        // gas_price: trim leading zeros (same as signing_hash)
        let mut gp = [0u8; 32];
        self.gas_price.to_big_endian(&mut gp);
        let gp_trim = gp.iter().position(|&b| b != 0).map(|i| &gp[i..]).unwrap_or(&[]);
        st.append(&gp_trim);
        st.append(&self.gas_limit);
        st.append(&self.to);
        // value: trim leading zeros (same as signing_hash)
        let mut val = [0u8; 32];
        self.value.to_big_endian(&mut val);
        let val_trim = val.iter().position(|&b| b != 0).map(|i| &val[i..]).unwrap_or(&[]);
        st.append(&val_trim);
        st.append(&self.data);
        st.append(&v);
        st.append(&r);
        st.append(&s);
        st.out().to_vec()
    }

    /// PQ-сообщение для Dilithium-подписи.
    /// Должно совпадать с формулой в ноде (aic-node/src/transaction.rs::pq_message).
    /// keccak256(rlp([b"AIC-PQ-v1", chain_id, nonce, from, to, value, data]))
    pub fn pq_message(&self, from: [u8; 20]) -> [u8; 32] {
        use sha3::{Digest, Keccak256};
        let mut stream = RlpStream::new_list(7);
        stream.append(&b"AIC-PQ-v1".as_slice());
        stream.append(&self.chain_id);
        stream.append(&self.nonce);
        stream.append(&from.as_slice());
        stream.append(&self.to.as_bytes());
        let mut v = [0u8; 32];
        self.value.to_big_endian(&mut v);
        let start = v.iter().position(|&b| b != 0).unwrap_or(32);
        let v_trim: &[u8] = &v[start..];
        stream.append(&v_trim);
        stream.append(&self.data);
        Keccak256::digest(&stream.out()).into()
    }
}

impl SignedTransaction {
    /// Hex-encoded raw transaction, 0x-prefixed, ready for
    /// `eth_sendRawTransaction`.
    pub fn raw_hex(&self) -> String {
        format!("0x{}", hex::encode(&self.raw))
    }

    /// Hex-encoded transaction hash (the tx id).
    pub fn hash_hex(&self) -> String {
        format!("0x{}", hex::encode(self.hash))
    }
}




