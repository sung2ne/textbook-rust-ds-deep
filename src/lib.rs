impl<T: Ord> MinHeap<T> {
    /// 기존 Vec에서 힙을 O(n)에 만든다.
    pub fn from_vec(data: Vec<T>) -> Self {
        let mut heap = MinHeap { data };
        if heap.data.len() <= 1 {
            return heap;
        }
        // 마지막 내부 노드(leaf가 아닌 노드)부터 루트까지 sift_down
        let last_internal = (heap.data.len() - 2) / 2;
        for i in (0..=last_internal).rev() {
            heap.sift_down(i);
        }
        heap
    }
}