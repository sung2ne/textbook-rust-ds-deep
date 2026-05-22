fn count<T>(tree: &Tree<T>) -> usize {
    match tree {
        None => 0,
        Some(node) => 1 + count(&node.left) + count(&node.right),
    }
}