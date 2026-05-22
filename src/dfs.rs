struct Graph {
    adj: Vec<Vec<usize>>,
}

impl Graph {
    fn new(n: usize) -> Self {
        Graph { adj: vec![Vec::new(); n] }
    }

    fn add_edge(&mut self, u: usize, v: usize) {
        self.adj[u].push(v);
        self.adj[v].push(u);
    }

    // --- 재귀 DFS ---

    /// 재귀 DFS로 start에서 도달 가능한 정점을 모두 방문한다.
    fn dfs_recursive(&self, start: usize) -> Vec<usize> {
        let mut visited = vec![false; self.adj.len()];
        let mut order = Vec::new();
        self.dfs_visit(start, &mut visited, &mut order);
        order
    }

    fn dfs_visit(&self, v: usize, visited: &mut Vec<bool>, order: &mut Vec<usize>) {
        visited[v] = true;
        order.push(v);
        for &neighbor in &self.adj[v] {
            if !visited[neighbor] {
                self.dfs_visit(neighbor, visited, order);
            }
        }
    }

    // --- 반복적 DFS (명시적 스택) ---

    /// 명시적 스택을 사용하는 DFS.
    fn dfs_iterative(&self, start: usize) -> Vec<usize> {
        let mut visited = vec![false; self.adj.len()];
        let mut stack = vec![start];
        let mut order = Vec::new();

        while let Some(v) = stack.pop() {
            if visited[v] {
                continue;
            }
            visited[v] = true;
            order.push(v);

            // 이웃을 역순으로 push하면 방문 순서가 재귀 DFS와 일치한다.
            for &neighbor in self.adj[v].iter().rev() {
                if !visited[neighbor] {
                    stack.push(neighbor);
                }
            }
        }

        order
    }

    // --- 사이클 탐지 (방향 그래프) ---

    /// 방향 그래프에서 사이클이 존재하는지 DFS로 탐지한다.
    /// "현재 재귀 경로 위에 있는" 정점을 in_stack으로 추적한다.
    fn has_cycle_directed(&self) -> bool {
        let n = self.adj.len();
        let mut visited = vec![false; n];
        let mut in_stack = vec![false; n];

        for start in 0..n {
            if !visited[start] {
                if self.dfs_cycle(start, &mut visited, &mut in_stack) {
                    return true;
                }
            }
        }
        false
    }

    fn dfs_cycle(&self, v: usize, visited: &mut Vec<bool>, in_stack: &mut Vec<bool>) -> bool {
        visited[v] = true;
        in_stack[v] = true;

        for &neighbor in &self.adj[v] {
            if !visited[neighbor] {
                if self.dfs_cycle(neighbor, visited, in_stack) {
                    return true;
                }
            } else if in_stack[neighbor] {
                // 현재 경로 위의 정점으로 돌아왔다 → 사이클 발견
                return true;
            }
        }

        in_stack[v] = false; // 이 정점에서의 탐색 완료, 경로에서 제거
        false
    }

    // --- 연결 요소 찾기 ---

    /// 그래프의 연결 요소 수와 각 요소에 속하는 정점 목록을 반환한다.
    fn connected_components(&self) -> Vec<Vec<usize>> {
        let n = self.adj.len();
        let mut visited = vec![false; n];
        let mut components = Vec::new();

        for start in 0..n {
            if !visited[start] {
                let mut component = Vec::new();
                self.dfs_visit(start, &mut visited, &mut component);
                components.push(component);
            }
        }

        components
    }
}

fn main() {
    let mut g = Graph::new(7);
    // 연결 요소 1: 0-1-2
    g.add_edge(0, 1);
    g.add_edge(1, 2);
    // 연결 요소 2: 3-4
    g.add_edge(3, 4);
    // 연결 요소 3: 5-6
    g.add_edge(5, 6);

    let components = g.connected_components();
    println!("연결 요소 수: {}", components.len());
    for (i, comp) in components.iter().enumerate() {
        println!("  요소 {}: {:?}", i, comp);
    }

    println!("\nDFS 재귀 방문 순서 (0에서 시작): {:?}", g.dfs_recursive(0));
    println!("DFS 반복 방문 순서 (0에서 시작): {:?}", g.dfs_iterative(0));
}