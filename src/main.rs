use std::future::Future;
use std::mem::size_of_val;

async fn small_future() -> i32 {
    42
}

async fn larger_future(x: i32, y: i32, z: i32) -> i32 {
    let a = x + y;
    some_point().await;
    a + z
}

async fn some_point() {}

fn main() {
    let f1 = small_future();
    let f2 = larger_future(1, 2, 3);

    println!("small_future 크기: {} bytes", size_of_val(&f1));
    println!("larger_future 크기: {} bytes", size_of_val(&f2));
}