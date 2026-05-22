/// 가중치 있는 방향 그래프 (인접 리스트 표현)
struct Graph {
    /// adj[v] = [(neighbor, weight), ...]
    adj: Vec<Vec<(usize, u64)>>,
    vertex_count: usize,
}

impl Graph {
    /// 정점 수를 지정해 빈 그래프를 만든다.
    fn new(vertex_count: usize) -> Self {
        Graph {
            adj: vec![Vec::new(); vertex_count],
            vertex_count,
        }
    }

    /// 방향 간선 u → v (가중치 w)를 추가한다.
    fn add_edge(&mut self, u: usize, v: usize, w: u64) {
        assert!(u < self.vertex_count && v < self.vertex_count);
        self.adj[u].push((v, w));
    }

    /// 무방향 간선을 추가하려면 양쪽 방향 모두 삽입한다.
    fn add_undirected_edge(&mut self, u: usize, v: usize, w: u64) {
        self.add_edge(u, v, w);
        self.add_edge(v, u, w);
    }

    /// 정점 v의 이웃 목록을 반환한다.
    fn neighbors(&self, v: usize) -> &[(usize, u64)] {
        &self.adj[v]
    }

    /// 정점 수를 반환한다.
    fn vertex_count(&self) -> usize {
        self.vertex_count
    }

    /// 간선 수를 반환한다 (방향 그래프 기준).
    fn edge_count(&self) -> usize {
        self.adj.iter().map(|list| list.len()).sum()
    }
}

fn main() {
    // 5개 정점 그래프: 서울(0), 인천(1), 수원(2), 대전(3), 대구(4)
    let mut g = Graph::new(5);

    g.add_undirected_edge(0, 1, 27);  // 서울 ↔ 인천 (27km)
    g.add_undirected_edge(0, 2, 47);  // 서울 ↔ 수원 (47km)
    g.add_undirected_edge(2, 3, 130); // 수원 ↔ 대전 (130km)
    g.add_undirected_edge(3, 4, 120); // 대전 ↔ 대구 (120km)
    g.add_undirected_edge(0, 3, 160); // 서울 ↔ 대전 (160km)

    println!("정점 수: {}, 간선 수(방향): {}", g.vertex_count(), g.edge_count());

    println!("\n서울(0)의 이웃:");
    for (neighbor, weight) in g.neighbors(0) {
        println!("  → {} ({}km)", neighbor, weight);
    }
}