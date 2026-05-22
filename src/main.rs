use std::sync::{Arc, Mutex, Condvar};
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;

struct WorkQueue {
    queue: Mutex<Vec<Job>>,
    condvar: Condvar,
}

impl WorkQueue {
    fn new() -> Arc<Self> {
        Arc::new(WorkQueue {
            queue: Mutex::new(vec![]),
            condvar: Condvar::new(),
        })
    }

    fn push(&self, job: Job) {
        self.queue.lock().unwrap().push(job);
        self.condvar.notify_one();   // 대기 중인 워커 스레드 하나 깨우기
    }

    fn pop_blocking(&self) -> Job {
        let mut q = self.queue.lock().unwrap();
        loop {
            if let Some(job) = q.pop() {
                return job;
            }
            q = self.condvar.wait(q).unwrap();  // 락 해제 후 대기
        }
    }
}

fn main() {
    let wq = WorkQueue::new();
    let mut handles = vec![];

    // 워커 스레드 4개
    for id in 0..4 {
        let wq_clone = Arc::clone(&wq);
        handles.push(thread::spawn(move || {
            loop {
                let job = wq_clone.pop_blocking();
                job();
                // 실제 스레드 풀에서는 종료 신호를 받으면 break
                break; // 예시 단순화
            }
        }));
    }

    // 작업 추가
    for i in 0..4 {
        let wq_clone = Arc::clone(&wq);
        wq_clone.push(Box::new(move || println!("작업 {} 실행", i)));
    }

    for h in handles {
        h.join().unwrap();
    }
}