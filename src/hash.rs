use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};
use std::io;

pub fn calculate_hash(data: &Vec<String>) -> Vec<u64> {
    data.iter()
        .filter(|string| !string.is_empty())
        .map(calculate_string_hash)
        .collect()
}

fn calculate_string_hash(string: &String) -> u64 {
    let mut hasher = DefaultHasher::new();
    string.hash(&mut hasher);
    hasher.finish()
}

pub fn check_hashes(left: &Vec<u64>, right: &Vec<u64>) -> bool {
    if left.len() != right.len() {
        return false;
    }

    for (i, element) in left.iter().enumerate() {
        if element == &right[i] {
            return true;
        }
    }

    false
}

pub fn save_hash(hash: &Vec<u64>) -> io::Result<()> {
    let strings: Vec<String> = hash.iter().map(|i| i.to_string()).collect();
    fs::write("store", strings.join("|"))
}

pub fn load_hash() -> io::Result<Vec<u64>> {
    let string = fs::read_to_string("store")?;
    let result: Vec<u64> = string.split("|")
        .flat_map(|s| s.parse::<u64>())
        .collect();
    Ok(result)
}
