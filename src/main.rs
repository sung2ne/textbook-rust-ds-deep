use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Version {
    major: u32,
    minor: u32,
    patch: u32,
}

impl Version {
    fn new(major: u32, minor: u32, patch: u32) -> Self {
        Version { major, minor, patch }
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

fn main() {
    let mut changelog: BTreeMap<Version, String> = BTreeMap::new();

    changelog.insert(Version::new(1, 0, 0), "최초 릴리스".to_string());
    changelog.insert(Version::new(1, 2, 0), "성능 개선".to_string());
    changelog.insert(Version::new(2, 0, 0), "API 대규모 변경".to_string());
    changelog.insert(Version::new(1, 1, 0), "버그 수정".to_string());

    println!("=== 버전 히스토리 (오름차순) ===");
    for (ver, note) in &changelog {
        println!("v{}: {}", ver, note);
    }

    // 1.x.x 버전만 조회
    let v1_start = Version::new(1, 0, 0);
    let v2_start = Version::new(2, 0, 0);
    println!("\n=== 1.x 버전 ===");
    for (ver, note) in changelog.range(v1_start..v2_start) {
        println!("v{}: {}", ver, note);
    }
}