fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    // 이터레이션 중 수정하려면 retain, iter_mut, 또는 인덱스를 사용한다.
    // 다음은 컴파일 에러가 난다:
    // for x in &v {
    //     v.push(*x * 2);  // error: cannot borrow `v` as mutable
    // }

    // 올바른 방법: 먼저 변환할 내용을 모으고, 이후 수정
    let to_add: Vec<i32> = v.iter().map(|&x| x * 2).collect();
    v.extend(to_add);
    println!("{:?}", v);  // [1, 2, 3, 4, 5, 2, 4, 6, 8, 10]
}