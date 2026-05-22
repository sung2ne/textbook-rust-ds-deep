use std::collections::BTreeMap;

fn main() {
    let mut map = BTreeMap::new();
    map.insert("apple", 3);
    map.insert("banana", 1);
    map.insert("cherry", 5);

    // 키가 항상 오름차순으로 정렬됨
    for (key, val) in &map {
        println!("{}: {}", key, val);
    }
    // apple: 3
    // banana: 1
    // cherry: 5

    // 범위 쿼리도 가능
    use std::ops::Bound::Included;
    for (key, val) in map.range((Included("banana"), Included("cherry"))) {
        println!("range: {}: {}", key, val);
    }
}