#[allow(dead_code)]
pub(crate) fn ceil_pow2(n: u32) -> u32 {
    32 - n.saturating_sub(1).leading_zeros()
}
