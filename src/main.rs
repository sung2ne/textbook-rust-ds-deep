fn main() {
    let data = vec![3i64, 1, 4, 1, 5, 9, 2, 6];

    // --- 세그먼트 트리 ---
    let mut seg = SegmentTree::new(&data);

    println!("세그먼트 트리");
    println!("구간 합 [2..5]: {}", seg.query(2, 5));   // 4+1+5+9 = 19
    println!("구간 합 [0..7]: {}", seg.query(0, 7));   // 31

    seg.update(3, 10);  // data[3] = 1 → 10
    println!("업데이트 후 [2..5]: {}", seg.query(2, 5)); // 4+10+5+9 = 28

    // --- 펜윅 트리 ---
    let mut fenwick = FenwickTree::from_slice(&data);

    println!("\n펜윅 트리");
    println!("구간 합 [3..6] (1-idx): {}", fenwick.range_sum(3, 6)); // 4+1+5+9 = 19
    println!("누적 합 [1..4]: {}", fenwick.prefix_sum(4));            // 3+1+4+1 = 9

    fenwick.update(4, 9);  // data[3] += 9 → 1+9=10
    println!("업데이트 후 [3..6]: {}", fenwick.range_sum(3, 6));      // 4+10+5+9 = 28
}