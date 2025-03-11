//File storage logic
use std::fs;
use std::path::Path;
use sha2::{Sha256, Digest};
use std::io::{Read, Write};

pub fn store_file(file_path: &str, data: &[u8]) -> String {
    let hash = calculate_hash(data);
    let storage_path = format!("./data/{}.dat", hash);
    
    fs::write(&storage_path, data).expect("Failed to write file");
    println!("Stored file at {}", storage_path);
    
    hash
}

pub fn retrieve_file(hash: &str) -> Option<Vec<u8>> {
    let file_path = format!("./data/{}.dat", hash);
    if Path::new(&file_path).exists() {
        let content = fs::read(file_path).ok()?;
        Some(content)
    } else {
        None
    }
}

fn calculate_hash(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}
