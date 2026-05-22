use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::hint::black_box;
use std::collections::HashMap;

fn vec_contains(data: &[(u32, &str)], key: u32) -> Option<&str> {
    data.iter().find(|(k, _)| *k == key).map(|(_, v)| *v)
}

fn hashmap_get<'a>(map: &'a HashMap<u32, &str>, key: u32) -> Option<&&'a str> {
    map.get(&key)
}

fn bench_small_lookup(c: &mut Criterion) {
    let mut group = c.benchmark_group("small_lookup");

    for &size in &[8usize, 16, 32, 64, 128, 256] {
        let vec_data: Vec<(u32, &str)> = (0..size as u32).map(|i| (i, "value")).collect();

        let mut map: HashMap<u32, &str> = HashMap::new();
        for i in 0..size as u32 {
            map.insert(i, "value");
        }

        let target = size as u32 - 1; // 최악의 경우 탐색

        group.bench_with_input(
            BenchmarkId::new("Vec_linear", size),
            &(&vec_data, target),
            |b, &(data, target)| b.iter(|| vec_contains(black_box(data), black_box(target))),
        );

        group.bench_with_input(
            BenchmarkId::new("HashMap_get", size),
            &(&map, target),
            |b, &(map, target)| b.iter(|| hashmap_get(black_box(map), black_box(target))),
        );
    }

    group.finish();
}

criterion_group!(benches, bench_small_lookup);
criterion_main!(benches);