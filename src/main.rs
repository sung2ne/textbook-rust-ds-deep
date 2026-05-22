use std::time::Instant;

fn some_work(n: u64) -> u64 {
    (0..n).sum()
}

fn bench_manual(n: u64, iterations: u32) {
    let mut total_ns = 0u128;

    for _ in 0..iterations {
        let start = Instant::now();
        let _result = some_work(n);
        total_ns += start.elapsed().as_nanos();
    }

    let avg_ns = total_ns / iterations as u128;
    println!("평균: {} ns ({iterations}회 반복)", avg_ns);
}

fn main() {
    bench_manual(1_000_000, 100);
}