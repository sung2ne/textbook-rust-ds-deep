fn main() {
    let mut v: Vec<i32> = Vec::with_capacity(1000);
    for i in 0..10 {
        v.push(i);
    }
    println!("shrink 전: len={}, cap={}", v.len(), v.capacity());
    // len=10, cap=1000

    v.shrink_to_fit();
    println!("shrink 후: len={}, cap={}", v.len(), v.capacity());
    // len=10, cap=10  (또는 10 이상의 최적화된 값)
}