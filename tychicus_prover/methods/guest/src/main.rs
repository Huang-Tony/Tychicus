#![no_main]
use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

fn main() {
    let file_chunk: Vec<u8> = env::read();
    let banned = b"restricted_ip";

    if file_chunk.windows(banned.len()).any(|window| window == banned) {
        panic!("🚨 COMPLIANCE FAILURE: Banned content found!");
    }

    let compliant = true;
    env::commit(&compliant);
}