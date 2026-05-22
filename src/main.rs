mod btree;
use btree::BTree;

fn main() {
    let mut bt: BTree<i32, &str> = BTree::new(3); // t=3: 노드당 2~5개 키

    for (k, v) in [(10,"ten"),(20,"twenty"),(5,"five"),(6,"six"),
                   (12,"twelve"),(30,"thirty"),(7,"seven"),(17,"seventeen")] {
        bt.insert(k, v);
    }

    assert_eq!(bt.search(&6),  Some(&"six"));
    assert_eq!(bt.search(&15), None);
    assert_eq!(bt.search(&20), Some(&"twenty"));

    bt.remove(&6);
    assert_eq!(bt.search(&6), None);

    bt.remove(&20);
    assert_eq!(bt.search(&20), None);

    bt.insert(15, "fifteen");
    assert_eq!(bt.search(&15), Some(&"fifteen"));

    println!("B-Tree 검증 완료");
}