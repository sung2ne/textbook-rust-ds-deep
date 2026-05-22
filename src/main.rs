use std::collections::VecDeque;

fn main() {
    // Vec → VecDeque
    let vec = vec![1, 2, 3, 4, 5];
    let deque: VecDeque<i32> = VecDeque::from(vec);
    println!("{:?}", deque);

    // VecDeque → Vec
    let back: Vec<i32> = Vec::from(deque);
    println!("{:?}", back);
}