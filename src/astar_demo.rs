use petgraph::graph::UnGraph;
use petgraph::algo::astar;

fn main() {
    let mut g: UnGraph<(i32, i32), u32> = UnGraph::new_undirected();
    // 정점 데이터로 2D 좌표를 저장
    let n00 = g.add_node((0, 0));
    let n10 = g.add_node((1, 0));
    let n20 = g.add_node((2, 0));
    let n01 = g.add_node((0, 1));
    let n11 = g.add_node((1, 1));
    let n21 = g.add_node((2, 1));

    g.add_edge(n00, n10, 1);
    g.add_edge(n10, n20, 1);
    g.add_edge(n00, n01, 1);
    g.add_edge(n10, n11, 1);
    g.add_edge(n20, n21, 1);
    g.add_edge(n01, n11, 1);
    g.add_edge(n11, n21, 1);

    // n00에서 n21까지 A* 탐색
    // 휴리스틱: 맨해튼 거리
    let result = astar(
        &g,
        n00,
        |finish| finish == n21,
        |e| *e.weight(),
        |n| {
            let (x, y) = g[n];
            let (gx, gy) = g[n21];
            ((gx - x).abs() + (gy - y).abs()) as u32
        },
    );

    if let Some((cost, path)) = result {
        println!("A* 최단 비용: {}", cost);
        let coords: Vec<(i32, i32)> = path.iter().map(|&n| g[n]).collect();
        println!("경로: {:?}", coords);
    }
}