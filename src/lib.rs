impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        // 앞에서부터 순차적으로 해제 (재귀 없음)
        while self.pop_front().is_some() {}
    }
}