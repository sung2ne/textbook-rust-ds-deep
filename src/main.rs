use std::collections::VecDeque;

struct Graph {
    adj: Vec<Vec<usize>>,
}

impl Graph {
    fn new(n: usize) -> Self {
        Graph { adj: vec![Vec::new(); n] }
    }

    fn add_edge(&mut self, u: usize, v: usize) {
        self.adj[u].push(v);
        self.adj[v].push(u); // 무방향
    }

    /// BFS로 start에서 각 정점까지의 최단 거리를 반환한다.
    /// 도달 불가능한 정점은 usize::MAX로 표시한다.
    fn bfs_distances(&self, start: usize) -> Vec<usize> {
        let n = self.adj.len();
        let mut dist = vec![usize::MAX; n];
        let mut queue = VecDeque::new();

        dist[start] = 0;
        queue.push_back(start);

        while let Some(v) = queue.pop_front() {
            for &neighbor in &self.adj[v] {
                if dist[neighbor] == usize::MAX {
                    dist[neighbor] = dist[v] + 1;
                    queue.push_back(neighbor);
                }
            }
        }

        dist
    }

    /// BFS로 start에서 target까지의 최단 경로를 반환한다.
    /// 경로가 없으면 None을 반환한다.
    fn bfs_shortest_path(&self, start: usize, target: usize) -> Option<Vec<usize>> {
        let n = self.adj.len();
        let mut parent = vec![usize::MAX; n];
        let mut visited = vec![false; n];
        let mut queue = VecDeque::new();

        visited[start] = true;
        queue.push_back(start);

        while let Some(v) = queue.pop_front() {
            if v == target {
                // target에 도달했으므로 경로를 역추적한다.
                let mut path = Vec::new();
                let mut cur = target;
                while cur != start {
                    path.push(cur);
                    cur = parent[cur];
                }
                path.push(start);
                path.reverse();
                return Some(path);
            }

            for &neighbor in &self.adj[v] {
                if !visited[neighbor] {
                    visited[neighbor] = true;
                    parent[neighbor] = v;
                    queue.push_back(neighbor);
                }
            }
        }

        None
    }
}

fn main() {
    let mut g = Graph::new(6);
    // 0 - 1 - 2
    // |       |
    // 3 - 4 - 5
    g.add_edge(0, 1);
    g.add_edge(1, 2);
    g.add_edge(0, 3);
    g.add_edge(2, 5);
    g.add_edge(3, 4);
    g.add_edge(4, 5);

    let dist = g.bfs_distances(0);
    for (v, d) in dist.iter().enumerate() {
        println!("0 → {} : 거리 {}", v, d);
    }

    println!("\n0에서 5까지 최단 경로:");
    if let Some(path) = g.bfs_shortest_path(0, 5) {
        println!("{:?}", path);
    }
}