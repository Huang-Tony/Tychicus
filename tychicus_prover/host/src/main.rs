use methods::{METHOD_ELF, METHOD_ID};
use risc0_zkvm::{default_prover, ExecutorEnv};
use risc0_zkvm::sha::Digest; // <--- This tool handles the ID format for us
use std::fs;
use std::time::Instant;

fn main() {
    // 1. Point to the "suspicious" file to test the crash first
    let file_path = "../data_gen/clean_file.txt";
    // let file_path = "../data_gen/suspicious_file.txt"; // for test of error detection
    println!("🕵️‍♀️  Auditing file: {}", file_path);   //for test of good, compliant data

    // 2. Read the file
    let data = fs::read(file_path).expect("File not found. Ensure Phase 1 Python script ran.");

    let env = ExecutorEnv::builder()
        .write(&data)
        .unwrap()
        .build()
        .unwrap();

    let prover = default_prover();

    println!("⚡ Generating ZK Proof on M3...");
    let start = Instant::now();

    // 3. PROVE (This will CRASH if 'restricted_ip' is found)
    let receipt = prover
        .prove(env, METHOD_ELF)
        .expect("Proof generation failed - this is expected if the file is non-compliant")
        .receipt;

    println!("✅ Proof Generated in {:.2?}", start.elapsed());
    
    // 4. PRINT ID (Corrected: No longer uses 'hex::encode')
    let id = Digest::from(METHOD_ID);
    println!("📜 Image ID: {}", id);
}