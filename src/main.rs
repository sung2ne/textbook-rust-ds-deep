mod kv_store;
use kv_store::KvStore;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let dir = Path::new("/tmp/rust_kv_test");

    // 저장소 생성 및 데이터 삽입
    {
        let mut store = KvStore::open(dir)?;
        store.put("user:1", "Alice")?;
        store.put("user:2", "Bob")?;
        store.put("user:3", "Charlie")?;
        store.put("score:1", "100")?;
        store.put("score:2", "200")?;
        store.delete("user:2")?;
        println!("저장 완료. 원소 수: {}", store.len());
    } // store 드롭 (파일 핸들 닫힘)

    // 재시작 — WAL에서 복구
    {
        let store = KvStore::open(dir)?;
        println!("복구 후 원소 수: {}", store.len());
        assert_eq!(store.get("user:1"), Some(&"Alice".to_string()));
        assert_eq!(store.get("user:2"), None);  // 삭제됨
        assert_eq!(store.get("user:3"), Some(&"Charlie".to_string()));
        assert_eq!(store.get("score:1"), Some(&"100".to_string()));

        // 범위 조회: "score:" 접두사
        let scores = store.range("score:", "score:~");
        println!("score 범위 조회:");
        for (k, v) in &scores {
            println!("  {} = {}", k, v);
        }
    }

    // WAL 압축 테스트
    {
        let mut store = KvStore::open(dir)?;
        for i in 0..100 {
            store.put(format!("key:{:04}", i), format!("val:{}", i))?;
        }
        store.compact()?;
        println!("압축 완료");
    }

    {
        let store = KvStore::open(dir)?;
        println!("압축 후 복구: {} 원소", store.len());
        assert_eq!(store.get("key:0042"), Some(&"val:42".to_string()));
    }

    // 테스트 디렉토리 정리
    std::fs::remove_dir_all(dir)?;
    println!("모든 검증 통과");
    Ok(())
}