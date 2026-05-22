use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput,};
use std::hint::black_box;

fn process_slice(data: &[u64]) -> u64 {
    data.iter().sum()
}

fn bench_throughput(c: &mut Criterion) {
    let mut group = c.benchmark_group("slice_sum");

    for size in [1_000u64, 10_000, 100_000] {
        let data: Vec<u64> = (0..size).collect();

        // throughput을 설정하면 "elements/second" 단위로 결과가 나옴
        group.throughput(Throughput::Elements(size));

        group.bench_with_input(BenchmarkId::from_parameter(size), &data, |b, data| {
            b.iter(|| process_slice(black_box(data)))
        });
    }

    group.finish();
}

criterion_group!(benches, bench_throughput);
criterion_main!(benches);