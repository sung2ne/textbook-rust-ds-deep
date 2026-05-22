fn first_two(data: &Vec<i32>) -> &[i32] {
    &data[..2.min(data.len())] // 인자로 받은 데이터의 슬라이스 반환
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let slice = first_two(&numbers);
    println!("{:?}", slice); // [1, 2]
}