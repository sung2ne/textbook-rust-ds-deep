use std::collections::VecDeque;

type Tree<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    value: T,
    left: Tree<T>,
    right: Tree<T>,
}

impl<T> Node<T> {
    fn new(value: T) -> Tree<T> {
        Some(Box::new(Node { value, left: None, right: None }))
    }
}

fn preorder<T: std::fmt::Debug>(tree: &Tree<T>) {
    if let Some(node) = tree {
        print!("{:?} ", node.value);
        preorder(&node.left);
        preorder(&node.right);
    }
}

fn inorder<T: std::fmt::Debug>(tree: &Tree<T>) {
    if let Some(node) = tree {
        inorder(&node.left);
        print!("{:?} ", node.value);
        inorder(&node.right);
    }
}

fn postorder<T: std::fmt::Debug>(tree: &Tree<T>) {
    if let Some(node) = tree {
        postorder(&node.left);
        postorder(&node.right);
        print!("{:?} ", node.value);
    }
}

fn level_order<T: std::fmt::Debug>(root: &Tree<T>) {
    let mut queue: VecDeque<&Box<Node<T>>> = VecDeque::new();
    if let Some(ref node) = root {
        queue.push_back(node);
    }
    while let Some(node) = queue.pop_front() {
        print!("{:?} ", node.value);
        if let Some(ref l) = node.left { queue.push_back(l); }
        if let Some(ref r) = node.right { queue.push_back(r); }
    }
    println!();
}

fn height<T>(tree: &Tree<T>) -> usize {
    match tree {
        None => 0,
        Some(node) => 1 + height(&node.left).max(height(&node.right)),
    }
}

fn count<T>(tree: &Tree<T>) -> usize {
    match tree {
        None => 0,
        Some(node) => 1 + count(&node.left) + count(&node.right),
    }
}

fn main() {
    //       1
    //      / \
    //     2   3
    //    / \
    //   4   5
    let mut root = Node::new(1);
    if let Some(ref mut r) = root {
        r.left = Node::new(2);
        r.right = Node::new(3);
        if let Some(ref mut l) = r.left {
            l.left = Node::new(4);
            l.right = Node::new(5);
        }
    }

    print!("전위 순회: ");  preorder(&root);   println!();
    print!("중위 순회: ");  inorder(&root);    println!();
    print!("후위 순회: ");  postorder(&root);  println!();
    print!("레벨 순회: ");  level_order(&root);

    println!("높이: {}", height(&root));   // 3
    println!("노드 수: {}", count(&root)); // 5
}