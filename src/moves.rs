use crate::{PieceType, Square};
use std::fmt;

/// Represents a move which either is a normal move or a drop move.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Move {
    Normal {
        from: Square,
        to: Square,
        promote: bool,
    },
    Drop {
        to: Square,
        piece_type: PieceType,
    },
}

impl Move {
    /// Creates a new instance of `Move` from SFEN formatted string.
    pub fn from_sfen(s: &str) -> Option<Move> {
        if s.len() != 4 && (s.len() != 5 || s.chars().nth(4).unwrap() != '+') {
            return None;
        }

        let first = s.chars().nth(0).unwrap();
        if first.is_digit(10) {
            if let Some(from) = Square::from_sfen(&s[0..2]) {
                if let Some(to) = Square::from_sfen(&s[2..4]) {
                    let promote = s.len() == 5;

                    return Some(Move::Normal { from, to, promote });
                }
            }

            return None;
        } else if first.is_uppercase() && s.chars().nth(1).unwrap() == '*' {
            if let Some(piece_type) = first.to_lowercase().nth(0).and_then(PieceType::from_sfen) {
                if let Some(to) = Square::from_sfen(&s[2..4]) {
                    return Some(Move::Drop { to, piece_type });
                }
            }

            return None;
        }

        None
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Move::Normal { from, to, promote } => {
                write!(f, "{}{}{}", from, to, if promote { "+" } else { "" })
            }
            Move::Drop { to, piece_type } => {
                write!(f, "{}*{}", piece_type.to_string().to_uppercase(), to)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::square::consts::*;

    #[test]
    fn from_sfen() {
        let ok_cases = [
            (
                "9a1i",
                Move::Normal {
                    from: SQ_9A,
                    to: SQ_1I,
                    promote: false,
                },
            ),
            (
                "9a1i+",
                Move::Normal {
                    from: SQ_9A,
                    to: SQ_1I,
                    promote: true,
                },
            ),
            (
                "S*5e",
                Move::Drop {
                    to: SQ_5E,
                    piece_type: PieceType::Silver,
                },
            ),
        ];
        let ng_cases = [
            "9j1i", "9a1j", "9a1", "9aj", "j1i", "9a1i1", "9a1i-", "S+5e", "S 5e", "Z*5e", "S+9j",
        ];

        for (i, case) in ok_cases.iter().enumerate() {
            let m = Move::from_sfen(case.0);
            assert!(m.is_some(), "failed at #{}", i);
            assert_eq!(case.1, m.unwrap(), "failed at #{}", i);
        }

        for (i, case) in ng_cases.iter().enumerate() {
            assert!(Move::from_sfen(case).is_none(), "failed at #{}", i);
        }
    }

    #[test]
    fn to_sfen() {
        let cases = [
            (
                "9a1i",
                Move::Normal {
                    from: SQ_9A,
                    to: SQ_1I,
                    promote: false,
                },
            ),
            (
                "9a1i+",
                Move::Normal {
                    from: SQ_9A,
                    to: SQ_1I,
                    promote: true,
                },
            ),
            (
                "S*5e",
                Move::Drop {
                    to: SQ_5E,
                    piece_type: PieceType::Silver,
                },
            ),
        ];

        for (i, case) in cases.iter().enumerate() {
            assert_eq!(case.1.to_string(), case.0, "failed at #{}", i);
        }
    }
}
