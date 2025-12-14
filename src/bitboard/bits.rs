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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pext() {
        // Test cases: (value, mask, expected)
        let table: [(u64, u64, u64); _] = [
            // mask is zero
            (
                0x0000_0000_0000_0000,
                0x0000_0000_0000_0000,
                0x0000_0000_0000_0000,
            ),
            (
                0x1234_5678_9ABC_DEF0,
                0x0000_0000_0000_0000,
                0x0000_0000_0000_0000,
            ),
            (
                0xFFFF_FFFF_FFFF_FFFF,
                0x0000_0000_0000_0000,
                0x0000_0000_0000_0000,
            ),
            // mask is all ones
            (
                0x0000_0000_0000_0000,
                0xFFFF_FFFF_FFFF_FFFF,
                0x0000_0000_0000_0000,
            ),
            (
                0x1234_5678_9ABC_DEF0,
                0xFFFF_FFFF_FFFF_FFFF,
                0x1234_5678_9ABC_DEF0,
            ),
            (
                0xFFFF_FFFF_FFFF_FFFF,
                0xFFFF_FFFF_FFFF_FFFF,
                0xFFFF_FFFF_FFFF_FFFF,
            ),
            // one-hot
            (0b1010_1010, 0b0000_1000, 0b1),
            (0b1010_1010, 0b0000_0100, 0b0),
            (0b1010_1010, 0b0000_0010, 0b1),
            (0b1010_1010, 0b0000_0001, 0b0),
            // continuous ones
            (0b1111_0000, 0b1111_0000, 0b1111),
            (0b0000_1111, 0b0000_1111, 0b1111),
            (0b1010_1010, 0b1111_0000, 0b1010),
            (0b1010_1010, 0b0000_1111, 0b1010),
            // alternating ones
            (0b1111_0000, 0b1010_1010, 0b1100),
            (0b1111_0000, 0b0101_0101, 0b1100),
            (0b0000_1111, 0b1010_1010, 0b0011),
            (0b0000_1111, 0b0101_0101, 0b0011),
            // random values
            (
                0x243F_6A88_85A3_08D3,
                0x1319_8A2E_0370_7344,
                0x0000_0000_003B_4502,
            ),
            (
                0xA409_3822_299F_31D0,
                0x082E_FA98_EC4E_6C89,
                0x0000_0000_0870_33A4,
            ),
        ];

        for &(value, mask, expected) in &table {
            let result = pext_naive(value, mask);
            assert_eq!(expected, result, "value: {:b}, mask: {:b}", value, mask);
        }
    }
}
