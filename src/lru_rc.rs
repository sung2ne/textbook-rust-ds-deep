use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

type Link<K, V> = Option<Rc<RefCell<Node<K, V>>>>;

struct Node<K, V> {
    key: K,
    val: V,
    prev: Link<K, V>,
    next: Link<K, V>,
}

impl<K, V> Node<K, V> {
    fn new(key: K, val: V) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            key,
            val,
            prev: None,
            next: None,
        }))
    }
}