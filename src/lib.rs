impl Trie {
    pub fn delete(&mut self, word: &str) {
        Self::delete_node(&mut self.root, word.chars().collect::<Vec<_>>().as_slice());
    }

    // 반환값: 이 노드를 부모에서 제거해도 되면 true
    fn delete_node(node: &mut TrieNode, chars: &[char]) -> bool {
        if chars.is_empty() {
            // 단어 끝에 도달
            node.is_end = false;
            return node.children.is_empty(); // 자식 없으면 이 노드 제거 가능
        }

        let ch = chars[0];
        if let Some(child) = node.children.get_mut(&ch) {
            let should_remove = Self::delete_node(child, &chars[1..]);
            if should_remove {
                node.children.remove(&ch);
            }
        }

        // 이 노드를 제거해도 되는 조건: is_end도 아니고 자식도 없을 때
        !node.is_end && node.children.is_empty()
    }
}