use crate::Color;
use std::fmt;
use std::iter;

const ASCII_1: u8 = b'1';
const ASCII_9: u8 = b'9';
const ASCII_LOWER_A: u8 = b'a';
const ASCII_LOWER_I: u8 = b'i';

/// Represents a position of each cell in the game board.
///
/// # Examples
///
/// ```
/// use shogi::Square;
///
/// let sq = Square::new(4, 4).unwrap();
/// assert_eq!("5e", sq.to_string());
/// ```
///
/// `Square` can be created by parsing a SFEN formatted string as well.
///
/// ```
/// use shogi::Square;
///
/// let sq = Square::from_sfen("5e").unwrap();
/// assert_eq!(4, sq.file());
/// assert_eq!(4, sq.rank());
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Square {
    inner: u8,
}

impl Square {
    /// Creates a new instance of `Square`.
    ///
    /// `file` can take a value from 0('1') to 8('9'), while `rank` is from 0('a') to 9('i').
    pub fn new(file: u8, rank: u8) -> Option<Square> {
        if file > 8 || rank > 8 {
            return None;
        }

        Some(Square {
            inner: file * 9 + rank,
        })
    }

    /// Creates a new instance of `Square` from SFEN formatted string.
    pub fn from_sfen(s: &str) -> Option<Square> {
        let bytes: &[u8] = s.as_bytes();

        if bytes.len() != 2
            || bytes[0] < ASCII_1
            || bytes[0] > ASCII_9
            || bytes[1] < ASCII_LOWER_A
            || bytes[1] > ASCII_LOWER_I
        {
            return None;
        }

        let file = bytes[0] - ASCII_1;
        let rank = bytes[1] - ASCII_LOWER_A;

        debug_assert!(
            file < 9 && rank < 9,
            "{} parsed as (file: {}, rank: {})",
            s,
            file,
            rank
        );

        Some(Square {
            inner: file * 9 + rank,
        })
    }

    /// Creates a new instance of `Square` with the given index value.
    pub fn from_index(index: u8) -> Option<Square> {
        if index >= 81 {
            return None;
        }

        Some(Square { inner: index })
    }

    /// Returns an iterator of all variants.
    pub fn iter() -> SquareIter {
        SquareIter { current: 0 }
    }

    /// Returns a file of the square.
    pub fn file(self) -> u8 {
        self.inner / 9
    }

    /// Returns a rank of the square.
    pub fn rank(self) -> u8 {
        self.inner % 9
    }

    /// Returns a new `Square` instance by moving the file and the rank values.
    ///
    /// # Examples
    ///
    /// ```
    /// use shogi::square::consts::*;
    ///
    /// let sq = SQ_2B;
    /// let shifted = sq.shift(2, 3).unwrap();
    ///
    /// assert_eq!(3, shifted.file());
    /// assert_eq!(4, shifted.rank());
    /// ```
    pub fn shift(self, df: i8, dr: i8) -> Option<Square> {
        let f = self.file() as i8 + df;
        let r = self.rank() as i8 + dr;

        if !(0..9).contains(&f) || !(0..9).contains(&r) {
            return None;
        }

        Some(Square {
            inner: (f * 9 + r) as u8,
        })
    }

    /// Returns a relative rank as if the specified color is black.
    ///
    /// # Examples
    ///
    /// ```
    /// use shogi::Color;
    /// use shogi::square::consts::*;
    ///
    /// let sq = SQ_1G;
    ///
    /// assert_eq!(6, sq.relative_rank(Color::Black));
    /// assert_eq!(2, sq.relative_rank(Color::White));
    /// ```
    pub fn relative_rank(self, c: Color) -> u8 {
        if c == Color::Black {
            self.rank()
        } else {
            8 - self.rank()
        }
    }

    /// Tests if the square is in a promotion zone.
    pub fn in_promotion_zone(self, c: Color) -> bool {
        self.relative_rank(c) < 3
    }

    /// Converts the instance into the unique number for array indexing purpose.
    #[inline(always)]
    pub fn index(self) -> usize {
        self.inner as usize
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        debug_assert!(
            self.file() < 9 && self.rank() < 9,
            "trying to stringify an invalid square: {:?}",
            self
        );
        write!(
            f,
            "{}{}",
            (self.file() + ASCII_1) as char,
            (self.rank() + ASCII_LOWER_A) as char
        )
    }
}

pub mod consts {
    use super::Square;

    macro_rules! make_square {
        {0, $t:ident $($ts:ident)+} => {
            pub const $t: Square = Square { inner: 0 };
            make_square!{1, $($ts)*}
        };
        {$n:expr, $t:ident $($ts:ident)+} => {
            pub const $t: Square = Square { inner: $n };
            make_square!{($n + 1), $($ts)*}
        };
        {$n:expr, $t:ident} => {
            pub const $t: Square = Square { inner: $n };
        };
    }

    make_square! {0, SQ_1A SQ_1B SQ_1C SQ_1D SQ_1E SQ_1F SQ_1G SQ_1H SQ_1I
    SQ_2A SQ_2B SQ_2C SQ_2D SQ_2E SQ_2F SQ_2G SQ_2H SQ_2I
    SQ_3A SQ_3B SQ_3C SQ_3D SQ_3E SQ_3F SQ_3G SQ_3H SQ_3I
    SQ_4A SQ_4B SQ_4C SQ_4D SQ_4E SQ_4F SQ_4G SQ_4H SQ_4I
    SQ_5A SQ_5B SQ_5C SQ_5D SQ_5E SQ_5F SQ_5G SQ_5H SQ_5I
    SQ_6A SQ_6B SQ_6C SQ_6D SQ_6E SQ_6F SQ_6G SQ_6H SQ_6I
    SQ_7A SQ_7B SQ_7C SQ_7D SQ_7E SQ_7F SQ_7G SQ_7H SQ_7I
    SQ_8A SQ_8B SQ_8C SQ_8D SQ_8E SQ_8F SQ_8G SQ_8H SQ_8I
    SQ_9A SQ_9B SQ_9C SQ_9D SQ_9E SQ_9F SQ_9G SQ_9H SQ_9I}
}

/// This struct is created by the [`iter`] method on [`Square`].
///
/// [`iter`]: ./struct.Square.html#method.iter
/// [`Square`]: struct.Square.html
pub struct SquareIter {
    current: u8,
}

impl iter::Iterator for SquareIter {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        let cur = self.current;

        if cur >= 81 {
            return None;
        }

        self.current += 1;

        Some(Square { inner: cur })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        for file in 0..9 {
            for rank in 0..9 {
                let sq = Square::new(file, rank).unwrap();
                assert_eq!(file, sq.file());
                assert_eq!(rank, sq.rank());
            }
        }

        assert_eq!(None, Square::new(10, 0));
        assert_eq!(None, Square::new(0, 10));
        assert_eq!(None, Square::new(10, 10));
    }

    #[test]
    fn from_sfen() {
        let ok_cases = [
            ("9a", 8, 0),
            ("1a", 0, 0),
            ("5e", 4, 4),
            ("9i", 8, 8),
            ("1i", 0, 8),
        ];
        let ng_cases = ["", "9j", "_a", "a9", "9 ", " a", "9", "foo"];

        for case in ok_cases.iter() {
            let sq = Square::from_sfen(case.0);
            assert!(sq.is_some());
            assert_eq!(case.1, sq.unwrap().file());
            assert_eq!(case.2, sq.unwrap().rank());
        }

        for case in ng_cases.iter() {
            assert!(
                Square::from_sfen(case).is_none(),
                "{} should cause an error",
                case
            );
        }
    }

    #[test]
    fn from_index() {
        for i in 0..81 {
            assert!(Square::from_index(i).is_some());
        }

        assert!(Square::from_index(82).is_none());
    }

    #[test]
    fn to_sfen() {
        let cases = [
            ("9a", 8, 0),
            ("1a", 0, 0),
            ("5e", 4, 4),
            ("9i", 8, 8),
            ("1i", 0, 8),
        ];

        for case in cases.iter() {
            let sq = Square::new(case.1, case.2).unwrap();
            assert_eq!(case.0, sq.to_string());
        }
    }

    #[test]
    fn shift() {
        let sq = consts::SQ_5E;

        let ok_cases = [
            (-4, -4, 0, 0),
            (-4, 0, 0, 4),
            (0, -4, 4, 0),
            (0, 0, 4, 4),
            (4, 0, 8, 4),
            (0, 4, 4, 8),
            (4, 4, 8, 8),
        ];

        let ng_cases = [(-5, -4), (-4, -5), (5, 0), (0, 5)];

        for case in ok_cases.iter() {
            let shifted = sq.shift(case.0, case.1).unwrap();
            assert_eq!(case.2, shifted.file());
            assert_eq!(case.3, shifted.rank());
        }

        for case in ng_cases.iter() {
            assert!(sq.shift(case.0, case.1).is_none());
        }
    }

    #[test]
    fn relative_rank() {
        let cases = [
            (0, 0, 0, 8),
            (0, 1, 1, 7),
            (0, 2, 2, 6),
            (0, 3, 3, 5),
            (0, 4, 4, 4),
            (0, 5, 5, 3),
            (0, 6, 6, 2),
            (0, 7, 7, 1),
            (0, 8, 8, 0),
        ];

        for case in cases.iter() {
            let sq = Square::new(case.0, case.1).unwrap();
            assert_eq!(case.2, sq.relative_rank(Color::Black));
            assert_eq!(case.3, sq.relative_rank(Color::White));
        }
    }

    #[test]
    fn in_promotion_zone() {
        let cases = [
            (0, 0, true, false),
            (0, 1, true, false),
            (0, 2, true, false),
            (0, 3, false, false),
            (0, 4, false, false),
            (0, 5, false, false),
            (0, 6, false, true),
            (0, 7, false, true),
            (0, 8, false, true),
        ];

        for case in cases.iter() {
            let sq = Square::new(case.0, case.1).unwrap();
            assert_eq!(case.2, sq.in_promotion_zone(Color::Black));
            assert_eq!(case.3, sq.in_promotion_zone(Color::White));
        }
    }

    #[test]
    fn consts() {
        for (i, sq) in Square::iter().enumerate() {
            assert_eq!((i / 9) as u8, sq.file());
            assert_eq!((i % 9) as u8, sq.rank());
        }
    }
}
