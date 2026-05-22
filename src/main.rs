use tokio::sync::Notify;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let notify = Arc::new(Notify::new());
    let notify2 = Arc::clone(&notify);

    // 대기 태스크
    let waiter = tokio::spawn(async move {
        println!("이벤트 대기 중...");
        notify2.notified().await;
        println!("이벤트 발생!");
    });

    sleep(Duration::from_millis(100)).await;
    notify.notify_one(); // 대기 중인 태스크 하나 깨우기

    waiter.await.unwrap();
}