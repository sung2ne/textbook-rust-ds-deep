#[inline(never)]
pub fn must_not_inline(n: u64) -> u64 {
    (0..n).sum()
}