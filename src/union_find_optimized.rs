struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>, // Union by rank 최적화를 위한 트리 높이 상한
    size: Vec<usize>, // 각 집합의 원소 수
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            rank: vec![0; n],
            size: vec![1; n],
        }
    }

    /// 경로 압축을 적용한 find.
    /// x의 루트를 찾으면서 경로상의 모든 노드를 루트 직속으로 연결한다.
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]); // 재귀 후 부모를 루트로 갱신
        }
        self.parent[x]
    }

    /// Union by rank + 경로 압축.
    /// 랭크가 낮은 트리를 높은 트리 아래에 붙여 트리 높이 증가를 억제한다.
    fn union(&mut self, x: usize, y: usize) -> bool {
        let rx = self.find(x);
        let ry = self.find(y);

        if rx == ry {
            return false; // 이미 같은 집합
        }

        // 랭크가 큰 쪽을 루트로 선택한다.
        match self.rank[rx].cmp(&self.rank[ry]) {
            std::cmp::Ordering::Less => {
                self.parent[rx] = ry;
                self.size[ry] += self.size[rx];
            }
            std::cmp::Ordering::Greater => {
                self.parent[ry] = rx;
                self.size[rx] += self.size[ry];
            }
            std::cmp::Ordering::Equal => {
                self.parent[ry] = rx;
                self.size[rx] += self.size[ry];
                self.rank[rx] += 1; // 높이가 같으면 합친 쪽의 랭크가 1 증가
            }
        }

        true
    }

    fn same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    /// x가 속한 집합의 원소 수를 반환한다.
    fn set_size(&mut self, x: usize) -> usize {
        let rx = self.find(x);
        self.size[rx]
    }
}

fn main() {
    let mut uf = UnionFind::new(8);

    // 여러 union 연산 수행
    uf.union(0, 1);
    uf.union(2, 3);
    uf.union(4, 5);
    uf.union(6, 7);
    uf.union(0, 2); // {0,1,2,3} 합치기
    uf.union(4, 6); // {4,5,6,7} 합치기

    println!("0과 3 같은 집합: {}", uf.same(0, 3)); // true
    println!("0과 4 같은 집합: {}", uf.same(0, 4)); // false
    println!("{{0,1,2,3}} 크기: {}", uf.set_size(0)); // 4
    println!("{{4,5,6,7}} 크기: {}", uf.set_size(4)); // 4
}