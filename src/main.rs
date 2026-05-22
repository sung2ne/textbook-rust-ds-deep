use box_list::LinkedList;

fn main() {
    let mut list = LinkedList::new();
    list.push_front(3);
    list.push_front(2);
    list.push_front(1);

    println!("길이: {}", list.len()); // 3

    for val in list.iter() {
        print!("{} ", val); // 1 2 3
    }
    println!();

    println!("peek: {:?}", list.peek_front()); // Some(1)
    println!("pop: {:?}", list.pop_front());   // Some(1)
    println!("pop: {:?}", list.pop_front());   // Some(2)

    // into_iter로 소비
    list.push_front(10);
    list.push_front(20);
    for val in list {
        print!("{} ", val); // 20 10
    }
    println!();
}