use std::fs;
use std::path::PathBuf;
use std::{collections::HashMap, env::temp_dir};

use securestore::{Error, KeySource, SecretsManager};
use uuid::Uuid;

fn create_file(dir: &mut PathBuf) -> String {
    let file_name = format!("{}.json", Uuid::new_v4());
    dir.push(file_name.as_str());
    file_name
}

pub trait SaveToString {
    fn save_as_string(&self) -> Result<String, Error>;
}

pub trait LoadFromString<T> {
    fn load_from_string(encoded: &String, key: KeySource) -> Result<T, Error>;
}

impl SaveToString for SecretsManager {
    fn save_as_string(&self) -> Result<String, Error> {
        let mut dir = temp_dir();
        let file_name = create_file(&mut dir);
        self.save_as(file_name.as_str())?;
        let encoded = fs::read_to_string(file_name.as_str())?;
        fs::remove_file(file_name.as_str())?;
        dir.pop();
        Ok(encoded)
    }
}

impl LoadFromString<SecretsManager> for SecretsManager {
    fn load_from_string(encoded: &String, key: KeySource) -> Result<SecretsManager, Error> {
        let mut dir = temp_dir();
        let file_name = create_file(&mut dir);
        fs::write(file_name.as_str(), encoded.as_bytes())?;
        let sman = SecretsManager::load(file_name.as_str(), key).unwrap();
        fs::remove_file(file_name.as_str())?;
        dir.pop();
        Ok(sman)
    }
}


pub fn encode(map: &HashMap<String, String>, key: &String) -> Result<String, Error> {
    let vault_pass = KeySource::Password(key.as_str());
    let mut sman = SecretsManager::new(vault_pass).unwrap();
    for (k, v) in map {
        sman.set(k, v.clone());
    }
    sman.save_as_string()
}


pub fn decode(encoded: &String, key: &String) -> Result<HashMap<String,String>, Error> {
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