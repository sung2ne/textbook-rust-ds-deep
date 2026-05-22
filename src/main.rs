// SwissTable 삭제 흐름 개념 시연
#[derive(Debug, Clone, PartialEq)]
enum CtrlByte {
    Empty,
    Deleted,
    Occupied(u8), // h2 값
}

impl CtrlByte {
    fn is_full(&self) -> bool {
        matches!(self, CtrlByte::Occupied(_))
    }
    fn is_empty(&self) -> bool {
        *self == CtrlByte::Empty
    }
}

fn can_stop_probe(ctrl: &CtrlByte) -> bool {
    // EMPTY를 만나면 탐사 중단 가능 (뒤에 찾는 키가 있을 수 없음)
    ctrl.is_empty()
}

fn main() {
    let chain = vec![
        CtrlByte::Occupied(0x42), // slot 0
        CtrlByte::Occupied(0x17), // slot 1
        CtrlByte::Deleted,        // slot 2 — 삭제된 슬롯, 탐사는 계속
        CtrlByte::Occupied(0x42), // slot 3 — 여기까지 탐사해야 h2=0x42 후보 발견
        CtrlByte::Empty,          // slot 4 — 여기서 탐사 종료
    ];

    let search_h2 = 0x42u8;
    println!("h2=0x{:02x} 탐사 시뮬레이션:", search_h2);
    for (i, ctrl) in chain.iter().enumerate() {
        print!("  slot[{}] {:?}", i, ctrl);
        if can_stop_probe(ctrl) {
            println!(" <- EMPTY, 탐사 종료");
            break;
        }
        if let CtrlByte::Occupied(h2) = ctrl {
            if *h2 == search_h2 {
                println!(" <- h2 일치! 실제 키 비교 필요");
            } else {
                println!(" <- h2 불일치, 계속");
            }
        } else {
            println!(" <- DELETED, 건너뜀 (탐사 계속)");
        }
    }
}