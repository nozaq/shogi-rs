use std::fmt;

///  Represents each side of player. Black player moves first.
///
/// # Examples
///
/// ```
/// use shogi::Color;
///
/// let c = Color::Black;
/// match c {
///    Color::Black => assert!(true),
///    Color::White => unreachable!(),
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Color {
    Black,
    White,
}

impl Color {
    /// Returns the color of the opposite side.
    ///
    /// # Examples
    ///
    /// ```
    /// use shogi::Color;
    ///
    /// assert_eq!(Color::White, Color::Black.flip());
    /// assert_eq!(Color::Black, Color::White.flip());
    /// ```
    pub fn flip(&self) -> Color {
        match *self {
            Color::Black => Color::White,
            Color::White => Color::Black,

        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Color::Black => write!(f, "Black"),
            Color::White => write!(f, "White"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flip() {
        assert_eq!(Color::White, Color::Black.flip());
        assert_eq!(Color::Black, Color::White.flip());
    }
}