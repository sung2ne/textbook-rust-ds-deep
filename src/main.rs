use std::sync::atomic::{AtomicUsize, Ordering};
use std::ptr::NonNull;
use std::ops::Deref;

struct Inner<T> {
    ref_count: AtomicUsize,
    value: T,
}

pub struct MyArc<T> {
    ptr: NonNull<Inner<T>>,
}

impl<T> MyArc<T> {
    pub fn new(value: T) -> Self {
        let boxed = Box::new(Inner {
            ref_count: AtomicUsize::new(1),
            value,
        });
        MyArc {
            ptr: NonNull::new(Box::into_raw(boxed)).unwrap(),
        }
    }
}

impl<T> Clone for MyArc<T> {
    fn clone(&self) -> Self {
        let inner = unsafe { self.ptr.as_ref() };
        // Relaxed: 참조 카운터 증가는 순서 무관 (이미 ref_count > 0이 보장됨)
        inner.ref_count.fetch_add(1, Ordering::Relaxed);
        MyArc { ptr: self.ptr }
    }
}

impl<T> Drop for MyArc<T> {
    fn drop(&mut self) {
        let inner = unsafe { self.ptr.as_ref() };
        // Release: 이 드롭 이전의 모든 수정이 완료됐음을 표시
        if inner.ref_count.fetch_sub(1, Ordering::Release) == 1 {
            // Acquire: 다른 스레드의 Release를 모두 관찰한 뒤 해제
            std::sync::atomic::fence(Ordering::Acquire);
            unsafe { drop(Box::from_raw(self.ptr.as_ptr())); }
        }
    }
}

impl<T> Deref for MyArc<T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &self.ptr.as_ref().value }
    }
}

// SAFETY: Inner<T>의 접근은 ref_count 원자 연산으로 보호
unsafe impl<T: Send + Sync> Send for MyArc<T> {}
unsafe impl<T: Send + Sync> Sync for MyArc<T> {}

fn main() {
    let a = MyArc::new(42u64);
    let b = a.clone();
    println!("a = {}, b = {}", *a, *b);
    drop(a);
    println!("b = {} (a 드롭 후)", *b);
    // b 드롭 시 실제 해제
}