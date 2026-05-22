// 작고 단순한 키 타입: Copy 구현
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct NodeId(u32);

// 힙 자원을 포함하는 노드: Move만
#[derive(Debug)]
struct GraphNode {
    id: NodeId,          // Copy 타입
    label: String,       // 힙 자원, Move
    edges: Vec<NodeId>,  // 힙 자원, Move
}

fn main() {
    let id1 = NodeId(1);
    let id2 = id1; // Copy: id1도 여전히 유효

    let node = GraphNode {
        id: NodeId(1),
        label: String::from("시작"),
        edges: vec![NodeId(2), NodeId(3)],
    };

    println!("노드 ID: {:?}", node.id);
    println!("복사된 ID: {:?}", id2);
}