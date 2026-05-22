impl<T: Clone> DoublyLinkedList<T> {
    /// 앞 요소를 복제해 반환한다.
    /// RefCell 대여 수명을 외부로 노출하기 어려우므로 Clone을 활용.
    pub fn peek_front(&self) -> Option<T> {
        self.head
            .as_ref()
            .map(|node| node.borrow().data.clone())
    }

    pub fn peek_back(&self) -> Option<T> {
        self.tail
            .as_ref()
            .map(|node| node.borrow().data.clone())
    }
}

impl<T: Clone> DoublyLinkedList<T> {
    /// 앞에서 뒤로 순회하며 Vec으로 수집한다.
    pub fn to_vec_forward(&self) -> Vec<T> {
        let mut result = Vec::with_capacity(self.len);
        let mut curr = self.head.clone();
        while let Some(node) = curr {
            result.push(node.borrow().data.clone());
            curr = node.borrow().next.clone();
        }
        result
    }

    /// 뒤에서 앞으로 순회.
    pub fn to_vec_backward(&self) -> Vec<T> {
        let mut result = Vec::with_capacity(self.len);
        let mut curr = self.tail.clone();
        while let Some(node) = curr {
            result.push(node.borrow().data.clone());
            curr = node
                .borrow()
                .prev
                .as_ref()
                .and_then(|w| w.upgrade());
        }
        result
    }
}