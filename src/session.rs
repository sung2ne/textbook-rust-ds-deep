use std::collections::{BTreeMap, HashMap};

// 만료 시각 기준 정렬된 보조 인덱스
// key: 만료 예정 시각, value: session_id
// cleanup 시 BTreeMap의 앞쪽(오래된 항목)만 순회하면 됨
type ExpiryIndex = BTreeMap<Instant, String>;