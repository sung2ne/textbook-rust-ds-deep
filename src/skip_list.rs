use rand::Rng;
use std::ptr;

const MAX_LEVEL: usize = 16;
const P: f64 = 0.5;

struct Node<K, V> {
    key: K,
    val: V,
    // forward[i]는 레벨 i에서 다음 노드를 가리킨다
    forward: Vec<*mut Node<K, V>>,
}

impl<K, V> Node<K, V> {
    fn new(key: K, val: V, level: usize) -> *mut Self {
        Box::into_raw(Box::new(Node {
            key,
            val,
            forward: vec![ptr::null_mut(); level + 1],
        }))
    }

    /// 더미 헤드용 — key/val 없이 포워드 배열만
    fn new_head(max_level: usize) -> *mut Self
    where
        K: Default,
        V: Default,
    {
        Box::into_raw(Box::new(Node {
            key: K::default(),
            val: V::default(),
            forward: vec![ptr::null_mut(); max_level + 1],
        }))
    }
}

pub struct SkipList<K, V> {
    head: *mut Node<K, V>,
    level: usize,   // 현재 사용 중인 최고 레벨
    len: usize,
}