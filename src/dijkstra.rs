use std::collections::BinaryHeap;
use std::cmp::Reverse;

const INF: u64 = u64::MAX / 2;

struct WeightedGraph {
    adj: Vec<Vec<(usize, u64)>>,
}

impl WeightedGraph {
    fn new(n: usize) -> Self {
        WeightedGraph { adj: vec![Vec::new(); n] }
    }

    fn add_edge(&mut self, u: usize, v: usize, w: u64) {
        self.adj[u].push((v, w));
        self.adj[v].push((u, w)); // 무방향
    }

    /// Dijkstra 알고리즘으로 start에서 모든 정점까지의 최단 거리를 반환한다.
    fn dijkstra(&self, start: usize) -> Vec<u64> {
        let n = self.adj.len();
        let mut dist = vec![INF; n];
        // BinaryHeap은 최대 힙이므로 Reverse로 최소 힙처럼 사용한다.
        // 요소: Reverse((거리, 정점))
        let mut heap: BinaryHeap<Reverse<(u64, usize)>> = BinaryHeap::new();

        dist[start] = 0;
        heap.push(Reverse((0, start)));

        while let Some(Reverse((d, v))) = heap.pop() {
            // 이미 더 짧은 거리로 확정된 정점이면 건너뛴다.
            if d > dist[v] {
                continue;
            }

            for &(neighbor, weight) in &self.adj[v] {
                let new_dist = dist[v] + weight;
                if new_dist < dist[neighbor] {
                    dist[neighbor] = new_dist;
                    heap.push(Reverse((new_dist, neighbor)));
                }
            }
        }

        dist
    }

    /// Dijkstra + 경로 역추적: start에서 target까지의 최단 경로를 반환한다.
    fn dijkstra_path(&self, start: usize, target: usize) -> Option<(u64, Vec<usize>)> {
        let n = self.adj.len();
        let mut dist = vec![INF; n];
        let mut parent = vec![usize::MAX; n];
        let mut heap: BinaryHeap<Reverse<(u64, usize)>> = BinaryHeap::new();

        dist[start] = 0;
        heap.push(Reverse((0, start)));

        while let Some(Reverse((d, v))) = heap.pop() {
            if d > dist[v] {
                continue;
            }
            if v == target {
                break; // 목표 도달 시 조기 종료
            }
            for &(neighbor, weight) in &self.adj[v] {
                let new_dist = dist[v] + weight;
                if new_dist < dist[neighbor] {
                    dist[neighbor] = new_dist;
                    parent[neighbor] = v;
                    heap.push(Reverse((new_dist, neighbor)));
                }
            }
        }

        if dist[target] == INF {
            return None;
        }

        // 경로 역추적
        let mut path = Vec::new();
        let mut cur = target;
        while cur != start {
            path.push(cur);
            cur = parent[cur];
        }
        path.push(start);
        path.reverse();

        Some((dist[target], path))
    }
}

fn main() {
    // 예시: 0=서울, 1=인천, 2=수원, 3=대전, 4=대구, 5=부산
    let mut g = WeightedGraph::new(6);
    g.add_edge(0, 1, 27);  // 서울-인천
    g.add_edge(0, 2, 47);  // 서울-수원
    g.add_edge(0, 3, 160); // 서울-대전
    g.add_edge(1, 2, 36);  // 인천-수원
    g.add_edge(2, 3, 130); // 수원-대전
    g.add_edge(3, 4, 120); // 대전-대구
    g.add_edge(4, 5, 90);  // 대구-부산
    g.add_edge(3, 5, 230); // 대전-부산

    let distances = g.dijkstra(0);
    let city_names = ["서울", "인천", "수원", "대전", "대구", "부산"];
    println!("서울로부터의 최단 거리:");
    for (i, &d) in distances.iter().enumerate() {
        println!("  → {} : {}km", city_names[i], d);
    }

    println!("\n서울 → 부산 최단 경로:");
    if let Some((total, path)) = g.dijkstra_path(0, 5) {
        let path_names: Vec<&str> = path.iter().map(|&i| city_names[i]).collect();
        println!("  경로: {}", path_names.join(" → "));
        println!("  총 거리: {}km", total);
    }
}