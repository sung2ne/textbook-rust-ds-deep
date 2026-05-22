use crossbeam_epoch::{self as epoch, Atomic, Owned, Shared};
use std::sync::atomic::Ordering;

struct Node<T> {
    value: T,
    next: Atomic<Node<T>>,
}

pub struct EpochStack<T> {
    head: Atomic<Node<T>>,
}

impl<T> EpochStack<T> {
    pub fn new() -> Self {
        EpochStack {
            head: Atomic::null(),
        }
    }

    pub fn push(&self, value: T) {
        let node = Owned::new(Node {
            value,
            next: Atomic::null(),
        });

        let guard = epoch::pin(); // 현재 스레드가 epoch에 참여

        loop {
            let old_head = self.head.load(Ordering::Relaxed, &guard);
            node.next.store(old_head, Ordering::Relaxed);

            match self.head.compare_exchange(
                old_head,
                node,
                Ordering::Release,
                Ordering::Relaxed,
                &guard,
            ) {
                Ok(_) => break,
                Err(e) => {
                    // e.new는 실패한 Owned를 반환
                    let _ = e.new; // 재시도에서 재사용 (실제로는 루프 변수로)
                    break; // 예시 단순화
                }
            }
        }
    }

    pub fn pop(&self) -> Option<T> {
        let guard = epoch::pin();

        loop {
            let old_head = self.head.load(Ordering::Acquire, &guard);

            match unsafe { old_head.as_ref() } {
                None => return None,
                Some(node) => {
                    let next = node.next.load(Ordering::Relaxed, &guard);

                    if self.head.compare_exchange(
                        old_head,
                        next,
                        Ordering::AcqRel,
                        Ordering::Relaxed,
                        &guard,
                    ).is_ok() {
                        // 즉시 해제하지 않고 defer_destroy로 예약
                        // guard가 드롭되고 모든 스레드가 새 epoch으로 이동하면 실제 해제
                        unsafe {
                            let value = std::ptr::read(&(*old_head.as_raw()).value);
                            guard.defer_destroy(old_head);
                            return Some(value);
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    let stack = EpochStack::new();

    stack.push(10);
    stack.push(20);
    stack.push(30);

    println!("{:?}", stack.pop()); // Some(30)
    println!("{:?}", stack.pop()); // Some(20)
}