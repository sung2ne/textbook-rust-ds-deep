use im::Vector;

fn main() {
    let v1: Vector<i32> = Vector::new();
    let v2 = v1.clone() + vector![1, 2, 3]; // vector! 매크로
    let v3 = v2.clone().push_back(4);

    // v1, v2, v3은 독립적인 버전
    assert_eq!(v1.len(), 0);
    assert_eq!(v2.len(), 3);
    assert_eq!(v3.len(), 4);

    // v2는 변경되지 않음
    assert_eq!(v2[0], 1);
    println!("Vector v2: {:?}", v2);
    println!("Vector v3: {:?}", v3);
}