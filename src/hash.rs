use std::fs;
use std::io;
use crate::errors::NoDyk;

pub fn calculate_hash(data: &Vec<String>) -> Result<&String, NoDyk> {
    data.first().ok_or(NoDyk)
}

pub fn save_hash(hash: &String) -> io::Result<()> {
    fs::write("store", hash)
}

pub fn load_hash() -> String {
    fs::read_to_string("store").unwrap_or("".to_owned())
}
