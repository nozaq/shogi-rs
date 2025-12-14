//! Implementation of the special bitwise operations

/// Trait providing the parallel bit extract (PEXT) operation.
/// The PEXT operation extracts bits from a value according to a given mask,
/// compressing them into the least significant bits of the result.
pub trait Pext {
    fn pext(&self, mask: Self) -> Self;
}

fn pext_naive(value: u64, mask: u64) -> u64 {
    let mut result = 0;
    let mut bb = 1;
    let mut mask = mask;

    while mask != 0 {
        let bit = mask & (!mask + 1);
        if value & bit != 0 {
            result |= bb;
        }
        mask &= mask - 1;
        bb <<= 1;
    }

    result
}

impl Pext for u64 {
    /// If the CPU supports the BMI2 instruction set, this method uses
    /// the `_pext_u64` intrinsic for efficient extraction.
    #[inline(always)]
    #[cfg(target_feature = "bmi2")]
    fn pext(&self, mask: Self) -> Self {
        unsafe { std::arch::x86_64::_pext_u64(*self, mask) }
    }

    /// Software fallback implementation of the PEXT operation.
    #[inline(always)]
    #[cfg(not(target_feature = "bmi2"))]
    fn pext(&self, mask: Self) -> Self {
        pext_naive(*self, mask)
    }
}
