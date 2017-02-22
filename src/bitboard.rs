use std::fmt;
use std::ops;
use bitintr::x86::bmi2::pext;

use Square;

#[derive(Debug, Default, Clone, Copy)]
pub struct Bitboard {
    p: [u64; 2],
}

/////////////////////////////////////////////////////////////////////////////
// Type implementation
/////////////////////////////////////////////////////////////////////////////

impl Bitboard {
    pub fn new(p1: u64, p2: u64) -> Bitboard {
        Bitboard { p: [p1, p2] }
    }

    pub fn init() {}
}

/////////////////////////////////////////////////////////////////////////////
// Operator implementations
/////////////////////////////////////////////////////////////////////////////

impl<'a> ops::Not for &'a Bitboard {
    type Output = Bitboard;

    fn not(self) -> Bitboard {
        Bitboard { p: [!self.p[0], !self.p[1]] }
    }
}

impl<'a, 'b> ops::BitAnd<&'a Bitboard> for &'b Bitboard {
    type Output = Bitboard;

    fn bitand(self, rhs: &'a Bitboard) -> Bitboard {
        Bitboard { p: [self.p[0] & rhs.p[0], self.p[1] & rhs.p[1]] }
    }
}

impl<'a> ops::BitAndAssign<&'a Bitboard> for Bitboard {
    fn bitand_assign(&mut self, rhs: &'a Bitboard) {
        self.p[0] &= rhs.p[0];
        self.p[1] &= rhs.p[1];
    }
}

impl<'a, 'b> ops::BitOr<&'a Bitboard> for &'b Bitboard {
    type Output = Bitboard;

    fn bitor(self, rhs: &'a Bitboard) -> Bitboard {
        Bitboard { p: [self.p[0] | rhs.p[0], self.p[1] | rhs.p[1]] }
    }
}

impl<'a> ops::BitOrAssign<&'a Bitboard> for Bitboard {
    fn bitor_assign(&mut self, rhs: &'a Bitboard) {
        self.p[0] |= rhs.p[0];
        self.p[1] |= rhs.p[1];
    }
}

impl<'a, 'b> ops::BitXor<&'a Bitboard> for &'b Bitboard {
    type Output = Bitboard;

    fn bitxor(self, rhs: &'a Bitboard) -> Bitboard {
        Bitboard { p: [self.p[0] ^ rhs.p[0], self.p[1] ^ rhs.p[1]] }
    }
}

impl<'a> ops::BitXorAssign<&'a Bitboard> for Bitboard {
    fn bitxor_assign(&mut self, rhs: &'a Bitboard) {
        self.p[0] ^= rhs.p[0];
        self.p[1] ^= rhs.p[1];
    }
}

impl<'a> ops::BitAnd<Square> for &'a Bitboard {
    type Output = Bitboard;

    fn bitand(self, rhs: Square) -> Bitboard {
        self & &SQUARE_BB[rhs.index()]
    }
}

impl<'a> ops::BitOr<Square> for &'a Bitboard {
    type Output = Bitboard;

    fn bitor(self, rhs: Square) -> Bitboard {
        self | &SQUARE_BB[rhs.index()]
    }
}

impl<'a> ops::BitXor<Square> for &'a Bitboard {
    type Output = Bitboard;

    fn bitxor(self, rhs: Square) -> Bitboard {
        self ^ &SQUARE_BB[rhs.index()]
    }
}

/////////////////////////////////////////////////////////////////////////////
// Trait implementation
/////////////////////////////////////////////////////////////////////////////

impl<'a> From<&'a Bitboard> for bool {
    fn from(bb: &'a Bitboard) -> bool {
        (bb.p[0] | bb.p[1]) != 0
    }
}

impl fmt::Display for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(writeln!(f, "   9   8   7   6   5   4   3   2   1"));
        try!(writeln!(f, "+---+---+---+---+---+---+---+---+---+"));

        for rank in 0..9 {
            try!(write!(f, "|"));
            for file in 0..9 {
                let sq = Square::new(file, rank).unwrap();
                try!(write!(f, " {} |", if bool::from(&(self & sq)) { "X" } else { " " }));
            }
            try!(writeln!(f, " {}", ('a' as u8 + rank) as char));
            try!(writeln!(f, "+---+---+---+---+---+---+---+---+---+"));
        }

        Ok(())
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

const FILE_BB: [Bitboard; 9] = [Bitboard { p: [0x1FF << (9 * 0), 0] },
                                Bitboard { p: [0x1FF << (9 * 1), 0] },
                                Bitboard { p: [0x1FF << (9 * 2), 0] },
                                Bitboard { p: [0x1FF << (9 * 3), 0] },
                                Bitboard { p: [0x1FF << (9 * 4), 0] },
                                Bitboard { p: [0x1FF << (9 * 5), 0] },
                                Bitboard { p: [0x1FF << (9 * 6), 0] },
                                Bitboard { p: [0, 0x1FF << (9 * 0)] },
                                Bitboard { p: [0, 0x1FF << (9 * 1)] }];

const RANK_BB: [Bitboard; 9] = [Bitboard { p: [0x40201008040201 << 0, 0x201 << 0] },
                                Bitboard { p: [0x40201008040201 << 1, 0x201 << 1] },
                                Bitboard { p: [0x40201008040201 << 2, 0x201 << 2] },
                                Bitboard { p: [0x40201008040201 << 3, 0x201 << 3] },
                                Bitboard { p: [0x40201008040201 << 4, 0x201 << 4] },
                                Bitboard { p: [0x40201008040201 << 5, 0x201 << 5] },
                                Bitboard { p: [0x40201008040201 << 6, 0x201 << 6] },
                                Bitboard { p: [0x40201008040201 << 7, 0x201 << 7] },
                                Bitboard { p: [0x40201008040201 << 8, 0x201 << 8] }];

static ROOK_BLOCK_MASK: [Bitboard; 81] = [Bitboard { p: [0, 0] }; 81];

static BETWEEN_BB: [[Bitboard; 81]; 81] = [[Bitboard { p: [0, 0] }; 81]; 81];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn square_bbs() {
        for i in 0..9 {
            println!("{}", FILE_BB[i]);
            println!("{}", RANK_BB[i]);
        }
    }
}