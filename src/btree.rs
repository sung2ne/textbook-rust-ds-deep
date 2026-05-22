pub struct BTreeNode<K, V> {
    keys: Vec<K>,
    vals: Vec<V>,
    children: Vec<Box<BTreeNode<K, V>>>,
    is_leaf: bool,
}

impl<K: Ord + Clone, V: Clone> BTreeNode<K, V> {
    fn new(is_leaf: bool) -> Self {
        BTreeNode {
            keys: Vec::new(),
            vals: Vec::new(),
            children: Vec::new(),
            is_leaf,
        }
    }
}

pub struct BTree<K, V> {
    root: Box<BTreeNode<K, V>>,
    t: usize,   // 최소 차수 (minimum degree)
}

impl<K: Ord + Clone, V: Clone> BTree<K, V> {
    pub fn new(t: usize) -> Self {
        assert!(t >= 2, "B-Tree 최소 차수는 2 이상이어야 합니다");
        BTree {
            root: Box::new(BTreeNode::new(true)),
            t,
        }
    }
}