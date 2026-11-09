//! EIP-1559 (Type 2) transaction: construction, signing, RLP serialization.

use k256::ecdsa::SigningKey;
use primitive_types::{H160, U256};
use rlp::RlpStream;
use sha3::{Digest, Keccak256};

use crate::error::{CoreError, Result};
use crate::wallet::eth::EthAccount;

#[derive(Debug, Clone)]
pub struct Eip1559Transaction {
    pub chain_id: u64,
    pub nonce: u64,
    pub max_priority_fee_per_gas: U256,
    pub max_fee_per_gas: U256,
    pub gas_limit: u64,
    pub to: H160,
    pub value: U256,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct SignedEip1559 {
    pub raw: Vec<u8>,
    pub hash: [u8; 32],
    pub y_parity: u8,
    pub r: [u8; 32],
    pub s: [u8; 32],
}

impl SignedEip1559 {
    pub fn raw_hex(&self) -> String { format!("0x{}", hex::encode(&self.raw)) }
    pub fn hash_hex(&self) -> String { format!("0x{}", hex::encode(self.hash)) }
}

impl Eip1559Transaction {
    /// EIP-1559 signing hash:
    /// keccak256(0x02 || rlp([chain_id, nonce, max_priority, max_fee, gas, to, value, data, []]))
    pub fn signing_hash(&self) -> [u8; 32] {
        let mut s = RlpStream::new_list(9);
        s.append(&self.chain_id);
        s.append(&self.nonce);
        s.append(&self.max_priority_fee_per_gas);
        s.append(&self.max_fee_per_gas);
        s.append(&self.gas_limit);
        s.append(&self.to);
        s.append(&self.value);
        s.append(&self.data);
        let empty_al = RlpStream::new_list(0).out();
        s.append_raw(&empty_al, 1);
        let inner = s.out();

        let mut typed = Vec::with_capacity(1 + inner.len());
        typed.push(0x02);
        typed.extend_from_slice(&inner);
        Keccak256::digest(&typed).into()
    }

    pub fn sign(&self, account: &EthAccount) -> Result<SignedEip1559> {
        let hash = self.signing_hash();
        let signing_key = SigningKey::from_bytes((&account.private_key).into())
            .map_err(|e| CoreError::Wallet(format!("signing key: {e}")))?;
        let (sig, rec_id) = signing_key
            .sign_prehash_recoverable(&hash)
            .map_err(|e| CoreError::Wallet(format!("sign prehash: {e}")))?;

        let r_bytes = sig.r().to_bytes();
        let s_bytes = sig.s().to_bytes();
        let r = U256::from_big_endian(&r_bytes);
        let s = U256::from_big_endian(&s_bytes);
        let y_parity = rec_id.to_byte();

        let raw = self.rlp_signed(y_parity as u64, r, s);
        let hash_out: [u8; 32] = Keccak256::digest(&raw).into();

        let mut r_arr = [0u8; 32];
        r_arr.copy_from_slice(&r_bytes[..]);
        let mut s_arr = [0u8; 32];
        s_arr.copy_from_slice(&s_bytes[..]);

        Ok(SignedEip1559 { raw, hash: hash_out, y_parity, r: r_arr, s: s_arr })
    }

    fn rlp_signed(&self, y_parity: u64, r: U256, s: U256) -> Vec<u8> {
        let mut st = RlpStream::new_list(12);
        st.append(&self.chain_id);
        st.append(&self.nonce);
        st.append(&self.max_priority_fee_per_gas);
        st.append(&self.max_fee_per_gas);
        st.append(&self.gas_limit);
        st.append(&self.to);
        st.append(&self.value);
        st.append(&self.data);
        let empty_al = RlpStream::new_list(0).out();
        st.append_raw(&empty_al, 1);
        st.append(&y_parity);
        st.append(&r);
        st.append(&s);
        let inner = st.out();

        let mut typed = Vec::with_capacity(1 + inner.len());
        typed.push(0x02);
        typed.extend_from_slice(&inner);
        typed
    }
}

