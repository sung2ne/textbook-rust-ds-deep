use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn compute_hash<T: Hash>(value: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    hasher.finish()
}

fn count_differing_bits(a: u64, b: u64) -> u32 {
    (a ^ b).count_ones()
}

fn main() {
    let pairs = [
        ("alice", "Alice"),
        ("hello", "hfllo"),
        ("rust",  "Rust"),
    ];

    for (a, b) in &pairs {
        let ha = compute_hash(a);
        let hb = compute_hash(b);
        let diff = count_differing_bits(ha, hb);
        println!(
            "{:6} vs {:6}: hash_a={:016x}, hash_b={:016x}, diff_bits={}",
            a, b, ha, hb, diff
        );
    }
}