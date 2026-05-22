use std::collections::HashMap;

fn main() {
    // with_capacity: 초기 용량 예약 (리해싱 횟수를 줄임)
    let mut map: HashMap<i32, i32> = HashMap::with_capacity(1000);
    println!("초기 capacity: {}", map.capacity());

    for i in 0..100 {
        map.insert(i, i * 2);
    }
    println!("100개 삽입 후 capacity: {}", map.capacity());
    println!("len: {}", map.len());

    // shrink_to_fit: 불필요한 여유 공간 해제
    map.shrink_to_fit();
    println!("shrink 후 capacity: {}", map.capacity());
}