struct Graph {
    adj: Vec<Vec<usize>>,
    n: usize,
}

impl Graph {
    fn new(n: usize) -> Self {
        Graph { adj: vec![Vec::new(); n], n }
    }

    fn add_edge(&mut self, u: usize, v: usize) {
        self.adj[u].push(v);
    }

    /// 그래프의 역방향 그래프를 반환한다.
    fn reverse(&self) -> Graph {
        let mut rev = Graph::new(self.n);
        for u in 0..self.n {
            for &v in &self.adj[u] {
                rev.add_edge(v, u); // 방향 반전
            }
        }
        rev
    }

    /// DFS로 탐색 완료 순서를 finish_order에 기록한다.
    fn dfs_finish_order(&self, start: usize, visited: &mut Vec<bool>, finish_order: &mut Vec<usize>) {
        visited[start] = true;
        for &neighbor in &self.adj[start] {
            if !visited[neighbor] {
                self.dfs_finish_order(neighbor, visited, finish_order);
            }
        }
        finish_order.push(start); // 이 정점의 탐색이 완료됨
    }

    /// DFS로 start에서 도달 가능한 모든 미방문 정점을 component에 수집한다.
    fn dfs_collect(&self, start: usize, visited: &mut Vec<bool>, component: &mut Vec<usize>) {
        visited[start] = true;
        component.push(start);
        for &neighbor in &self.adj[start] {
            if !visited[neighbor] {
                self.dfs_collect(neighbor, visited, component);
            }
        }
    }

    /// Kosaraju 알고리즘으로 SCC 목록을 반환한다.
    fn kosaraju_scc(&self) -> Vec<Vec<usize>> {
        // 1단계: 정방향 DFS, 완료 순서 기록
        let mut visited = vec![false; self.n];
        let mut finish_order = Vec::new();
        for v in 0..self.n {
            if !visited[v] {
                self.dfs_finish_order(v, &mut visited, &mut finish_order);
            }
        }

        // 2단계: 역방향 그래프 생성
        let rev = self.reverse();

        // 3단계: 완료 순서의 역순으로 역방향 그래프에서 DFS
        let mut visited = vec![false; self.n];
        let mut sccs = Vec::new();
        for &v in finish_order.iter().rev() {
            if !visited[v] {
                let mut component = Vec::new();
                rev.dfs_collect(v, &mut visited, &mut component);
                sccs.push(component);
            }
        }

        sccs
    }
}

fn main() {
    // 예시 그래프
    // SCC 1: {0, 1, 2} (0→1→2→0)
    // SCC 2: {3}       (2→3이지만 3→어딘가는 없음)
    // SCC 3: {4, 5}    (4→5→4)
    let mut g = Graph::new(6);
    g.add_edge(0, 1);
    g.add_edge(1, 2);
    g.add_edge(2, 0); // SCC: {0, 1, 2}
    g.add_edge(2, 3); // {0,1,2} → {3}
    g.add_edge(4, 5);
    g.add_edge(5, 4); // SCC: {4, 5}
    g.add_edge(3, 4); // {3} → {4, 5}

    let sccs = g.kosaraju_scc();
    println!("SCC 수: {}", sccs.len());
    for (i, scc) in sccs.iter().enumerate() {
        let mut sorted_scc = scc.clone();
        sorted_scc.sort();
        println!("  SCC {}: {:?}", i, sorted_scc);
    }

    // 실용 예시: 순환 의존성 검출
    println!("\n--- 마이크로서비스 순환 의존성 검출 ---");
    let service_names = ["auth", "user", "order", "payment", "notification"];
    let mut services = Graph::new(5);
    // auth → user (auth가 user에 의존)
    services.add_edge(0, 1);
    // user → order
    services.add_edge(1, 2);
    // order → payment
    services.add_edge(2, 3);
    // payment → order (순환!)
    services.add_edge(3, 2);
    // order → notification
    services.add_edge(2, 4);

    let sccs = services.kosaraju_scc();
    for scc in &sccs {
        if scc.len() > 1 {
            let names: Vec<&str> = scc.iter().map(|&i| service_names[i]).collect();
            println!("순환 의존성 발견: {:?}", names);
        }
    }
}