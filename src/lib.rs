//! This module provides functions and traits for saving and loading secrets as strings.
//! It includes implementations for saving and loading secrets using the `SecretsManager` struct.
//!
//! # Example
//!
//! ```
//! use sealed_secrets::{encode, decode};
//! use std::collections::HashMap;
//!
//! fn main() {
//!     let mut map = HashMap::new();
//!     map.insert("key1".to_string(), "value1".to_string());
//!     map.insert("key2".to_string(), "value2".to_string());
//!     let key = "password".to_string();
//!     let encoded = encode(&map, &key).unwrap();
//!     println!("{}", encoded);
//!     let key = "password".to_string();
//!     let map = decode(&encoded, &key).unwrap();
//!     println!("{:?}", map);
//! }
//! ```
mod io;

use std::collections::HashMap;

use io::{LoadFromString, SaveToString};
use securestore::{Error, KeySource, SecretsManager};

pub fn encode(map: &HashMap<String, String>, key: &String) -> Result<String, Error> {
    let vault_pass = KeySource::Password(key.as_str());
    let mut sman = SecretsManager::new(vault_pass).unwrap();
    for (k, v) in map {
        sman.set(k, v.clone());
    }
    sman.save_as_string()
}

pub fn decode(encoded: &String, key: &String) -> Result<HashMap<String, String>, Error> {
    let vault_pass = KeySource::Password(key.as_str());
    let sman = SecretsManager::load_from_string(encoded, vault_pass)?;
    let mut map = HashMap::new();
    for key in sman.keys() {
        map.insert(key.to_string(), sman.get(key)?);
    }
    Ok(map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_decode() {
        let mut map = HashMap::new();
        map.insert("key1".to_string(), "value1".to_string());
        map.insert("key2".to_string(), "value2".to_string());
        let key = "password".to_string();
        let encoded = encode(&map, &key).unwrap();
        let decoded = decode(&encoded, &key).unwrap();
        assert_eq!(map, decoded);
    }
}
