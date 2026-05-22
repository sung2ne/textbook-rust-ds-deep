fn sum_slice(s: &[i32]) -> i32 {
    s.iter().sum()
}

fn main() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    // &arr 은 &[i32; 5] 타입이지만,
    // 함수가 &[i32]를 기대하므로 자동으로 변환된다.
    let total = sum_slice(&arr);
    println!("합계: {}", total);  // 15

    // Vec도 마찬가지다. Vec<i32>는 Deref<Target=[i32]>를 구현한다.
    let v = vec![10, 20, 30];
    let total2 = sum_slice(&v);
    println!("합계: {}", total2);  // 60
}