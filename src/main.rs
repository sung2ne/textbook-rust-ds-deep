mod bloom;
use bloom::BloomFilter;

fn main() {
    // 1만 원소, 오탐률 1% 목표
    let mut bf = BloomFilter::new(10_000, 0.01);

    // 1~5000 삽입
    for i in 0u64..5000 {
        bf.insert(&i);
    }

    // 삽입한 값은 반드시 true
    for i in 0u64..5000 {
        assert!(bf.may_contain(&i), "false negative 발생! (있어야 함)");
    }

    // 5000~10000은 삽입 안 했으므로 대부분 false
    let false_pos: usize = (5000u64..10000)
        .filter(|i| bf.may_contain(i))
        .count();

    let fpr = false_pos as f64 / 5000.0;
    println!("실제 오탐률: {:.2}%", fpr * 100.0);
    println!("이론 오탐률: {:.2}%", bf.estimated_fpr(5000) * 100.0);
    println!("false negative: 0건 보장됨");
}