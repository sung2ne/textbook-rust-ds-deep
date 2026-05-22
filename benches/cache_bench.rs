use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::hint::black_box;

struct EntityAoS {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    health: f32,
}

struct AoS {
    data: Vec<EntityAoS>,
}

struct SoA {
    x: Vec<f32>,
    y: Vec<f32>,
    vx: Vec<f32>,
    vy: Vec<f32>,
    health: Vec<f32>,
}

fn update_aos(entities: &mut AoS, dt: f32) {
    for e in &mut entities.data {
        e.x += e.vx * dt;
        e.y += e.vy * dt;
    }
}

fn update_soa(entities: &mut SoA, dt: f32) {
    for i in 0..entities.x.len() {
        entities.x[i] += entities.vx[i] * dt;
        entities.y[i] += entities.vy[i] * dt;
    }
}

fn bench_cache(c: &mut Criterion) {
    let mut group = c.benchmark_group("aos_vs_soa");

    for n in [10_000usize, 100_000, 1_000_000] {
        let mut aos = AoS {
            data: (0..n)
                .map(|i| EntityAoS {
                    x: i as f32, y: i as f32,
                    vx: 1.0, vy: 0.5, health: 100.0,
                })
                .collect(),
        };
        let mut soa = SoA {
            x: (0..n).map(|i| i as f32).collect(),
            y: (0..n).map(|i| i as f32).collect(),
            vx: vec![1.0; n],
            vy: vec![0.5; n],
            health: vec![100.0; n],
        };

        group.bench_with_input(
            BenchmarkId::new("AoS", n),
            &n,
            |b, _| b.iter(|| update_aos(black_box(&mut aos), black_box(0.016))),
        );

        group.bench_with_input(
            BenchmarkId::new("SoA", n),
            &n,
            |b, _| b.iter(|| update_soa(black_box(&mut soa), black_box(0.016))),
        );
    }

    group.finish();
}

criterion_group!(benches, bench_cache);
criterion_main!(benches);