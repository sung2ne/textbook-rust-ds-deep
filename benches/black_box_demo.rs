use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

fn sum_to_n(n: u64) -> u64 {
    (0..n).sum()
}

fn bench_correct(c: &mut Criterion) {
    // 올바른 사용: 입력과 결과 모두 black_box로 감쌈
    c.bench_function("sum_correct", |b| {
        b.iter(|| {
            let result = sum_to_n(black_box(1000));
            black_box(result) // 결과를 사용한 것처럼 처리
        })
    });
}

fn bench_wrong(c: &mut Criterion) {
    // 잘못된 사용: black_box 없음 → 컴파일러가 최적화로 없앨 수 있음
    c.bench_function("sum_wrong", |b| {
        b.iter(|| sum_to_n(1000))
    });
}

criterion_group!(benches, bench_correct, bench_wrong);
criterion_main!(benches);