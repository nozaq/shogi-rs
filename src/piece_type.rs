use std::fmt;
use std::iter;

/// Represents a kind of pieces.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PieceType {
    King,
    Rook,
    Bishop,
    Gold,
    Silver,
    Knight,
    Lance,
    Pawn,
    ProRook,
    ProBishop,
    ProSilver,
    ProKnight,
    ProLance,
    ProPawn,
}

impl PieceType {
    /// Returns an iterator over all variants.
    pub fn iter() -> PieceTypeIter {
        PieceTypeIter::new()
    }

    /// Creates a new instance of `PieceType` from SFEN formatted string.
    pub fn from_sfen(c: char) -> Option<PieceType> {
        Some(match c {
            'k' | 'K' => PieceType::King,
            'r' | 'R' => PieceType::Rook,
            'b' | 'B' => PieceType::Bishop,
            'g' | 'G' => PieceType::Gold,
            's' | 'S' => PieceType::Silver,
            'n' | 'N' => PieceType::Knight,
            'l' | 'L' => PieceType::Lance,
            'p' | 'P' => PieceType::Pawn,
            _ => return None,
        })
    }

    /// Returns an instance of `PieceType` after promotion.
    ///
    /// # Examples
    ///
    /// ```
    /// use shogi::PieceType;
    ///
    /// assert_eq!(Some(PieceType::ProPawn), PieceType::Pawn.promote());
    /// assert_eq!(None, PieceType::ProPawn.promote());
    /// ```
    pub fn promote(self) -> Option<PieceType> {
        use self::PieceType::*;

        Some(match self {
            Pawn => ProPawn,
            Lance => ProLance,
            Knight => ProKnight,
            Silver => ProSilver,
            Rook => ProRook,
            Bishop => ProBishop,
            _ => return None,
        })
    }

    /// Returns an instance of `PieceType` before promotion.
    ///
    /// # Examples
    ///
    /// ```
    /// use shogi::PieceType;
    ///
    /// assert_eq!(Some(PieceType::Pawn), PieceType::ProPawn.unpromote());
    /// assert_eq!(None, PieceType::Pawn.unpromote());
    /// ```
    pub fn unpromote(self) -> Option<PieceType> {
        use self::PieceType::*;

        Some(match self {
            ProPawn => Pawn,
            ProLance => Lance,
            ProKnight => Knight,
            ProSilver => Silver,
            ProRook => Rook,
            ProBishop => Bishop,
            _ => return None,
        })
    }

    /// Checks if this piece type can be a part of hand pieces.
    pub fn is_hand_piece(self) -> bool {
        matches!(
            self,
            PieceType::Rook
                | PieceType::Bishop
                | PieceType::Gold
                | PieceType::Silver
                | PieceType::Knight
                | PieceType::Lance
                | PieceType::Pawn
        )
    }

    /// Converts the instance into the unique number for array indexing purpose.
    #[inline(always)]
    pub fn index(self) -> usize {
        self as usize
    }
}

impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "{}",
            match *self {
                PieceType::Bishop => "b",
                PieceType::Gold => "g",
                PieceType::King => "k",
                PieceType::Lance => "l",
                PieceType::Knight => "n",
                PieceType::Pawn => "p",
                PieceType::Rook => "r",
                PieceType::Silver => "s",
                PieceType::ProBishop => "+b",
                PieceType::ProLance => "+l",
                PieceType::ProKnight => "+n",
                PieceType::ProPawn => "+p",
                PieceType::ProRook => "+r",
                PieceType::ProSilver => "+s",
            }
        )
    }
}

/// This struct is created by the [`iter`] method on [`PieceType`].
///
/// [`iter`]: ./struct.PieceType.html#method.iter
/// [`PieceType`]: struct.PieceType.html
pub struct PieceTypeIter {
    current: Option<PieceType>,
}

impl PieceTypeIter {
    fn new() -> PieceTypeIter {
        PieceTypeIter {
            current: Some(PieceType::King),
        }
    }
}

impl iter::Iterator for PieceTypeIter {
    type Item = PieceType;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        if let Some(current) = self.current {
            self.current = match current {
                PieceType::King => Some(PieceType::Rook),
                PieceType::Rook => Some(PieceType::Bishop),
                PieceType::Bishop => Some(PieceType::Gold),
                PieceType::Gold => Some(PieceType::Silver),
                PieceType::Silver => Some(PieceType::Knight),
                PieceType::Knight => Some(PieceType::Lance),
                PieceType::Lance => Some(PieceType::Pawn),
                PieceType::Pawn => Some(PieceType::ProRook),
                PieceType::ProRook => Some(PieceType::ProBishop),
                PieceType::ProBishop => Some(PieceType::ProSilver),
                PieceType::ProSilver => Some(PieceType::ProKnight),
                PieceType::ProKnight => Some(PieceType::ProLance),
                PieceType::ProLance => Some(PieceType::ProPawn),
                PieceType::ProPawn => None,
            };
        }

        current
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_sfen() {
        let ok_cases = [
            ('k', PieceType::King),
            ('r', PieceType::Rook),
            ('b', PieceType::Bishop),
            ('g', PieceType::Gold),
            ('s', PieceType::Silver),
            ('n', PieceType::Knight),
            ('l', PieceType::Lance),
            ('p', PieceType::Pawn),
        ];
        let ng_cases = ['\0', ' ', '_', 'a', 'z', '+'];

        for case in ok_cases.iter() {
            assert_eq!(Some(case.1), PieceType::from_sfen(case.0));
            assert_eq!(
                Some(case.1),
                PieceType::from_sfen(case.0.to_uppercase().next().unwrap())
            );
        }

        for case in ng_cases.iter() {
            assert!(PieceType::from_sfen(*case).is_none());
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
            assert_eq!(case.0, case.1.to_string());
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
            assert_eq!(Some(case.1), case.0.promote());
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
            assert_eq!(Some(case.1), case.0.unpromote());
        }

        for case in ng_cases.iter() {
            assert!(case.unpromote().is_none());
        }
    }
}
