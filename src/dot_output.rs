use petgraph::graph::DiGraph;
use petgraph::dot::{Dot, Config};

fn main() {
    let mut g: DiGraph<&str, &str> = DiGraph::new();
    let a = g.add_node("A");
    let b = g.add_node("B");
    let c = g.add_node("C");

    g.add_edge(a, b, "10");
    g.add_edge(b, c, "20");
    g.add_edge(a, c, "25");

    // DOT 형식으로 출력
    let dot = Dot::with_config(&g, &[Config::EdgeNoLabel]);
    println!("{:?}", dot);

    // 간선 레이블 포함 출력
    let dot_with_labels = Dot::new(&g);
    println!("\n--- 레이블 포함 ---");
    println!("{:?}", dot_with_labels);
}