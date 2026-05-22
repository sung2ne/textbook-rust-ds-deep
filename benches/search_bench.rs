use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput,};
use std::hint::black_box;

fn linear_search(data: &[u64], target: u64) -> bool {
    data.iter().any(|&x| x == target)
}

fn binary_search(data: &[u64], target: u64) -> bool {
    data.binary_search(&target).is_ok()
}

fn bench_search(c: &mut Criterion) {
    let mut group = c.benchmark_group("search");

    for &size in &[100usize, 1_000, 10_000, 100_000] {
        let data: Vec<u64> = (0..size as u64).collect(); // 정렬된 상태
        let target = size as u64 - 1; // 최악의 경우

        group.throughput(Throughput::Elements(size as u64));

        group.bench_with_input(
            BenchmarkId::new("linear", size),
            &(&data, target),
            |b, &(data, target)| {
                b.iter(|| linear_search(black_box(data), black_box(target)))
            },
        );

        group.bench_with_input(
            BenchmarkId::new("binary", size),
            &(&data, target),
            |b, &(data, target)| {
                b.iter(|| binary_search(black_box(data), black_box(target)))
            },
        );
    }

    group.finish();
}

criterion_group!(benches, bench_search);
criterion_main!(benches);