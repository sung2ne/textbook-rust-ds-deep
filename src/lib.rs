/// Box 기반 링크드리스트 스택 (이전 챕터 ch02 활용).
/// 이 예시는 별도 크레이트에서 가져온다고 가정한다.
/// 여기서는 동일 원리를 새로 인라인 구현.

type Link<T> = Option<Box<StackNode<T>>>;

struct StackNode<T> {
    data: T,
    next: Link<T>,
}

pub struct LinkedStack<T> {
    top: Link<T>,
    len: usize,
}

impl<T> LinkedStack<T> {
    pub fn new() -> Self {
        LinkedStack { top: None, len: 0 }
    }

    pub fn push(&mut self, val: T) {
        let old_top = self.top.take();
        self.top = Some(Box::new(StackNode {
            data: val,
            next: old_top,
        }));
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.top.take().map(|node| {
            self.top = node.next;
            self.len -= 1;
            node.data
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.top.as_ref().map(|node| &node.data)
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl<T> Drop for LinkedStack<T> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}