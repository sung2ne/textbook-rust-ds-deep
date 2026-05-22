// 나쁜 예: 에러를 이해하지 않고 unsafe로 우회
fn bad_example(data: &mut Vec<i32>) {
    let ptr = data.as_mut_ptr();
    // 컴파일러가 막는 이유가 있는데 unsafe로 강제 통과
    unsafe {
        let r1 = &mut *ptr;
        let r2 = &mut *ptr; // 실제로 aliasing 발생 → 미정의 동작
        *r1 = 1;
        *r2 = 2; // r1과 r2가 같은 주소를 가리킴
    }
}