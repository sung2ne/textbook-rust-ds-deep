use crossbeam_channel::{unbounded, tick, after};
use std::time::Duration;

fn main() {
    let (work_tx, work_rx) = unbounded::<String>();
    let ticker = tick(Duration::from_millis(100)); // 100ms마다 신호
    let deadline = after(Duration::from_secs(1));  // 1초 후 신호

    // 워커 스레드가 작업을 보냄
    let tx = work_tx.clone();
    std::thread::spawn(move || {
        for i in 0..5 {
            std::thread::sleep(Duration::from_millis(150));
            tx.send(format!("작업 {}", i)).unwrap();
        }
    });
    drop(work_tx);

    loop {
        crossbeam_channel::select! {
            recv(work_rx) -> msg => {
                match msg {
                    Ok(m) => println!("작업 수신: {}", m),
                    Err(_) => {
                        println!("채널 닫힘");
                        break;
                    }
                }
            },
            recv(ticker) -> _ => {
                println!("틱: 주기적 점검");
            },
            recv(deadline) -> _ => {
                println!("타임아웃: 1초 경과");
                break;
            },
        }
    }
}