use std::fmt;
use std::iter;

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
    /// Returns an iterator of all variants.
    pub fn iter() -> ColorIter {
        ColorIter {
            current: Some(Color::Black),
        }
    }

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
    #[must_use]
    pub fn flip(self) -> Self {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
        }
    }

    /// Converts the instance into the unique number for array indexing purpose.
    #[inline(always)]
    pub fn index(self) -> usize {
        self as usize
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

/// This struct is created by the [`iter`] method on [`Color`].
///
/// [`iter`]: enum.Color.html#method.iter
/// [`Color`]: enum.Color.html
pub struct ColorIter {
    current: Option<Color>,
}

impl iter::Iterator for ColorIter {
    type Item = Color;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        if let Some(current) = self.current {
            self.current = match current {
                Color::Black => Some(Color::White),
                Color::White => None,
            }
        }

        current
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
