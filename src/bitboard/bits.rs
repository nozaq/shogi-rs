pub trait Pext {
    fn pext(&self, mask: Self) -> Self;
}

impl Pext for u64 {
    #[inline(always)]
    #[cfg(target_feature = "bmi2")]
    fn pext(&self, mask: Self) -> Self {
        unsafe { std::arch::x86_64::_pext_u64(*self, mask) }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "bmi2"))]
    fn pext(&self, mask: Self) -> Self {
        let mut result = 0;
        let mut bb = 1;
        let mut mask = mask;
        let value = *self;

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
}
