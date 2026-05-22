use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use std::hint::black_box;

fn process_bytes(data: &[u8]) -> u64 {
    data.iter().map(|&b| b as u64).sum()
}

fn bench_throughput(c: &mut Criterion) {
    let data: Vec<u8> = (0..1_000_000u8.wrapping_add(0)).cycle().take(1_000_000).collect();

    let mut group = c.benchmark_group("bytes_processing");
    group.throughput(Throughput::Bytes(data.len() as u64));

    group.bench_function("sum_bytes", |b| {
        b.iter(|| process_bytes(black_box(&data)))
    });

    group.finish();
}

criterion_group!(benches, bench_throughput);
criterion_main!(benches);