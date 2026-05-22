// 조건부 컴파일로 할당기 선택
#[cfg(all(feature = "use-jemalloc", not(target_env = "msvc")))]
use tikv_jemallocator::Jemalloc;

#[cfg(all(feature = "use-jemalloc", not(target_env = "msvc")))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

#[cfg(feature = "use-mimalloc")]
use mimalloc::MiMalloc;

#[cfg(feature = "use-mimalloc")]
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

use std::collections::HashMap;

pub fn bench_hashmap_insert(n: usize) -> HashMap<u64, u64> {
    let mut map = HashMap::with_capacity(n);
    for i in 0..n as u64 {
        map.insert(i, i.wrapping_mul(6364136223846793005));
    }
    map
}

pub fn bench_vec_push(n: usize) -> Vec<u64> {
    let mut v = Vec::new();
    for i in 0..n as u64 {
        v.push(i.wrapping_mul(6364136223846793005));
    }
    v
}