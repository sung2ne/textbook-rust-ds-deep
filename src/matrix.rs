pub fn matmul_naive(a: &[f64], b: &[f64], c: &mut [f64], n: usize) {
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                c[i * n + j] += a[i * n + k] * b[k * n + j];
            }
        }
    }
}

// 캐시 오블리비어스 재귀 행렬 곱셈
pub fn matmul_recursive(
    a: &[f64], b: &[f64], c: &mut [f64],
    n: usize, threshold: usize,
) {
    if n <= threshold {
        // 기저 사례: 작은 행렬은 직접 계산
        for i in 0..n {
            for j in 0..n {
                for k in 0..n {
                    c[i * n + j] += a[i * n + k] * b[k * n + j];
                }
            }
        }
        return;
    }

    let half = n / 2;
    // 실제 구현에서는 부분 행렬을 별도 메모리로 추출해 재귀 호출
    // 여기서는 개념만 표현
    let _ = half;
    // 생략: C11 += A11*B11 + A12*B21, 나머지 블록...
}