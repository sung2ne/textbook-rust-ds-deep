/// 불변 순회를 위한 이터레이터.
pub struct ArenaListIter<'a, T> {
    list: &'a ArenaList<T>,
    curr: Option<usize>,
}

impl<T> ArenaList<T> {
    pub fn iter(&self) -> ArenaListIter<'_, T> {
        ArenaListIter {
            list: self,
            curr: self.head,
        }
    }
}

impl<'a, T> Iterator for ArenaListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.curr?;
        self.curr = self.list.nodes[idx].next;
        Some(&self.list.nodes[idx].data)
    }
}