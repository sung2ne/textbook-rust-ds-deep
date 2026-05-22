use std::collections::HashMap;
use std::ptr;

struct Node<K, V> {
    key: K,
    val: V,
    prev: *mut Node<K, V>,
    next: *mut Node<K, V>,
}

impl<K, V> Node<K, V> {
    fn new(key: K, val: V) -> *mut Self {
        Box::into_raw(Box::new(Node {
            key,
            val,
            prev: ptr::null_mut(),
            next: ptr::null_mut(),
        }))
    }
}

pub struct LruCacheUnsafe<K, V> {
    cap: usize,
    map: HashMap<K, *mut Node<K, V>>,
    head: *mut Node<K, V>,   // 더미 헤드
    tail: *mut Node<K, V>,   // 더미 테일
}