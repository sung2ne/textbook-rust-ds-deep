use std::collections::BTreeMap;

fn main() {
    let mut scores: BTreeMap<String, u32> = BTreeMap::new();
    scores.insert("charlie".to_string(), 85);
    scores.insert("alice".to_string(), 92);
    scores.insert("bob".to_string(), 78);

    // 삽입 순서와 무관하게 알파벳 순으로 출력됨
    for (name, score) in &scores {
        println!("{name}: {score}");
    }

    // 범위 쿼리: 'a'에서 'c' 사이 이름만 조회
    let range: Vec<_> = scores.range("a".to_string()..="c".to_string()).collect();
    println!("a~c range: {range:?}");
}