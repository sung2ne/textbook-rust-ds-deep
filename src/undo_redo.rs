use im::HashMap;

pub struct Editor {
    history: Vec<HashMap<String, String>>,  // 이전 버전들
    future:  Vec<HashMap<String, String>>,  // redo 스택
    current: HashMap<String, String>,
}

impl Editor {
    pub fn new() -> Self {
        Editor {
            history: Vec::new(),
            future:  Vec::new(),
            current: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: impl Into<String>, val: impl Into<String>) {
        // 새 상태를 만들기 전에 현재 상태를 history에 저장
        self.history.push(self.current.clone()); // O(1) clone!
        self.future.clear(); // 새 변경이 생기면 redo 스택 초기화
        self.current = self.current.update(key.into(), val.into());
    }

    pub fn undo(&mut self) -> bool {
        if let Some(prev) = self.history.pop() {
            self.future.push(self.current.clone());
            self.current = prev;
            true
        } else {
            false
        }
    }

    pub fn redo(&mut self) -> bool {
        if let Some(next) = self.future.pop() {
            self.history.push(self.current.clone());
            self.current = next;
            true
        } else {
            false
        }
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.current.get(key)
    }
}