use crate::{Color, Piece, PieceType};

/// Manages the number of each pieces in each player's hand.
///
/// # Examples
///
/// ```
/// use shogi::{Color, Hand, Piece, PieceType};
///
/// let mut hand: Hand = Default::default();
/// let black_pawn = Piece{piece_type: PieceType::Pawn, color: Color::Black};
/// let white_pawn = Piece{piece_type: PieceType::Pawn, color: Color::White};
///
/// hand.set(&black_pawn, 2);
/// hand.increment(&black_pawn);
/// assert_eq!(3, hand.get(&black_pawn));
/// assert_eq!(0, hand.get(&white_pawn));
/// ```
#[derive(Debug, Default)]
pub struct Hand {
    inner: [u8; 14],
}

impl Hand {
    /// Returns a number of the given piece.
    pub fn get(&self, p: &Piece) -> u8 {
        Hand::index(p).map(|i| self.inner[i]).unwrap_or(0)
    }

    /// Sets a number of the given piece.
    pub fn set(&mut self, p: &Piece, num: u8) {
        if let Some(i) = Hand::index(p) {
            self.inner[i] = num;
        }
    }

    /// Increments a number of the given piece.
    pub fn increment(&mut self, p: &Piece) {
        if let Some(i) = Hand::index(p) {
            self.inner[i] += 1
        }
    }

    /// Decrements a number of the given piece.
    pub fn decrement(&mut self, p: &Piece) {
        if let Some(i) = Hand::index(p) {
            self.inner[i] -= 1
        }
    }

    /// Clears all pieces.
    pub fn clear(&mut self) {
        for i in 0..self.inner.len() {
            self.inner[i] = 0;
        }
    }

    fn index(p: &Piece) -> Option<usize> {
        let base = match p.piece_type {
            PieceType::Pawn => 0,
            PieceType::Lance => 1,
            PieceType::Knight => 2,
            PieceType::Silver => 3,
            PieceType::Gold => 4,
            PieceType::Rook => 5,
            PieceType::Bishop => 6,
            _ => return None,
        };
        let offset = if p.color == Color::Black { 0 } else { 7 };

        Some(base + offset)
    }
}
