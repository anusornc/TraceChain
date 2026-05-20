#[path = "../blockchain.rs"]
mod blockchain;

use blockchain::Block;
use sha2::{Digest, Sha256};
use std::time::Instant;

fn main() {
    let block = Block::new(1, "x".repeat(1000), "0".repeat(64));

    // baseline implementation
    let calculate_hash_baseline = || {
        let record = format!(
            "{}{}{}{}",
            block.index, block.timestamp, block.data, block.previous_hash
        );
        let mut hasher = Sha256::new();
        hasher.update(record.as_bytes());
        format!("{:x}", hasher.finalize())
    };

    let start = Instant::now();
    let iters = 100000;
    for _ in 0..iters {
        let _ = calculate_hash_baseline();
    }
    let baseline_time = start.elapsed().as_millis();
    println!("Baseline: {} ms for {} iterations", baseline_time, iters);

    let start = Instant::now();
    for _ in 0..iters {
        let _ = block.calculate_hash();
    }
    let optimized_time = start.elapsed().as_millis();
    println!("Optimized: {} ms for {} iterations", optimized_time, iters);
}
