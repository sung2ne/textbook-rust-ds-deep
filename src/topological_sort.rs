use std::collections::VecDeque;

struct DiGraph {
    adj: Vec<Vec<usize>>,
    in_degree: Vec<usize>,
}

impl DiGraph {
    fn new(n: usize) -> Self {
        DiGraph {
            adj: vec![Vec::new(); n],
            in_degree: vec![0; n],
        }
    }

    /// 방향 간선 u → v 추가
    fn add_edge(&mut self, u: usize, v: usize) {
        self.adj[u].push(v);
        self.in_degree[v] += 1;
    }

    /// Kahn 알고리즘으로 위상 정렬을 수행한다.
    /// 사이클이 없으면 Ok(순서), 있으면 Err을 반환한다.
    fn topological_sort(&self) -> Result<Vec<usize>, &'static str> {
        let n = self.adj.len();
        let mut in_deg = self.in_degree.clone();
        let mut queue = VecDeque::new();
        let mut result = Vec::new();

        // 진입 차수가 0인 정점을 모두 큐에 넣는다.
        for v in 0..n {
            if in_deg[v] == 0 {
                queue.push_back(v);
            }
        }

        while let Some(v) = queue.pop_front() {
            result.push(v);
            for &neighbor in &self.adj[v] {
                in_deg[neighbor] -= 1;
                if in_deg[neighbor] == 0 {
                    queue.push_back(neighbor);
                }
            }
        }

        if result.len() == n {
            Ok(result)
        } else {
            Err("사이클이 존재합니다 — 위상 정렬 불가")
        }
    }
}

fn main() {
    // 빌드 의존성 예시
    // 0=libc, 1=libm, 2=libssl, 3=openssl, 4=curl, 5=myapp
    // myapp(5) → curl(4), openssl(3)
    // curl(4) → openssl(3), libssl(2)
    // openssl(3) → libssl(2), libm(1), libc(0)
    // libssl(2) → libm(1), libc(0)
    // libm(1) → libc(0)
    let package_names = ["libc", "libm", "libssl", "openssl", "curl", "myapp"];
    let mut g = DiGraph::new(6);

    g.add_edge(5, 4); // myapp → curl
    g.add_edge(5, 3); // myapp → openssl
    g.add_edge(4, 3); // curl → openssl
    g.add_edge(4, 2); // curl → libssl
    g.add_edge(3, 2); // openssl → libssl
    g.add_edge(3, 1); // openssl → libm
    g.add_edge(3, 0); // openssl → libc
    g.add_edge(2, 1); // libssl → libm
    g.add_edge(2, 0); // libssl → libc
    g.add_edge(1, 0); // libm → libc

    match g.topological_sort() {
        Ok(order) => {
            print!("빌드 순서: ");
            let names: Vec<&str> = order.iter().map(|&i| package_names[i]).collect();
            println!("{}", names.join(" → "));
        }
        Err(msg) => println!("오류: {}", msg),
    }

    // 사이클이 있는 경우
    let mut cyclic = DiGraph::new(3);
    cyclic.add_edge(0, 1);
    cyclic.add_edge(1, 2);
    cyclic.add_edge(2, 0); // 사이클: 0→1→2→0
    match cyclic.topological_sort() {
        Ok(_) => println!("위상 정렬 성공"),
        Err(msg) => println!("순환 의존성 탐지: {}", msg),
    }
}