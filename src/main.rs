use std::mem;

// 패딩이 생기는 구조체
struct Padded {
    a: u8,   // 1 byte + 7 bytes padding
    b: u64,  // 8 bytes
}

// 필드 순서를 바꿔 패딩을 없앤 구조체
struct Compact {
    b: u64,  // 8 bytes
    a: u8,   // 1 byte + 7 bytes padding (구조체 끝에만)
}

fn main() {
    println!("Padded:  {} bytes", mem::size_of::<Padded>());
    println!("Compact: {} bytes", mem::size_of::<Compact>());
}