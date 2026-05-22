use arena_list::ArenaList;

fn main() {
    let mut list: ArenaList<i32> = ArenaList::new();

    list.push_front(3);
    list.push_front(2);
    list.push_front(1);

    println!("리스트: {:?}", list.iter().collect::<Vec<_>>());
    // 출력: [1, 2, 3]

    list.remove(&2);
    println!("2 제거 후: {:?}", list.iter().collect::<Vec<_>>());
    // 출력: [1, 3]

    while let Some(val) = list.pop_front() {
        println!("pop: {}", val);
    }
    println!("비었나요? {}", list.is_empty());
}