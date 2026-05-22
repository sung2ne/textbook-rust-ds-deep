mod lru_rc;
use lru_rc::LruCacheRc;

fn main() {
    let mut cache: LruCacheRc<i32, i32> = LruCacheRc::new(2);
    cache.put(1, 10);
    cache.put(2, 20);
    assert_eq!(cache.get(&1), Some(10)); // 1을 최근으로 갱신
    cache.put(3, 30);                    // 용량 초과 → 2 제거
    assert_eq!(cache.get(&2), None);     // 2는 이미 제거됨
    assert_eq!(cache.get(&3), Some(30));
    println!("Rc/RefCell LRU: 모든 검증 통과");
}