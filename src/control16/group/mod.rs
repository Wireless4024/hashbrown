cfg_if! {
    // Use the SSE2 implementation if possible: it allows us to scan 16 buckets
    // at once instead of 8. We don't bother with AVX since it would require
    // runtime dispatch and wouldn't gain us much anyways: the probability of
    // finding a match drops off drastically after the first few buckets.
    //
    // I attempted an implementation on ARM using NEON instructions, but it
    // turns out that most NEON instructions have multi-cycle latency, which in
    // the end outweighs any gains over the generic implementation.
    if #[cfg(all(
        target_feature = "avx512f",
        target_feature = "avx512bw",
        feature = "avx512",
        any(target_arch = "x86", target_arch = "x86_64"),
        not(miri),
    ))] {
        mod avx512;
        use avx512 as imp;
    } else {
        mod generic;
        use generic as imp;
    }
}
pub(crate) use self::imp::Group;
pub(super) use self::imp::{
    BitMaskWord, NonZeroBitMaskWord, BITMASK_ITER_MASK, BITMASK_MASK, BITMASK_STRIDE,
};
