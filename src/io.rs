use std::env::temp_dir;
use std::fs;
use std::path::PathBuf;

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
