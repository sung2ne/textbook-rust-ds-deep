use doubly_linked::DoublyLinkedList;

fn main() {
    let mut list: DoublyLinkedList<i32> = DoublyLinkedList::new();

    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    list.push_front(0);

    println!("앞→뒤: {:?}", list.to_vec_forward());  // [0, 1, 2, 3]
    println!("뒤→앞: {:?}", list.to_vec_backward()); // [3, 2, 1, 0]

    println!("pop_back: {:?}", list.pop_back());   // Some(3)
    println!("pop_front: {:?}", list.pop_front()); // Some(0)

    println!("길이: {}", list.len()); // 2
    println!("앞→뒤: {:?}", list.to_vec_forward()); // [1, 2]
}