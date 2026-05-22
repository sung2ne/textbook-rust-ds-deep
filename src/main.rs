use rayon::prelude::*;

fn is_prime(n: u64) -> bool {
    if n < 2 { return false; }
    if n == 2 { return true; }
    if n % 2 == 0 { return false; }
    let limit = (n as f64).sqrt() as u64 + 1;
    !(3..limit).step_by(2).any(|i| n % i == 0)
}

fn main() {
    let limit = 100_000u64;

    // 병렬 소수 찾기: filter는 병렬로, collect는 순서를 유지하며 합침
    let primes: Vec<u64> = (2..=limit)
        .into_par_iter()
        .filter(|&n| is_prime(n))
        .collect();

    println!("{}까지 소수 개수: {}", limit, primes.len());
    println!("마지막 5개: {:?}", &primes[primes.len()-5..]);
}