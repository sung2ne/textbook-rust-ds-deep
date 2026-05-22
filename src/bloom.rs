use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct BloomFilter {
    bits: Vec<u64>,   // m 비트를 u64 배열로 표현
    m: usize,         // 비트 배열 크기
    k: usize,         // 해시 함수 개수
}

impl BloomFilter {
    /// capacity: 예상 삽입 원소 수, fpr: 목표 오탐률 (예: 0.01 = 1%)
    pub fn new(capacity: usize, fpr: f64) -> Self {
        // 최적 m = -(n * ln p) / (ln 2)^2
        let ln2 = std::f64::consts::LN_2;
        let m = (-(capacity as f64) * fpr.ln() / (ln2 * ln2)).ceil() as usize;
        // 최적 k = (m/n) * ln2
        let k = ((m as f64 / capacity as f64) * ln2).round().max(1.0) as usize;
        let words = (m + 63) / 64;
        BloomFilter {
            bits: vec![0u64; words],
            m,
            k,
        }
    }

    /// 비트 인덱스 i를 (word 인덱스, bit 위치)로 분리
    fn bit_pos(i: usize) -> (usize, u64) {
        (i / 64, 1u64 << (i % 64))
    }

    fn set_bit(&mut self, i: usize) {
        let (word, mask) = Self::bit_pos(i);
        self.bits[word] |= mask;
    }

    fn get_bit(&self, i: usize) -> bool {
        let (word, mask) = Self::bit_pos(i);
        self.bits[word] & mask != 0
    }

    /// double hashing으로 k개 인덱스를 생성
    fn indices<T: Hash>(&self, val: &T) -> Vec<usize> {
        // h1: DefaultHasher (시드 0)
        let mut h1 = DefaultHasher::new();
        val.hash(&mut h1);
        let hash1 = h1.finish();

        // h2: 비트 회전으로 독립적인 두 번째 해시 근사
        let hash2 = hash1.rotate_left(32) ^ hash1.wrapping_mul(0x9e3779b97f4a7c15);

        (0..self.k)
            .map(|i| {
                let h = hash1.wrapping_add((i as u64).wrapping_mul(hash2));
                (h % self.m as u64) as usize
            })
            .collect()
    }

    pub fn insert<T: Hash>(&mut self, val: &T) {
        for idx in self.indices(val) {
            self.set_bit(idx);
        }
    }

    /// false: 확실히 없음 / true: 있을 수 있음 (오탐 가능)
    pub fn may_contain<T: Hash>(&self, val: &T) -> bool {
        self.indices(val).iter().all(|&idx| self.get_bit(idx))
    }

    /// 현재 추정 오탐률
    pub fn estimated_fpr(&self, n_inserted: usize) -> f64 {
        let k = self.k as f64;
        let m = self.m as f64;
        let n = n_inserted as f64;
        (1.0 - (-k * n / m).exp()).powf(k)
    }
}