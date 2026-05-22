use std::collections::BTreeMap;

/// 각 키마다 버전 이력을 유지
pub struct MvccStore {
    // key → [(txn_id, value)]  (txn_id 오름차순)
    data: BTreeMap<String, Vec<(u64, Option<String>)>>,
    next_txn: u64,
}

impl MvccStore {
    pub fn new() -> Self {
        MvccStore { data: BTreeMap::new(), next_txn: 1 }
    }

    /// 새 트랜잭션 ID 발급 (읽기 스냅샷 시점)
    pub fn begin_txn(&mut self) -> u64 {
        let id = self.next_txn;
        self.next_txn += 1;
        id
    }

    /// txn_id 시점의 값 읽기 (해당 txn_id 이하의 최신 버전)
    pub fn read(&self, key: &str, txn_id: u64) -> Option<&String> {
        self.data.get(key)?
            .iter()
            .rev()
            .find(|(tid, _)| *tid <= txn_id)
            .and_then(|(_, val)| val.as_ref())
    }

    /// 쓰기: 새 버전 추가
    pub fn write(&mut self, key: impl Into<String>, value: Option<String>, txn_id: u64) {
        self.data
            .entry(key.into())
            .or_default()
            .push((txn_id, value));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mvcc_isolation() {
        let mut store = MvccStore::new();

        // txn 1: alice 씀
        let t1 = store.begin_txn();
        store.write("alice", Some("100".into()), t1);

        // txn 2: 읽기 시작 (t1 이후)
        let t2 = store.begin_txn();

        // txn 3: alice 값 변경
        let t3 = store.begin_txn();
        store.write("alice", Some("200".into()), t3);

        // t2는 t3 변경 전 버전(100)을 읽어야 함
        assert_eq!(store.read("alice", t2), Some(&"100".to_string()));
        // t3 이후에 읽으면 새 버전(200)
        assert_eq!(store.read("alice", t3), Some(&"200".to_string()));
    }
}