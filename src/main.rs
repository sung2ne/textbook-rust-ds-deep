fn main() {
    let mut bst = BinarySearchTree::new();

    for v in [5, 3, 7, 1, 4, 6, 8] {
        bst.insert(v);
    }

    println!("contains(4): {}", bst.contains(&4)); // true
    println!("contains(9): {}", bst.contains(&9)); // false
    println!("min: {:?}", bst.min());               // Some(1)
    println!("max: {:?}", bst.max());               // Some(8)

    bst.delete(&3);
    println!("contains(3) after delete: {}", bst.contains(&3)); // false
    println!("contains(4) after delete: {}", bst.contains(&4)); // true
}