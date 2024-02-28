use std::fs;

mod secret;

fn main() {
    let mut map = std::collections::HashMap::new();
    map.insert("key1".to_string(), "value1".to_string());
    map.insert("key2".to_string(), "value2".to_string());
    let key = "password".to_string();
    let encoded = secret::encode(&map, &key).unwrap();
    println!("{}", encoded);

    let encoded = fs::read_to_string("sealed.json").unwrap();
    let decoded = secret::decode(&encoded, &key).unwrap();
    println!("{:?}", decoded);
}
