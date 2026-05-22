use allocator_bench::{bench_hashmap_insert, bench_vec_push};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::hint::black_box;

fn bench_allocators(c: &mut Criterion) {
    let mut group = c.benchmark_group("allocator_comparison");

    for n in [10_000usize, 100_000, 1_000_000] {
        group.bench_with_input(
            BenchmarkId::new("hashmap_insert", n),
            &n,
            |b, &n| b.iter(|| bench_hashmap_insert(black_box(n))),
        );

        group.bench_with_input(
            BenchmarkId::new("vec_push", n),
            &n,
            |b, &n| b.iter(|| bench_vec_push(black_box(n))),
        );
    }

    group.finish();
}

criterion_group!(benches, bench_allocators);
criterion_main!(benches);