use rs_merkle::{MerkleTree, algorithms::Sha256};
use sha2::Digest;
use std::fs::File;
use std::io::{BufReader, Read};
use std::time::Instant;

fn main() {
    // Note the path: we are going up one level to find the data folder
    let file_path = "../data_gen/dataset_10gb.txt"; 
    
    println!("🚀 Starting Commitment on M3 (Streaming Mode)...");
    println!("Reading: {}", file_path);

    let start = Instant::now();
    let f = File::open(file_path).expect("File not found! Run the Python script first.");
    let mut reader = BufReader::new(f);
    
    // 1MB chunks to keep memory usage low
    let mut buffer = [0; 1024 * 1024]; 
    let mut leaves: Vec<[u8; 32]> = Vec::new();

    let mut bytes_read: u64 = 0;

    loop {
        let count = reader.read(&mut buffer).expect("Read failed");
        if count == 0 { break; } 
        
        let hash = sha2::Sha256::digest(&buffer[..count]);
        leaves.push(hash.into());
        bytes_read += count as u64;
    }

    println!("✅ Hashing complete. Processed {} GB.", bytes_read / 1024 / 1024 / 1024);
    println!("🌳 Building Merkle Tree...");

    let tree = MerkleTree::<Sha256>::from_leaves(&leaves);
    let root = tree.root_hex().unwrap();
    
    let duration = start.elapsed();
    println!("------------------------------------------------");
    println!("🏆 Root Hash: {}", root);
    println!("⏱️  Time taken: {:.2?}", duration);
    println!("------------------------------------------------");
}