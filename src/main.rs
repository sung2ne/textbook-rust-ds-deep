use petgraph::graph::{DiGraph, UnGraph, NodeIndex};
use petgraph::algo::dijkstra;

fn main() {
    // --- 무방향 가중치 그래프 ---
    // UnGraph<정점데이터타입, 간선데이터타입>
    let mut g: UnGraph<&str, u32> = UnGraph::new_undirected();

    let seoul   = g.add_node("서울");
    let incheon = g.add_node("인천");
    let suwon   = g.add_node("수원");
    let daejeon = g.add_node("대전");
    let daegu   = g.add_node("대구");
    let busan   = g.add_node("부산");

    g.add_edge(seoul, incheon, 27);
    g.add_edge(seoul, suwon, 47);
    g.add_edge(seoul, daejeon, 160);
    g.add_edge(incheon, suwon, 36);
    g.add_edge(suwon, daejeon, 130);
    g.add_edge(daejeon, daegu, 120);
    g.add_edge(daegu, busan, 90);
    g.add_edge(daejeon, busan, 230);

    println!("정점 수: {}", g.node_count());
    println!("간선 수: {}", g.edge_count());

    // --- Dijkstra 최단 경로 ---
    // petgraph의 dijkstra는 HashMap<NodeIndex, 거리>를 반환한다.
    let result = dijkstra(&g, seoul, Some(busan), |e| *e.weight());

    if let Some(&dist) = result.get(&busan) {
        println!("\n서울 → 부산 최단 거리: {}km", dist);
    }

    // 서울로부터 모든 도시까지의 거리 출력
    println!("\n서울로부터의 최단 거리:");
    let nodes = [
        (seoul, "서울"), (incheon, "인천"), (suwon, "수원"),
        (daejeon, "대전"), (daegu, "대구"), (busan, "부산"),
    ];
    for (node, name) in &nodes {
        if let Some(&dist) = result.get(node) {
            println!("  → {} : {}km", name, dist);
        }
    }
}