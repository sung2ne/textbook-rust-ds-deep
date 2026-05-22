fn main() {
    // extend: 소스 크기를 미리 알고 한 번만 재할당
    let source = vec![1u64, 2, 3, 4, 5];
    let mut v = Vec::new();
    v.extend(source.iter().copied());

    // 반복 push: 크기를 모르므로 여러 번 재할당 가능
    let mut v2 = Vec::new();
    for &x in &source {
        v2.push(x);
    }
}