struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind { parent: (0..n).collect(), rank: vec![0; n] }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let rx = self.find(x);
        let ry = self.find(y);
        if rx == ry { return false; }
        match self.rank[rx].cmp(&self.rank[ry]) {
            std::cmp::Ordering::Less    => self.parent[rx] = ry,
            std::cmp::Ordering::Greater => self.parent[ry] = rx,
            std::cmp::Ordering::Equal   => { self.parent[ry] = rx; self.rank[rx] += 1; }
        }
        true
    }
}

#[derive(Debug)]
struct Edge {
    u: usize,
    v: usize,
    weight: u64,
}

/// Kruskal 알고리즘으로 MST를 구한다.
/// 반환값: (MST 간선 목록, 총 가중치)
fn kruskal(vertex_count: usize, mut edges: Vec<Edge>) -> (Vec<Edge>, u64) {
    // 1단계: 간선을 가중치 순으로 정렬
    edges.sort_by_key(|e| e.weight);

    let mut uf = UnionFind::new(vertex_count);
    let mut mst_edges = Vec::new();
    let mut total_weight = 0u64;

    // 2단계: 가중치가 작은 간선부터 선택
    for edge in edges {
        // 사이클이 생기지 않는 간선만 선택 (union이 true 반환)
        if uf.union(edge.u, edge.v) {
            total_weight += edge.weight;
            mst_edges.push(edge);
            // 정점 수 - 1개의 간선이 선택되면 MST 완성
            if mst_edges.len() == vertex_count - 1 {
                break;
            }
        }
    }

    (mst_edges, total_weight)
}

fn main() {
    // 6개 도시, 9개 도로 중 MST 찾기
    let edges = vec![
        Edge { u: 0, v: 1, weight: 4 },
        Edge { u: 0, v: 2, weight: 2 },
        Edge { u: 1, v: 2, weight: 1 },
        Edge { u: 1, v: 3, weight: 5 },
        Edge { u: 2, v: 3, weight: 8 },
        Edge { u: 2, v: 4, weight: 10 },
        Edge { u: 3, v: 4, weight: 2 },
        Edge { u: 3, v: 5, weight: 6 },
        Edge { u: 4, v: 5, weight: 3 },
    ];

    let (mst, total) = kruskal(6, edges);

    println!("최소 신장 트리 간선:");
    for e in &mst {
        println!("  {} -- {} (가중치: {})", e.u, e.v, e.weight);
    }
    println!("총 가중치: {}", total);
}