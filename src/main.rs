use std::iter::Iterator;

// Box<dyn Iterator>는 동적 디스패치
fn sum_boxed(iter: Box<dyn Iterator<Item = i32>>) -> i32 {
    iter.sum()
}

fn main() {
    let data = vec![1i32, 2, 3, 4, 5];
    let sum = sum_boxed(Box::new(data.into_iter()));
    println!("sum: {}", sum);
}