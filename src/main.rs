fn sum_to_n(n: u64) -> u64 {
    (0..n).sum()
}

fn main() {
    // 컴파일러는 sum_to_n(1000)이 항상 499500임을 알고 있음
    // 이 함수 호출 자체를 499500로 대체할 수 있음
    let result = sum_to_n(1000);
    println!("{}", result);
}