use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};
use std::io;

pub fn calculate_hash(data: &Vec<String>) -> u64 {
    let mut hasher = DefaultHasher::new();
    data.hash(&mut hasher);
    hasher.finish()
}

pub fn save_hash(hash: u64) -> io::Result<()> {
    fs::write("store", hash.to_string())
}

pub fn load_hash() -> u64 {
    fs::read_to_string("store")
        .unwrap_or("0".to_owned())
        .parse::<u64>()
        .unwrap_or(0u64)
}
