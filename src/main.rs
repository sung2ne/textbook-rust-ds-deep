fn main() {
    // 1. 리터럴에서
    let s1 = String::from("hello");
    let s2 = "hello".to_string();
    let s3 = "hello".to_owned();

    // 2. 포맷으로
    let name = "Rust";
    let s4 = format!("Hello, {}!", name);

    // 3. 이터레이터에서 collect
    let chars = vec!['R', 'u', 's', 't'];
    let s5: String = chars.iter().collect();
    println!("{}", s5);  // Rust

    // 4. 문자열 이어붙이기 (+)
    let hello = String::from("Hello, ");
    let world = String::from("world!");
    let hw = hello + &world;  // hello의 소유권이 이동됨!
    println!("{}", hw);

    // 5. push_str / push
    let mut s = String::new();
    s.push_str("Hello");
    s.push(',');
    s.push_str(" world!");
    println!("{}", s);
}