// async traits의 dyn 사용 제약 설명

use std::future::Future;

trait AsyncProcessor {
    async fn process(&self, input: i32) -> i32;
}

struct DoubleProcessor;

impl AsyncProcessor for DoubleProcessor {
    async fn process(&self, input: i32) -> i32 {
        input * 2
    }
}

// dyn AsyncProcessor는 아직 직접 사용이 복잡함
// 대안 1: trait_variant 크레이트 (공식 지원 중)
// 대안 2: 수동으로 반환 타입 박싱

// 수동 박싱 방식
trait AsyncProcessorDyn {
    fn process<'a>(&'a self, input: i32) -> std::pin::Pin<Box<dyn Future<Output = i32> + 'a>>;
}

impl AsyncProcessorDyn for DoubleProcessor {
    fn process<'a>(&'a self, input: i32) -> std::pin::Pin<Box<dyn Future<Output = i32> + 'a>> {
        Box::pin(async move { input * 2 })
    }
}

fn main() {
    println!("async traits: Rust 1.75에서 기본 사용 안정화");
    println!("dyn async trait: 추가 작업 필요 (trait_variant 크레이트 권장)");
}