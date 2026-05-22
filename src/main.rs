use std::time::Instant;

/// O(1) — 배열의 첫 번째 원소 반환
fn o1_access(data: &[u64]) -> u64 {
    data[0]
}

/// O(log n) — 이진 탐색
fn o_log_n_search(data: &[u64], target: u64) -> bool {
    data.binary_search(&target).is_ok()
}

/// O(n) — 선형 탐색
fn o_n_search(data: &[u64], target: u64) -> bool {
    data.iter().any(|&x| x == target)
}

/// O(n²) — 버블 정렬 (측정 목적으로 단순 구현)
fn o_n_squared_sort(mut data: Vec<u64>) -> Vec<u64> {
    let n = data.len();
    for i in 0..n {
        for j in 0..n - 1 - i {
            if data[j] > data[j + 1] {
                data.swap(j, j + 1);
            }
        }
    }
    data
}

fn measure<F, R>(label: &str, n: usize, mut f: F)
where
    F: FnMut() -> R,
{
    let start = Instant::now();
    let _result = f();
    let elapsed = start.elapsed();
    println!("{label} (n={n}): {:.3} µs", elapsed.as_secs_f64() * 1_000_000.0);
}

fn main() {
    // 정렬된 데이터 준비 (이진 탐색을 위해 정렬 필요)
    let sizes = [100, 1_000, 10_000, 100_000];

    for &n in &sizes {
        let data: Vec<u64> = (0..n as u64).collect();
        let target = n as u64 - 1; // 최악의 경우 탐색

        println!("--- n = {n} ---");

        // O(1): 크기가 바뀌어도 시간이 거의 변하지 않아야 함
        measure("O(1)  배열 접근", n, || o1_access(&data));

        // O(log n): n이 10배 커지면 약 3.3배 느려져야 함
        measure("O(log n) 이진 탐색", n, || o_log_n_search(&data, target));

        // O(n): n이 10배 커지면 약 10배 느려져야 함
        measure("O(n)  선형 탐색", n, || o_n_search(&data, target));

        // O(n²): n이 10배 커지면 약 100배 느려져야 함
        // n=100_000은 시간이 너무 걸려 n=10_000까지만 측정
        if n <= 10_000 {
            let unsorted: Vec<u64> = (0..n as u64).rev().collect();
            measure("O(n²) 버블 정렬", n, || o_n_squared_sort(unsorted.clone()));
        }

        println!();
    }
}