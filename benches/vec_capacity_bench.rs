use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::hint::black_box;

fn fill_without_capacity(n: usize) -> Vec<u64> {
    let mut v = Vec::new(); // 초기 용량: 0
    for i in 0..n as u64 {
        v.push(i); // n에 따라 log₂(n)번 재할당 발생
    }
    v
}

fn fill_with_capacity(n: usize) -> Vec<u64> {
    let mut v = Vec::with_capacity(n); // 처음부터 n개 예약
    for i in 0..n as u64 {
        v.push(i); // 재할당 없음
    }
    v
}

fn bench_capacity(c: &mut Criterion) {
    let mut group = c.benchmark_group("vec_capacity");

    for &size in &[1_000usize, 10_000, 100_000, 1_000_000] {
        group.bench_with_input(
            BenchmarkId::new("without_capacity", size),
            &size,
            |b, &size| b.iter(|| fill_without_capacity(black_box(size))),
        );

        group.bench_with_input(
            BenchmarkId::new("with_capacity", size),
            &size,
            |b, &size| b.iter(|| fill_with_capacity(black_box(size))),
        );
    }

    group.finish();
}

criterion_group!(benches, bench_capacity);
criterion_main!(benches);