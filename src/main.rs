fn make_greeting() -> String {
    String::from("안녕하세요") // 소유권을 반환
}

fn main() {
    let greeting = make_greeting();
    println!("{}", greeting);
}