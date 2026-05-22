use std::pin::Pin;

fn main() {
    let mut s = String::from("hello");
    
    // String은 Unpin이므로 Pin해도 이동 가능
    let mut pinned = Pin::new(&mut s);
    
    // get_mut()은 Unpin인 경우에만 안전하게 사용 가능
    let s_ref: &mut String = Pin::get_mut(pinned.as_mut());
    s_ref.push_str(", world");
    
    println!("{}", s); // "hello, world"
}