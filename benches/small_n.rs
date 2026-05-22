use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::hint::black_box;
use std::collections::HashMap;

fn lookup_vec(v: &[(u32, u32)], key: u32) -> Option<u32> {
    v.iter().find(|&&(k, _)| k == key).map(|&(_, v)| v)
}

fn lookup_hashmap(m: &HashMap<u32, u32>, key: u32) -> Option<u32> {
    m.get(&key).copied()
}

fn bench_small_n(c: &mut Criterion) {
    let mut group = c.benchmark_group("small_n_lookup");

    for n in [5usize, 10, 20, 50, 100] {
        let vec_data: Vec<(u32, u32)> = (0..n as u32).map(|i| (i, i * 2)).collect();
        let map_data: HashMap<u32, u32> = (0..n as u32).map(|i| (i, i * 2)).collect();
        let target = (n as u32).saturating_sub(1);

        group.bench_with_input(
            BenchmarkId::new("Vec", n),
            &(vec_data, target),
            |b, (v, t)| b.iter(|| lookup_vec(black_box(v), black_box(*t))),
        );

        group.bench_with_input(
            BenchmarkId::new("HashMap", n),
            &(map_data, target),
            |b, (m, t)| b.iter(|| lookup_hashmap(black_box(m), black_box(*t))),
        );
    }

    group.finish();
}

criterion_group!(benches, bench_small_n);
criterion_main!(benches);