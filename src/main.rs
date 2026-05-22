fn first_and_last(v: &Vec<i32>) -> Option<(i32, i32)> {
    let first = v.first()?;   // &i32
    let last = v.last()?;     // &i32
    Some((*first, *last))
}

fn main() {
    let v = vec![1, 2, 3, 4, 5];
    if let Some((f, l)) = first_and_last(&v) {
        println!("first: {}, last: {}", f, l);
    }
}