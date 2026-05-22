impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        // pop_front를 반복해 순차적으로 해제 (재귀 없음)
        while self.pop_front().is_some() {}
    }
}