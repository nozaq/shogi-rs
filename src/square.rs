use std::fmt;
use Color;

const ASCII_1: u8 = '1' as u8;
const ASCII_9: u8 = '9' as u8;
const ASCII_LOWER_A: u8 = 'a' as u8;
const ASCII_LOWER_I: u8 = 'i' as u8;

/// Represents a position of each cell in the game board.
///
/// # Examples
///
/// ```
/// use shogi::Square;
///
/// // (4, 4) represents 5e.
/// let sq = Square::new(4, 4);
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
    /// `file` and `rank` need to be lower than 9 the returned instance to be valid value.
    pub fn new(file: u8, rank: u8) -> Square {
        Square { inner: file | rank << 4 }
    }

    /// Creates a new instance of `Square` from SFEN formatted string.
    pub fn from_sfen(s: &str) -> Option<Square> {
        let bytes: &[u8] = s.as_bytes();

        if bytes.len() != 2 || bytes[0] < ASCII_1 || bytes[0] > ASCII_9 ||
           bytes[1] < ASCII_LOWER_A || bytes[1] > ASCII_LOWER_I {
            return None;
        }

        let file = ASCII_9 - bytes[0];
        let rank = bytes[1] - ASCII_LOWER_A;

        debug_assert!(file < 9 && rank < 9,
                      "{} parsed as (file: {}, rank: {})",
                      s,
                      file,
                      rank);

        Some(Square::new(file, rank))
    }

    /// Returns a file of the square.
    pub fn file(&self) -> u8 {
        self.inner & 0x0F
    }

    /// Returns a rank of the square.
    pub fn rank(&self) -> u8 {
        self.inner >> 4
    }

    /// Tests if both the file and the rank are valid values.
    ///
    /// # Examples
    ///
    /// ```
    /// use shogi::Square;
    ///
    /// assert!(Square::new(0, 0).is_valid());
    /// assert!(!Square::new(9, 9).is_valid());
    /// ```
    pub fn is_valid(&self) -> bool {
        let f = self.file();
        let r = self.rank();

        f < 9 && r < 9
    }

    /// Returns a new `Square` instance by moving the file and the rank values.
    ///
    /// # Examples
    ///
    /// ```
    /// use shogi::Square;
    ///
    /// let sq = Square::new(1, 1);
    /// let shifted = sq.shift(2, 3);
    ///
    /// assert_eq!(3, shifted.file());
    /// assert_eq!(4, shifted.rank());
    /// ```
    pub fn shift(&self, df: i8, dr: i8) -> Square {
        Square::new((self.file() as i8 + df) as u8,
                    (self.rank() as i8 + dr) as u8)
    }

    /// Returns a relative rank as if the specified color is black.
    ///
    /// # Examples
    ///
    /// ```
    /// use shogi::{Color, Square};
    ///
    /// let sq = Square::new(0, 6);
    ///
    /// assert_eq!(6, sq.relative_rank(Color::Black));
    /// assert_eq!(2, sq.relative_rank(Color::White));
    /// ```
    pub fn relative_rank(&self, c: Color) -> u8 {
        if c == Color::Black {
            self.rank()
        } else {
            8 - self.rank()
        }
    }

    /// Tests if the square is in a promotion zone.
    pub fn in_promotion_zone(&self, c: Color) -> bool {
        self.relative_rank(c) < 3
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        debug_assert!(self.file() < 9 && self.rank() < 9,
                      "trying to stringify an invalid square: {:?}",
                      self);
        write!(f,
               "{}{}",
               (ASCII_9 - self.file()) as char,
               (self.rank() + ASCII_LOWER_A) as char)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        for file in 0..9 {
            for rank in 0..9 {
                let sq = Square::new(file, rank);
                assert_eq!(file, sq.file());
                assert_eq!(rank, sq.rank());
            }
        }
    }

    #[test]
    fn from_sfen() {
        let ok_cases = [("9a", 0, 0), ("1a", 8, 0), ("5e", 4, 4), ("9i", 0, 8), ("1i", 8, 8)];
        let ng_cases = ["", "9j", "_a", "a9", "9 ", " a", "9", "foo"];

        for case in ok_cases.iter() {
            let sq = Square::from_sfen(case.0);
            assert!(sq.is_some());
            assert_eq!(case.1, sq.unwrap().file());
            assert_eq!(case.2, sq.unwrap().rank());
        }

        for case in ng_cases.iter() {
            assert!(Square::from_sfen(case).is_none(),
                    "{} should cause an error",
                    case);
        }
    }

    #[test]
    fn to_sfen() {
        let cases = [("9a", 0, 0), ("1a", 8, 0), ("5e", 4, 4), ("9i", 0, 8), ("1i", 8, 8)];

        for case in cases.iter() {
            let sq = Square::new(case.1, case.2);
            assert_eq!(case.0, sq.to_string());
        }
    }

    #[test]
    fn is_valid() {
        let cases = [(0, 0, true),
                     (8, 0, true),
                     (4, 4, true),
                     (0, 8, true),
                     (8, 8, true),
                     (9, 0, false),
                     (0, 9, false),
                     (9, 9, false),
                     (255, 255, false)];

        for case in cases.iter() {
            let sq = Square::new(case.0, case.1);
            assert_eq!(case.2, sq.is_valid());
        }
    }

    #[test]
    fn shift() {
        let sq = Square::new(4, 4);

        let cases = [(-4, -4, 0, 0),
                     (-4, 0, 0, 4),
                     (0, -4, 4, 0),
                     (0, 0, 4, 4),
                     (4, 0, 8, 4),
                     (0, 4, 4, 8),
                     (4, 4, 8, 8)];

        for case in cases.iter() {
            let shifted = sq.shift(case.0, case.1);
            assert_eq!(case.2, shifted.file());
            assert_eq!(case.3, shifted.rank());
        }
    }

    #[test]
    fn relative_rank() {
        let cases = [(0, 0, 0, 8),
                     (0, 1, 1, 7),
                     (0, 2, 2, 6),
                     (0, 3, 3, 5),
                     (0, 4, 4, 4),
                     (0, 5, 5, 3),
                     (0, 6, 6, 2),
                     (0, 7, 7, 1),
                     (0, 8, 8, 0)];

        for case in cases.iter() {
            let sq = Square::new(case.0, case.1);
            assert_eq!(case.2, sq.relative_rank(Color::Black));
            assert_eq!(case.3, sq.relative_rank(Color::White));
        }
    }

    #[test]
    fn in_promotion_zone() {
        let cases = [(0, 0, true, false),
                     (0, 1, true, false),
                     (0, 2, true, false),
                     (0, 3, false, false),
                     (0, 4, false, false),
                     (0, 5, false, false),
                     (0, 6, false, true),
                     (0, 7, false, true),
                     (0, 8, false, true)];

        for case in cases.iter() {
            let sq = Square::new(case.0, case.1);
            assert_eq!(case.2, sq.in_promotion_zone(Color::Black));
            assert_eq!(case.3, sq.in_promotion_zone(Color::White));
        }
    }
}