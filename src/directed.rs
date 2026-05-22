use petgraph::graph::DiGraph;
use petgraph::algo::{toposort, kosaraju_scc};

fn main() {
    // 패키지 의존성 그래프 (방향: A → B는 "A가 B에 의존한다")
    // app → http → socket
    // app → json
    // http → json
    let mut deps: DiGraph<&str, ()> = DiGraph::new();

    let app    = deps.add_node("app");
    let http   = deps.add_node("http");
    let json   = deps.add_node("json");
    let socket = deps.add_node("socket");

    deps.add_edge(app, http, ());
    deps.add_edge(app, json, ());
    deps.add_edge(http, json, ());
    deps.add_edge(http, socket, ());

    // 위상 정렬: 의존성이 없는 패키지부터 빌드 순서를 결정
    match toposort(&deps, None) {
        Ok(order) => {
            print!("빌드 순서: ");
            let names: Vec<&str> = order.iter().map(|&n| deps[n]).collect();
            println!("{}", names.join(" → "));
        }
        Err(_) => println!("사이클이 있어 위상 정렬 불가"),
    }

    // --- SCC (강한 연결 요소) ---
    // SCC가 의미 있으려면 방향 사이클이 필요하므로 새 그래프를 만든다.
    let mut scc_graph: DiGraph<&str, ()> = DiGraph::new();
    let a = scc_graph.add_node("A");
    let b = scc_graph.add_node("B");
    let c = scc_graph.add_node("C");
    let d = scc_graph.add_node("D");

    // A→B→C→A (강한 연결 요소 1)
    scc_graph.add_edge(a, b, ());
    scc_graph.add_edge(b, c, ());
    scc_graph.add_edge(c, a, ());
    // D는 독립 (강한 연결 요소 2)
    scc_graph.add_edge(b, d, ());

    let sccs = kosaraju_scc(&scc_graph);
    println!("\n강한 연결 요소 수: {}", sccs.len());
    for (i, scc) in sccs.iter().enumerate() {
        let names: Vec<&str> = scc.iter().map(|&n| scc_graph[n]).collect();
        println!("  SCC {}: {:?}", i, names);
    }
}