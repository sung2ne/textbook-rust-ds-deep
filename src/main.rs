use std::collections::{HashSet, BTreeSet};

fn main() {
    // 이미 방문한 URL 추적 — 순서 불필요, 빠른 조회 우선
    let mut crawled: HashSet<String> = HashSet::new();
    crawled.insert("https://example.com".to_string());
    crawled.insert("https://example.com/about".to_string());

    // 이벤트 발생 날짜 집합 — 날짜 순으로 순회 필요
    let mut event_dates: BTreeSet<String> = BTreeSet::new();
    event_dates.insert("2026-03-15".to_string());
    event_dates.insert("2026-01-20".to_string());
    event_dates.insert("2026-05-01".to_string());

    println!("이벤트 날짜 (정렬됨):");
    for date in &event_dates {
        println!("  {}", date);
    }

    // 2026-02 이후 이벤트만
    println!("\n2월 이후 이벤트:");
    for date in event_dates.range("2026-02".to_string()..) {
        println!("  {}", date);
    }
}