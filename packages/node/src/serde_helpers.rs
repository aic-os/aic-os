//! Помощники сериализации для serde_json: HashMap с ключами [u8; N]
//! превращаются в Vec<(hex_string, value)>, чтобы JSON-ключи были строками.

use std::collections::HashMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub mod map_bytes20 {
    use super::*;
    pub fn serialize<V: Serialize, S: Serializer>(m: &HashMap<[u8; 20], V>, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeSeq;
        let mut seq = s.serialize_seq(Some(m.len()))?;
        for (k, v) in m {
            seq.serialize_element(&(hex::encode(k), v))?;
        }
        seq.end()
    }
    pub fn deserialize<'de, V: Deserialize<'de>, D: Deserializer<'de>>(d: D) -> Result<HashMap<[u8; 20], V>, D::Error> {
        let pairs: Vec<(String, V)> = Vec::deserialize(d)?;
        let mut m = HashMap::new();
        for (k, v) in pairs {
            let b = hex::decode(&k).map_err(serde::de::Error::custom)?;
            if b.len() != 20 { return Err(serde::de::Error::custom("expected 20-byte key")); }
            let mut a = [0u8; 20]; a.copy_from_slice(&b);
            m.insert(a, v);
        }
        Ok(m)
    }
}

pub mod map_bytes32 {
    use super::*;
    pub fn serialize<V: Serialize, S: Serializer>(m: &HashMap<[u8; 32], V>, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeSeq;
        let mut seq = s.serialize_seq(Some(m.len()))?;
        for (k, v) in m {
            seq.serialize_element(&(hex::encode(k), v))?;
        }
        seq.end()
    }
    pub fn deserialize<'de, V: Deserialize<'de>, D: Deserializer<'de>>(d: D) -> Result<HashMap<[u8; 32], V>, D::Error> {
        let pairs: Vec<(String, V)> = Vec::deserialize(d)?;
        let mut m = HashMap::new();
        for (k, v) in pairs {
            let b = hex::decode(&k).map_err(serde::de::Error::custom)?;
            if b.len() != 32 { return Err(serde::de::Error::custom("expected 32-byte key")); }
            let mut a = [0u8; 32]; a.copy_from_slice(&b);
            m.insert(a, v);
        }
        Ok(m)
    }
}
pub mod map_storage {
    use super::*;

    pub fn serialize<S: Serializer>(
        m: &HashMap<[u8; 20], HashMap<[u8; 32], [u8; 32]>>,
        s: S,
    ) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeSeq;
        let mut seq = s.serialize_seq(Some(m.len()))?;
        for (addr, slots) in m {
            let inner: Vec<(String, String)> = slots.iter()
                .map(|(k, v)| (hex::encode(k), hex::encode(v)))
                .collect();
            seq.serialize_element(&(hex::encode(addr), inner))?;
        }
        seq.end()
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(
        d: D,
    ) -> Result<HashMap<[u8; 20], HashMap<[u8; 32], [u8; 32]>>, D::Error> {
        let pairs: Vec<(String, Vec<(String, String)>)> = Vec::deserialize(d)?;
        let mut out = HashMap::new();
        for (addr_hex, slots) in pairs {
            let ab = hex::decode(&addr_hex).map_err(|e| <D::Error as serde::de::Error>::custom(e))?;
            if ab.len() != 20 { return Err(<D::Error as serde::de::Error>::custom("20-byte addr")); }
            let mut addr = [0u8; 20]; addr.copy_from_slice(&ab);
            let mut inner = HashMap::new();
            for (k, v) in slots {
                let kb = hex::decode(&k).map_err(|e| <D::Error as serde::de::Error>::custom(e))?;
                let vb = hex::decode(&v).map_err(|e| <D::Error as serde::de::Error>::custom(e))?;
                if kb.len() != 32 || vb.len() != 32 { return Err(<D::Error as serde::de::Error>::custom("32-byte slot")); }
                let mut kk = [0u8; 32]; kk.copy_from_slice(&kb);
                let mut vv = [0u8; 32]; vv.copy_from_slice(&vb);
                inner.insert(kk, vv);
            }
            out.insert(addr, inner);
        }
        Ok(out)
    }
}
