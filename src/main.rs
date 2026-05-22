use rustc_hash::FxHashMap;
use std::collections::HashMap;
use std::time::Instant;

fn bench_hashmap(n: usize) -> std::time::Duration {
    let start = Instant::now();
    let mut map: HashMap<u64, u64> = HashMap::with_capacity(n);
    for i in 0..n as u64 {
        map.insert(i, i);
    }
    let mut sum = 0u64;
    for i in 0..n as u64 {
        sum = sum.wrapping_add(*map.get(&i).unwrap());
    }
    std::hint::black_box(sum);
    start.elapsed()
}

fn bench_fxhashmap(n: usize) -> std::time::Duration {
    let start = Instant::now();
    let mut map: FxHashMap<u64, u64> = FxHashMap::with_capacity_and_hasher(
        n,
        Default::default(),
    );
    for i in 0..n as u64 {
        map.insert(i, i);
    }
    let mut sum = 0u64;
    for i in 0..n as u64 {
        sum = sum.wrapping_add(*map.get(&i).unwrap());
    }
    std::hint::black_box(sum);
    start.elapsed()
}

fn main() {
    let n = 1_000_000;
    let hash_time = bench_hashmap(n);
    let fxhash_time = bench_fxhashmap(n);

    println!("HashMap  (SipHash):  {:?}", hash_time);
    println!("FxHashMap (FxHash): {:?}", fxhash_time);
    println!(
        "FxHash가 {:.1}배 빠름",
        hash_time.as_nanos() as f64 / fxhash_time.as_nanos() as f64
    );
}