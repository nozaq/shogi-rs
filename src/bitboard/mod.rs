use std::fmt;
use std::iter;
use std::ops;
use bitintr::x86::bmi2::pext;

use {Color, PieceType, Square};

/// Represents a board state in which each square takes two possible values, filled or empty.
///
/// `Bitboard` implements [PEXT Bitboard](http://chessprogramming.wikispaces.com/BMI2#PEXTBitboards) which relies on [BMI2 instruction set](http://chessprogramming.wikispaces.com/BMI2).
/// For environments which do not support BMI2, it will use software fallback methods. Thanks to [bitintr](https://github.com/gnzlbg/bitintr) crate.
///
/// # Examples
///
/// ```
/// use shogi::Bitboard;
/// use shogi::square::consts::*;
///
/// let mut bb = Bitboard::empty();
/// bb ^= SQ_1A;
/// bb |= SQ_9I;
///
/// assert_eq!(2, bb.count());
/// assert_eq!(1, bb.filter(|sq| sq.file() == 0).count());
/// ```
#[derive(Debug, Default, Clone, Copy)]
pub struct Bitboard {
    p: [u64; 2],
}

/////////////////////////////////////////////////////////////////////////////
// Type implementation
/////////////////////////////////////////////////////////////////////////////

impl Bitboard {
    /// Returns an empty instance of `Bitboard`.
    #[inline(always)]
    pub fn empty() -> Bitboard {
        Bitboard { p: [0, 0] }
    }

    /// Checks if any of its squares is filled.
    #[inline(always)]
    pub fn is_any(&self) -> bool {
        (self.p[0] | self.p[1]) != 0
    }

    /// Checks if all of its squares are empty.
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        (self.p[0] | self.p[1]) == 0
    }

    /// Sets the given square as empty.
    #[inline(always)]
    pub fn clear_at(&mut self, sq: Square) {
        *self &= &!&square_bb(sq)
    }

    /// Returns the number of squares filled.
    #[inline(always)]
    pub fn count(&self) -> u32 {
        self.p[0].count_ones() + self.p[1].count_ones()
    }

    /// Sets the first filled square as empty and returns that square.
    ///
    /// This method expects the bitboard not being empty.
    #[inline(always)]
    pub fn pop(&mut self) -> Square {
        if self.p[0] != 0 {
            let sq = Square::from_index(self.p[0].trailing_zeros() as u8).unwrap();
            self.p[0] &= self.p[0] - 1;
            sq
        } else {
            let sq = Square::from_index(self.p[1].trailing_zeros() as u8 + 63).unwrap();
            self.p[1] &= self.p[1] - 1;
            sq
        }
    }

    #[inline(always)]
    fn merge(&self) -> u64 {
        self.p[0] | self.p[1]
    }
}

/////////////////////////////////////////////////////////////////////////////
// Operator implementations
/////////////////////////////////////////////////////////////////////////////

impl<'a> ops::Not for &'a Bitboard {
    type Output = Bitboard;

    #[inline(always)]
    fn not(self) -> Bitboard {
        Bitboard { p: [!self.p[0], !self.p[1]] }
    }
}

impl<'a, 'b> ops::BitAnd<&'a Bitboard> for &'b Bitboard {
    type Output = Bitboard;

    #[inline(always)]
    fn bitand(self, rhs: &'a Bitboard) -> Bitboard {
        Bitboard { p: [self.p[0] & rhs.p[0], self.p[1] & rhs.p[1]] }
    }
}

impl<'a> ops::BitAndAssign<&'a Bitboard> for Bitboard {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: &'a Bitboard) {
        self.p[0] &= rhs.p[0];
        self.p[1] &= rhs.p[1];
    }
}

impl<'a, 'b> ops::BitOr<&'a Bitboard> for &'b Bitboard {
    type Output = Bitboard;

    #[inline(always)]
    fn bitor(self, rhs: &'a Bitboard) -> Bitboard {
        Bitboard { p: [self.p[0] | rhs.p[0], self.p[1] | rhs.p[1]] }
    }
}

impl<'a> ops::BitOrAssign<&'a Bitboard> for Bitboard {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: &'a Bitboard) {
        self.p[0] |= rhs.p[0];
        self.p[1] |= rhs.p[1];
    }
}

impl<'a, 'b> ops::BitXor<&'a Bitboard> for &'b Bitboard {
    type Output = Bitboard;

    #[inline(always)]
    fn bitxor(self, rhs: &'a Bitboard) -> Bitboard {
        Bitboard { p: [self.p[0] ^ rhs.p[0], self.p[1] ^ rhs.p[1]] }
    }
}

impl<'a> ops::BitXorAssign<&'a Bitboard> for Bitboard {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: &'a Bitboard) {
        self.p[0] ^= rhs.p[0];
        self.p[1] ^= rhs.p[1];
    }
}

impl<'a> ops::BitAnd<Square> for &'a Bitboard {
    type Output = Bitboard;

    #[inline(always)]
    fn bitand(self, rhs: Square) -> Bitboard {
        self & &square_bb(rhs)
    }
}

impl<'a> ops::BitAndAssign<Square> for Bitboard {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: Square) {
        *self &= &square_bb(rhs)
    }
}

impl<'a> ops::BitOr<Square> for &'a Bitboard {
    type Output = Bitboard;

    #[inline(always)]
    fn bitor(self, rhs: Square) -> Bitboard {
        self | &square_bb(rhs)
    }
}

impl<'a> ops::BitOrAssign<Square> for Bitboard {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: Square) {
        *self |= &square_bb(rhs)
    }
}

impl<'a> ops::BitXor<Square> for &'a Bitboard {
    type Output = Bitboard;

    #[inline(always)]
    fn bitxor(self, rhs: Square) -> Bitboard {
        self ^ &square_bb(rhs)
    }
}

impl<'a> ops::BitXorAssign<Square> for Bitboard {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: Square) {
        *self ^= &square_bb(rhs)
    }
}

/////////////////////////////////////////////////////////////////////////////
// Trait implementation
/////////////////////////////////////////////////////////////////////////////

impl fmt::Display for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(writeln!(f, "   9   8   7   6   5   4   3   2   1"));
        try!(writeln!(f, "+---+---+---+---+---+---+---+---+---+"));

        for rank in 0..9 {
            try!(write!(f, "|"));
            for file in (0..9).rev() {
                let sq = Square::new(file, rank).unwrap();
                try!(write!(f, " {} |", if (self & sq).is_empty() { " " } else { "X" }));
            }
            try!(writeln!(f, " {}", ('a' as u8 + rank) as char));
            try!(writeln!(f, "+---+---+---+---+---+---+---+---+---+"));
        }

        Ok(())
    }
}

impl iter::Iterator for Bitboard {
    type Item = Square;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.is_any() {
            Some(self.pop())
        } else {
            None
        }
    }
}

/////////////////////////////////////////////////////////////////////////////
// Constants
/////////////////////////////////////////////////////////////////////////////

const SQUARE_BB: [Bitboard; 81] = [Bitboard { p: [1 << 0, 0] },
                                   Bitboard { p: [1 << 1, 0] },
                                   Bitboard { p: [1 << 2, 0] },
                                   Bitboard { p: [1 << 3, 0] },
                                   Bitboard { p: [1 << 4, 0] },
                                   Bitboard { p: [1 << 5, 0] },
                                   Bitboard { p: [1 << 6, 0] },
                                   Bitboard { p: [1 << 7, 0] },
                                   Bitboard { p: [1 << 8, 0] },
                                   Bitboard { p: [1 << 9, 0] },
                                   Bitboard { p: [1 << 10, 0] },
                                   Bitboard { p: [1 << 11, 0] },
                                   Bitboard { p: [1 << 12, 0] },
                                   Bitboard { p: [1 << 13, 0] },
                                   Bitboard { p: [1 << 14, 0] },
                                   Bitboard { p: [1 << 15, 0] },
                                   Bitboard { p: [1 << 16, 0] },
                                   Bitboard { p: [1 << 17, 0] },
                                   Bitboard { p: [1 << 18, 0] },
                                   Bitboard { p: [1 << 19, 0] },
                                   Bitboard { p: [1 << 20, 0] },
                                   Bitboard { p: [1 << 21, 0] },
                                   Bitboard { p: [1 << 22, 0] },
                                   Bitboard { p: [1 << 23, 0] },
                                   Bitboard { p: [1 << 24, 0] },
                                   Bitboard { p: [1 << 25, 0] },
                                   Bitboard { p: [1 << 26, 0] },
                                   Bitboard { p: [1 << 27, 0] },
                                   Bitboard { p: [1 << 28, 0] },
                                   Bitboard { p: [1 << 29, 0] },
                                   Bitboard { p: [1 << 30, 0] },
                                   Bitboard { p: [1 << 31, 0] },
                                   Bitboard { p: [1 << 32, 0] },
                                   Bitboard { p: [1 << 33, 0] },
                                   Bitboard { p: [1 << 34, 0] },
                                   Bitboard { p: [1 << 35, 0] },
                                   Bitboard { p: [1 << 36, 0] },
                                   Bitboard { p: [1 << 37, 0] },
                                   Bitboard { p: [1 << 38, 0] },
                                   Bitboard { p: [1 << 39, 0] },
                                   Bitboard { p: [1 << 40, 0] },
                                   Bitboard { p: [1 << 41, 0] },
                                   Bitboard { p: [1 << 42, 0] },
                                   Bitboard { p: [1 << 43, 0] },
                                   Bitboard { p: [1 << 44, 0] },
                                   Bitboard { p: [1 << 45, 0] },
                                   Bitboard { p: [1 << 46, 0] },
                                   Bitboard { p: [1 << 47, 0] },
                                   Bitboard { p: [1 << 48, 0] },
                                   Bitboard { p: [1 << 49, 0] },
                                   Bitboard { p: [1 << 50, 0] },
                                   Bitboard { p: [1 << 51, 0] },
                                   Bitboard { p: [1 << 52, 0] },
                                   Bitboard { p: [1 << 53, 0] },
                                   Bitboard { p: [1 << 54, 0] },
                                   Bitboard { p: [1 << 55, 0] },
                                   Bitboard { p: [1 << 56, 0] },
                                   Bitboard { p: [1 << 57, 0] },
                                   Bitboard { p: [1 << 58, 0] },
                                   Bitboard { p: [1 << 59, 0] },
                                   Bitboard { p: [1 << 60, 0] },
                                   Bitboard { p: [1 << 61, 0] },
                                   Bitboard { p: [1 << 62, 0] },
                                   Bitboard { p: [0, 1 << 0] },
                                   Bitboard { p: [0, 1 << 1] },
                                   Bitboard { p: [0, 1 << 2] },
                                   Bitboard { p: [0, 1 << 3] },
                                   Bitboard { p: [0, 1 << 4] },
                                   Bitboard { p: [0, 1 << 5] },
                                   Bitboard { p: [0, 1 << 6] },
                                   Bitboard { p: [0, 1 << 7] },
                                   Bitboard { p: [0, 1 << 8] },
                                   Bitboard { p: [0, 1 << 9] },
                                   Bitboard { p: [0, 1 << 10] },
                                   Bitboard { p: [0, 1 << 11] },
                                   Bitboard { p: [0, 1 << 12] },
                                   Bitboard { p: [0, 1 << 13] },
                                   Bitboard { p: [0, 1 << 14] },
                                   Bitboard { p: [0, 1 << 15] },
                                   Bitboard { p: [0, 1 << 16] },
                                   Bitboard { p: [0, 1 << 17] }];

#[inline(always)]
fn square_bb(sq: Square) -> Bitboard {
    SQUARE_BB[sq.index()]
}

mod factory;

pub use self::factory::Factory;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
