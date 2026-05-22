// heapless는 no_std 임베디드 환경뿐 아니라 일반 std 환경에서도 동작합니다.
// 여기서는 std 환경으로 동작을 검증합니다. 실제 임베디드 배포 시
// #![no_std] #![no_main]으로 교체하고 패닉 핸들러를 추가합니다.
use heapless::{String, Vec, FnvIndexMap};

fn process_sensor_data() {
    // 스택에 최대 16개 원소를 담는 Vec
    let mut readings: Vec<u16, 16> = Vec::new();

    // 데이터 추가 — 실패 시 Err 반환 (패닉 없음)
    for i in 0..10u16 {
        readings.push(i * 100).ok(); // 결과 무시
    }

    // 평균 계산
    let sum: u32 = readings.iter().map(|&x| x as u32).sum();
    let avg = sum / readings.len() as u32;
    let _ = avg; // 실제 코드에서는 LED나 UART로 출력
}

fn process_config() {
    // 스택에 최대 8개 키-값 쌍을 담는 HashMap (FNV 해시)
    let mut config: FnvIndexMap<&str, u32, 8> = FnvIndexMap::new();

    config.insert("baud_rate", 115200).ok();
    config.insert("timeout_ms", 5000).ok();
    config.insert("retry_count", 3).ok();

    if let Some(&baud) = config.get("baud_rate") {
        let _ = baud;
    }
}

fn process_message() {
    // 스택에 최대 64바이트 문자열
    let mut msg: String<64> = String::new();
    msg.push_str("OK:").ok();
    msg.push_str("sensor=").ok();
    // 숫자를 문자열로 포매팅은 write! 매크로 사용 필요
}

fn main() {
    process_sensor_data();
    process_config();
    process_message();
}