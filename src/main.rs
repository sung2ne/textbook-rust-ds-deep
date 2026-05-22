use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Arc: 스레드 간 참조 카운터 공유 (Arc는 Send + Sync)
    // Mutex: 내부 가변성을 스레드 안전하게 제공 (Mutex<T>는 T: Send이면 Send + Sync)
    let counter = Arc::new(Mutex::new(0u64));

    let mut handles = vec![];

    for _ in 0..8 {
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..1_000_000 {
                *c.lock().unwrap() += 1;
            }
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("최종 카운터: {}", *counter.lock().unwrap());
    // 항상 8_000_000 출력
}