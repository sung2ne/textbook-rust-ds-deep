use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct TreeNode {
    value: i32,
    parent: Option<Weak<RefCell<TreeNode>>>, // Weak: 소유권 없이 참조
    children: Vec<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(value: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(TreeNode {
            value,
            parent: None,
            children: vec![],
        }))
    }
}

fn main() {
    let root = TreeNode::new(1);
    let child = TreeNode::new(2);

    // 부모 → 자식: Rc (소유)
    root.borrow_mut().children.push(Rc::clone(&child));

    // 자식 → 부모: Weak (소유 없이 참조)
    child.borrow_mut().parent = Some(Rc::downgrade(&root));

    // Weak 참조로 부모에 접근
    if let Some(parent_weak) = &child.borrow().parent {
        if let Some(parent) = parent_weak.upgrade() {
            println!("부모 값: {}", parent.borrow().value);
        }
    }

    println!("root 참조 카운터: {}", Rc::strong_count(&root)); // 1 (Weak은 카운트 안 됨)
}