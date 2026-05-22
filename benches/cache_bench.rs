use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::hint::black_box;
use std::collections::LinkedList;

fn sum_vec(data: &[u64]) -> u64 {
    data.iter().sum()
}

fn sum_linked_list(data: &LinkedList<u64>) -> u64 {
    data.iter().sum()
}

fn bench_cache(c: &mut Criterion) {
    let mut group = c.benchmark_group("cache_friendliness");

    for &size in &[1_000usize, 10_000, 100_000] {
        // Vec: 연속 메모리
        let vec_data: Vec<u64> = (0..size as u64).collect();

        // LinkedList: 흩어진 메모리
        let mut list_data: LinkedList<u64> = LinkedList::new();
        for i in 0..size as u64 {
            list_data.push_back(i);
        }

        group.bench_with_input(
            BenchmarkId::new("Vec", size),
            &vec_data,
            |b, data| b.iter(|| sum_vec(black_box(data))),
        );

        group.bench_with_input(
            BenchmarkId::new("LinkedList", size),
            &list_data,
            |b, data| b.iter(|| sum_linked_list(black_box(data))),
        );
    }

    group.finish();
}

criterion_group!(benches, bench_cache);
criterion_main!(benches);