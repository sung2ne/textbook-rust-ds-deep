use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

use std::collections::HashMap;

fn main() {
    let mut map: HashMap<u64, u64> = HashMap::new();
    for i in 0..100_000u64 {
        map.insert(i, i * 2);
    }
    println!("Inserted {} entries", map.len());
}