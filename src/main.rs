use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

/// 힙에 들어갈 상태. 거리가 작을수록 우선 처리 → Reverse 사용.
#[derive(Eq, PartialEq)]
struct State {
    cost: u64,
    node: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // cost가 작을수록 우선이므로 반전
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// 인접 리스트 그래프: graph[u] = [(v, weight), ...]
fn dijkstra(graph: &Vec<Vec<(usize, u64)>>, start: usize) -> Vec<u64> {
    let n = graph.len();
    let mut dist = vec![u64::MAX; n];
    dist[start] = 0;

    let mut heap = BinaryHeap::new();
    heap.push(State { cost: 0, node: start });

    while let Some(State { cost, node }) = heap.pop() {
        // 이미 더 짧은 경로를 알고 있으면 스킵
        if cost > dist[node] {
            continue;
        }
        for &(next, weight) in &graph[node] {
            let next_cost = cost + weight;
            if next_cost < dist[next] {
                dist[next] = next_cost;
                heap.push(State { cost: next_cost, node: next });
            }
        }
    }
    dist
}

fn main() {
    // 노드 5개, 간선 (u, v, weight) 형태
    // 0→1:4, 0→2:1, 2→1:2, 1→3:1, 2→3:5, 3→4:3
    let mut graph: Vec<Vec<(usize, u64)>> = vec![vec![]; 5];
    graph[0].push((1, 4));
    graph[0].push((2, 1));
    graph[2].push((1, 2));
    graph[1].push((3, 1));
    graph[2].push((3, 5));
    graph[3].push((4, 3));

    let dist = dijkstra(&graph, 0);
    println!("0번 노드에서의 최단 거리:");
    for (node, d) in dist.iter().enumerate() {
        println!("  → {}: {}", node, d);
    }
    // → 0: 0
    // → 1: 3  (0→2→1: 1+2)
    // → 2: 1  (0→2)
    // → 3: 4  (0→2→1→3: 1+2+1)
    // → 4: 7  (0→2→1→3→4: 1+2+1+3)
}