use crate::{Color, PieceType, Square};
use std::fmt;

/// Represents a piece on the game board.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
}

impl Piece {
    /// Creates a new instance of `Piece` from SFEN formatted string.
    pub fn from_sfen(c: char) -> Option<Piece> {
        let color = if c.is_uppercase() {
            Color::Black
        } else {
            Color::White
        };

        PieceType::from_sfen(c).map(|pt| Piece {
            piece_type: pt,
            color: color,
        })
    }

    /// Returns an instance of `Piece` after promotion.
    ///
    /// # Examples
    ///
    /// ```
    /// use shogi::{Color, PieceType, Piece};
    ///
    /// let pc1 = Piece{piece_type: PieceType::Pawn, color: Color::Black};
    /// let pc2 = Piece{piece_type: PieceType::ProPawn, color: Color::Black};
    ///
    /// assert_eq!(Some(pc2), pc1.promote());
    /// assert_eq!(None, pc2.promote());
    /// ```
    pub fn promote(&self) -> Option<Piece> {
        self.piece_type.promote().map(|pt| Piece {
            piece_type: pt,
            color: self.color,
        })
    }

    /// Returns an instance of `Piece` before promotion.
    ///
    /// # Examples
    ///
    /// ```
    /// use shogi::{Color, PieceType, Piece};
    ///
    /// let pc1 = Piece{piece_type: PieceType::Pawn, color: Color::Black};
    /// let pc2 = Piece{piece_type: PieceType::ProPawn, color: Color::Black};
    ///
    /// assert_eq!(Some(pc1), pc2.unpromote());
    /// assert_eq!(None, pc1.unpromote());
    /// ```
    pub fn unpromote(&self) -> Option<Piece> {
        self.piece_type.unpromote().map(|pt| Piece {
            piece_type: pt,
            color: self.color,
        })
    }

    /// Returns an instance of `Piece` with the reversed color.
    ///
    /// # Examples
    ///
    /// ```
    /// use shogi::{Color, PieceType, Piece};
    ///
    /// let pc1 = Piece{piece_type: PieceType::Pawn, color: Color::Black};
    /// let pc2 = Piece{piece_type: PieceType::Pawn, color: Color::White};
    ///
    /// assert_eq!(pc2, pc1.flip());
    /// assert_eq!(pc1, pc2.flip());
    /// ```
    pub fn flip(&self) -> Piece {
        Piece {
            piece_type: self.piece_type,
            color: self.color.flip(),
        }
    }

    /// Tests if it is legal to place this piece at the given square.
    pub fn is_placeable_at(&self, sq: Square) -> bool {
        match self.piece_type {
            PieceType::Pawn | PieceType::Lance => sq.relative_rank(self.color) > 0,
            PieceType::Knight => sq.relative_rank(self.color) > 1,
            _ => true,
        }
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        if self.color == Color::Black {
            write!(f, "{}", self.piece_type.to_string().to_uppercase())
        } else {
            write!(f, "{}", self.piece_type)
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
            ('k', PieceType::King, Color::White),
            ('r', PieceType::Rook, Color::White),
            ('b', PieceType::Bishop, Color::White),
            ('g', PieceType::Gold, Color::White),
            ('s', PieceType::Silver, Color::White),
            ('n', PieceType::Knight, Color::White),
            ('l', PieceType::Lance, Color::White),
            ('p', PieceType::Pawn, Color::White),
            ('K', PieceType::King, Color::Black),
            ('R', PieceType::Rook, Color::Black),
            ('B', PieceType::Bishop, Color::Black),
            ('G', PieceType::Gold, Color::Black),
            ('S', PieceType::Silver, Color::Black),
            ('N', PieceType::Knight, Color::Black),
            ('L', PieceType::Lance, Color::Black),
            ('P', PieceType::Pawn, Color::Black),
        ];
        let ng_cases = ['\0', ' ', '_', 'a', 'z', '+', 'A', 'Z'];

        for case in ok_cases.iter() {
            let pc = Piece::from_sfen(case.0);
            assert!(pc.is_some());
            assert_eq!(case.1, pc.unwrap().piece_type);
            assert_eq!(case.2, pc.unwrap().color);
        }

        for case in ng_cases.iter() {
            assert!(Piece::from_sfen(*case).is_none());
        }
    }

    #[test]
    fn to_sfen() {
        let ok_cases = [
            ("k", PieceType::King),
            ("r", PieceType::Rook),
            ("b", PieceType::Bishop),
            ("g", PieceType::Gold),
            ("s", PieceType::Silver),
            ("n", PieceType::Knight),
            ("l", PieceType::Lance),
            ("p", PieceType::Pawn),
            ("+r", PieceType::ProRook),
            ("+b", PieceType::ProBishop),
            ("+s", PieceType::ProSilver),
            ("+n", PieceType::ProKnight),
            ("+l", PieceType::ProLance),
            ("+p", PieceType::ProPawn),
        ];

        for case in ok_cases.iter() {
            let bpc = Piece {
                piece_type: case.1,
                color: Color::Black,
            };
            let wpc = Piece {
                piece_type: case.1,
                color: Color::White,
            };
            assert_eq!(case.0.to_uppercase(), bpc.to_string());
            assert_eq!(case.0, wpc.to_string());
        }
    }

    #[test]
    fn promote() {
        let ok_cases = [
            (PieceType::Rook, PieceType::ProRook),
            (PieceType::Bishop, PieceType::ProBishop),
            (PieceType::Silver, PieceType::ProSilver),
            (PieceType::Knight, PieceType::ProKnight),
            (PieceType::Lance, PieceType::ProLance),
            (PieceType::Pawn, PieceType::ProPawn),
        ];
        let ng_cases = [
            PieceType::King,
            PieceType::Gold,
            PieceType::ProRook,
            PieceType::ProBishop,
            PieceType::ProSilver,
            PieceType::ProKnight,
            PieceType::ProLance,
            PieceType::ProPawn,
        ];

        for case in ok_cases.iter() {
            let bpc = Piece {
                piece_type: case.0,
                color: Color::Black,
            }
            .promote()
            .unwrap();
            let wpc = Piece {
                piece_type: case.0,
                color: Color::White,
            }
            .promote()
            .unwrap();

            assert_eq!(
                Piece {
                    piece_type: case.1,
                    color: Color::Black,
                },
                bpc
            );
            assert_eq!(
                Piece {
                    piece_type: case.1,
                    color: Color::White,
                },
                wpc
            );
        }

        for case in ng_cases.iter() {
            assert!(case.promote().is_none());
        }
    }

    #[test]
    fn unpromote() {
        let ok_cases = [
            (PieceType::ProRook, PieceType::Rook),
            (PieceType::ProBishop, PieceType::Bishop),
            (PieceType::ProSilver, PieceType::Silver),
            (PieceType::ProKnight, PieceType::Knight),
            (PieceType::ProLance, PieceType::Lance),
            (PieceType::ProPawn, PieceType::Pawn),
        ];
        let ng_cases = [
            PieceType::King,
            PieceType::Rook,
            PieceType::Bishop,
            PieceType::Gold,
            PieceType::Silver,
            PieceType::Knight,
            PieceType::Lance,
            PieceType::Pawn,
        ];

        for case in ok_cases.iter() {
            let bpc = Piece {
                piece_type: case.0,
                color: Color::Black,
            }
            .unpromote()
            .unwrap();
            let wpc = Piece {
                piece_type: case.0,
                color: Color::White,
            }
            .unpromote()
            .unwrap();

            assert_eq!(
                Piece {
                    piece_type: case.1,
                    color: Color::Black,
                },
                bpc
            );
            assert_eq!(
                Piece {
                    piece_type: case.1,
                    color: Color::White,
                },
                wpc
            );
        }

        for case in ng_cases.iter() {
            assert!(case.unpromote().is_none());
        }
    }

    #[test]
    fn flip() {
        let bpc = Piece {
            piece_type: PieceType::Pawn,
            color: Color::Black,
        };
        let wpc = Piece {
            piece_type: PieceType::Pawn,
            color: Color::White,
        };

        assert_eq!(Color::White, bpc.flip().color);
        assert_eq!(Color::Black, wpc.flip().color);
    }

    #[test]
    fn is_placeable_at() {
        let cases = [
            (SQ_1A, PieceType::Pawn, false, true),
            (SQ_1B, PieceType::Pawn, true, true),
            (SQ_1H, PieceType::Pawn, true, true),
            (SQ_1I, PieceType::Pawn, true, false),
            (SQ_1A, PieceType::Lance, false, true),
            (SQ_1B, PieceType::Lance, true, true),
            (SQ_1H, PieceType::Lance, true, true),
            (SQ_1I, PieceType::Lance, true, false),
            (SQ_1A, PieceType::Knight, false, true),
            (SQ_1B, PieceType::Knight, false, true),
            (SQ_1C, PieceType::Knight, true, true),
            (SQ_1G, PieceType::Knight, true, true),
            (SQ_1H, PieceType::Knight, true, false),
            (SQ_1I, PieceType::Knight, true, false),
        ];

        for case in cases.iter() {
            let sq = case.0;
            let bpc = Piece {
                piece_type: case.1,
                color: Color::Black,
            };
            let wpc = Piece {
                piece_type: case.1,
                color: Color::White,
            };
            assert_eq!(case.2, bpc.is_placeable_at(sq));
            assert_eq!(case.3, wpc.is_placeable_at(sq));
        }
    }
}
