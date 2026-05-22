struct MatrixGraph {
    /// matrix[u][v] = Some(weight) if edge u→v exists
    matrix: Vec<Vec<Option<u64>>>,
    vertex_count: usize,
}

impl MatrixGraph {
    fn new(vertex_count: usize) -> Self {
        MatrixGraph {
            matrix: vec![vec![None; vertex_count]; vertex_count],
            vertex_count,
        }
    }

    fn add_edge(&mut self, u: usize, v: usize, w: u64) {
        self.matrix[u][v] = Some(w);
    }

    fn add_undirected_edge(&mut self, u: usize, v: usize, w: u64) {
        self.matrix[u][v] = Some(w);
        self.matrix[v][u] = Some(w);
    }

    /// 간선 존재 여부를 O(1)에 확인한다.
    fn has_edge(&self, u: usize, v: usize) -> bool {
        self.matrix[u][v].is_some()
    }

    /// 간선 가중치를 O(1)에 반환한다.
    fn weight(&self, u: usize, v: usize) -> Option<u64> {
        self.matrix[u][v]
    }
}

fn main() {
    let mut g = MatrixGraph::new(4);
    g.add_undirected_edge(0, 1, 10);
    g.add_undirected_edge(1, 2, 20);
    g.add_undirected_edge(2, 3, 30);

    println!("0→1 간선 존재: {}", g.has_edge(0, 1));  // true
    println!("0→2 간선 존재: {}", g.has_edge(0, 2));  // false
    println!("0→1 가중치: {:?}", g.weight(0, 1));      // Some(10)
}