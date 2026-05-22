use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::hint::black_box;
use std::collections::LinkedList;

fn vec_insert_front(n: usize) -> Vec<u64> {
    let mut v: Vec<u64> = Vec::with_capacity(n);
    for i in 0..n as u64 {
        v.insert(0, i); // 앞에 삽입 → O(n)
    }
    v
}

fn linked_list_push_front(n: usize) -> LinkedList<u64> {
    let mut list = LinkedList::new();
    for i in 0..n as u64 {
        list.push_front(i); // 앞에 삽입 → O(1)
    }
    list
}

fn bench_insert_front(c: &mut Criterion) {
    let mut group = c.benchmark_group("insert_front");

    for &size in &[100usize, 500, 1_000, 5_000] {
        group.bench_with_input(
            BenchmarkId::new("Vec_insert_front", size),
            &size,
            |b, &size| b.iter(|| vec_insert_front(black_box(size))),
        );

        group.bench_with_input(
            BenchmarkId::new("LinkedList_push_front", size),
            &size,
            |b, &size| b.iter(|| linked_list_push_front(black_box(size))),
        );
    }

    group.finish();
}

criterion_group!(benches, bench_insert_front);
criterion_main!(benches);