#[derive(Clone)]
struct Edge {
    from: usize,
    to: usize,
    weight: i64, // 음수도 허용하므로 i64 사용
}

struct BellmanFord {
    edges: Vec<Edge>,
    vertex_count: usize,
}

impl BellmanFord {
    fn new(vertex_count: usize) -> Self {
        BellmanFord {
            edges: Vec::new(),
            vertex_count,
        }
    }

    fn add_edge(&mut self, from: usize, to: usize, weight: i64) {
        self.edges.push(Edge { from, to, weight });
    }

    /// Bellman-Ford 알고리즘으로 start에서 모든 정점까지의 최단 거리를 계산한다.
    /// 음수 사이클이 없으면 Ok(distances), 있으면 Err을 반환한다.
    fn run(&self, start: usize) -> Result<Vec<i64>, &'static str> {
        const INF: i64 = i64::MAX / 2;
        let n = self.vertex_count;
        let mut dist = vec![INF; n];
        dist[start] = 0;

        // V-1번 완화 반복
        for _ in 0..(n - 1) {
            for edge in &self.edges {
                if dist[edge.from] != INF {
                    let new_dist = dist[edge.from] + edge.weight;
                    if new_dist < dist[edge.to] {
                        dist[edge.to] = new_dist;
                    }
                }
            }
        }

        // V번째 반복에서 완화가 일어나면 음수 사이클 존재
        for edge in &self.edges {
            if dist[edge.from] != INF {
                if dist[edge.from] + edge.weight < dist[edge.to] {
                    return Err("음수 사이클이 존재합니다");
                }
            }
        }

        Ok(dist)
    }
}

fn main() {
    let mut bf = BellmanFord::new(5);
    bf.add_edge(0, 1, 6);
    bf.add_edge(0, 3, 7);
    bf.add_edge(1, 2, 5);
    bf.add_edge(1, 3, 8);
    bf.add_edge(1, 4, -4);
    bf.add_edge(2, 1, -2);
    bf.add_edge(3, 2, -3);
    bf.add_edge(3, 4, 9);
    bf.add_edge(4, 0, 2);
    bf.add_edge(4, 2, 7);

    match bf.run(0) {
        Ok(dist) => {
            println!("0번 정점에서의 최단 거리:");
            for (v, d) in dist.iter().enumerate() {
                if *d == i64::MAX / 2 {
                    println!("  → {} : 도달 불가", v);
                } else {
                    println!("  → {} : {}", v, d);
                }
            }
        }
        Err(msg) => println!("오류: {}", msg),
    }

    // 음수 사이클 탐지 예시
    println!("\n--- 음수 사이클이 있는 그래프 ---");
    let mut bf2 = BellmanFord::new(3);
    bf2.add_edge(0, 1, 1);
    bf2.add_edge(1, 2, -3);
    bf2.add_edge(2, 0, 1); // 0→1→2→0 합: 1 + (-3) + 1 = -1 (음수 사이클!)
    match bf2.run(0) {
        Ok(_) => println!("음수 사이클 없음"),
        Err(msg) => println!("탐지: {}", msg),
    }
}