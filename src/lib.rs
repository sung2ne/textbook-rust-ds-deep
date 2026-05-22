// AVL 노드 (높이 정보 포함)
#[derive(Debug)]
struct AvlNode<T: Ord> {
    value: T,
    height: i32,
    left: AvlTree<T>,
    right: AvlTree<T>,
}

type AvlTree<T> = Option<Box<AvlNode<T>>>;

fn node_height<T: Ord>(tree: &AvlTree<T>) -> i32 {
    tree.as_ref().map_or(0, |n| n.height)
}

// 오른쪽 회전 (LL 케이스)
fn rotate_right<T: Ord>(mut z: Box<AvlNode<T>>) -> Box<AvlNode<T>> {
    let mut y = z.left.take().expect("rotate_right: no left child");

    // y의 오른쪽을 z의 왼쪽으로 이동
    z.left = y.right.take();
    z.height = 1 + node_height(&z.left).max(node_height(&z.right));

    // z를 y의 오른쪽으로
    y.right = Some(z);
    y.height = 1 + node_height(&y.left).max(node_height(&y.right));

    y  // y가 새 루트
}