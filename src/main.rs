mod skip_list;
use skip_list::SkipList;

fn main() {
    let mut sl: SkipList<i32, &str> = SkipList::new();

    sl.insert(3, "three");
    sl.insert(1, "one");
    sl.insert(4, "four");
    sl.insert(1, "ONE"); // 중복 키 — 값 갱신
    sl.insert(5, "five");
    sl.insert(9, "nine");

    assert_eq!(sl.get(&1), Some(&"ONE"));
    assert_eq!(sl.get(&4), Some(&"four"));
    assert_eq!(sl.get(&2), None);

    assert!(sl.remove(&3));
    assert_eq!(sl.get(&3), None);
    assert!(!sl.remove(&3)); // 이미 없음

    println!("Skip List 원소 수: {}", sl.len());
    println!("Skip List 검증 완료");
}