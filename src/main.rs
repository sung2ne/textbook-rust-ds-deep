fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    // 빌림 검사기 거부 (같은 슬라이스의 두 가변 참조)
    // let left = &mut v[0..2];
    // let right = &mut v[3..5];

    // split_at_mut으로 안전하게 분리
    let (left, right) = v.split_at_mut(3);
    left[0] = 10;
    right[0] = 40;
    println!("{:?}", v);  // [10, 2, 3, 40, 5]
}