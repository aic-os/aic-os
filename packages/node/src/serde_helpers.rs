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
