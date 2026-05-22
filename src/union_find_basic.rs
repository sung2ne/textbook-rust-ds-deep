struct UnionFind {
    parent: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        // 처음에는 각 원소가 자기 자신의 대표
        UnionFind {
            parent: (0..n).collect(),
        }
    }

    /// 원소 x의 대표(루트)를 반환한다.
    fn find(&self, x: usize) -> usize {
        if self.parent[x] == x {
            x
        } else {
            self.find(self.parent[x])
        }
    }

    /// 원소 x와 y의 집합을 합친다.
    fn union(&mut self, x: usize, y: usize) {
        let rx = self.find(x);
        let ry = self.find(y);
        if rx != ry {
            self.parent[rx] = ry; // x의 루트를 y의 루트 아래로 붙인다
        }
    }

    /// 두 원소가 같은 집합에 속하는지 확인한다.
    fn same(&self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
}

fn main() {
    let mut uf = UnionFind::new(6); // 원소: 0, 1, 2, 3, 4, 5

    uf.union(0, 1);
    uf.union(1, 2);
    uf.union(3, 4);

    println!("0과 2: {}", uf.same(0, 2)); // true (같은 집합)
    println!("0과 3: {}", uf.same(0, 3)); // false (다른 집합)
    println!("3과 4: {}", uf.same(3, 4)); // true (같은 집합)
}