impl FenwickTree {
    // 1부터 idx까지의 누적 합
    pub fn prefix_sum(&self, mut idx: usize) -> i64 {
        let mut sum = 0;
        while idx > 0 {
            sum += self.tree[idx];
            idx -= idx & idx.wrapping_neg();  // idx -= LSB(idx)
        }
        sum
    }

    // l부터 r까지의 구간 합 (1-indexed)
    pub fn range_sum(&self, l: usize, r: usize) -> i64 {
        self.prefix_sum(r) - if l > 1 { self.prefix_sum(l - 1) } else { 0 }
    }
}