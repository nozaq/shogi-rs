use std::cmp::min;
use std::time::Duration;

use crate::Color;

/// Represents various time controls.
///
/// Currently
/// [Byo-yomi](https://en.wikipedia.org/wiki/Time_control#Japanese_byo-yomi) and
/// [Fischer Clock](https://en.wikipedia.org/wiki/Time_control#Compensation_.28delay_or_increment_methods.29)
/// are supported.
///
/// # Examples
///
/// ```
/// use std::time::Duration;
/// use shogi::{Color, TimeControl};
///
/// let mut byoyomi = TimeControl::Byoyomi{
///     black_time: Duration::from_secs(10),
///     white_time: Duration::from_secs(10),
///     byoyomi: Duration::from_secs(5)
/// };
///
/// // Black player can use the time up to black_time + byoyomi.
/// byoyomi.consume(Color::Black, Duration::from_secs(15));
/// assert_eq!(Duration::from_secs(0), byoyomi.black_time());
/// assert_eq!(Duration::from_secs(10), byoyomi.white_time());
/// ```
///
/// ```
/// use std::time::Duration;
/// use shogi::{Color, TimeControl};
///
/// let mut fischer_clock = TimeControl::FischerClock{
///     black_time: Duration::from_secs(10),
///     white_time: Duration::from_secs(10),
///     black_inc: Duration::from_secs(1),
///     white_inc: Duration::from_secs(1)
/// };
///
/// // White player gets additional 1 second after the black move.
/// fischer_clock.consume(Color::Black, Duration::from_secs(3));
/// assert_eq!(Duration::from_secs(7), fischer_clock.black_time());
/// assert_eq!(Duration::from_secs(11), fischer_clock.white_time());
/// ```
#[derive(Debug, Clone, Copy)]
pub enum TimeControl {
    Byoyomi {
        black_time: Duration,
        white_time: Duration,
        byoyomi: Duration,
    },
    FischerClock {
        black_time: Duration,
        white_time: Duration,
        black_inc: Duration,
        white_inc: Duration,
    },
}

impl TimeControl {
    /// Returns the current remaining time for the black player.
    pub fn black_time(&self) -> Duration {
        match *self {
            TimeControl::Byoyomi { black_time, .. } => black_time,
            TimeControl::FischerClock { black_time, .. } => black_time,
        }
    }

    /// Returns the current remaining time for the white player.
    pub fn white_time(&self) -> Duration {
        match *self {
            TimeControl::Byoyomi { white_time, .. } => white_time,
            TimeControl::FischerClock { white_time, .. } => white_time,
        }
    }

    /// Updates the current remaining time after consuming the given amount of time for the given player.
    ///
    /// Returns false if the given player runs out of time, true otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::time::Duration;
    /// use shogi::{Color, TimeControl};
    ///
    /// let mut byoyomi = TimeControl::Byoyomi{
    ///     black_time: Duration::from_secs(10),
    ///     white_time: Duration::from_secs(10),
    ///     byoyomi: Duration::from_secs(5)
    /// };
    ///
    /// assert!(byoyomi.consume(Color::Black, Duration::from_secs(15)));
    /// assert!(!byoyomi.consume(Color::White, Duration::from_secs(20)));
    /// ```
    pub fn consume(&mut self, c: Color, d: Duration) -> bool {
        match self {
            &mut TimeControl::Byoyomi {
                ref mut black_time,
                ref mut white_time,
                ref byoyomi,
            } => {
                let target_time = if c == Color::Black {
                    black_time
                } else {
                    white_time
                };

                if d > (*target_time + *byoyomi) {
                    return false;
                }
                *target_time -= min(*target_time, d);
            }
            &mut TimeControl::FischerClock {
                ref mut black_time,
                ref mut white_time,
                ref black_inc,
                ref white_inc,
            } => {
                let (stm_time, opponent_time, inc_time) = if c == Color::Black {
                    (black_time, white_time, white_inc)
                } else {
                    (white_time, black_time, black_inc)
                };

                if d > *stm_time {
                    return false;
                }
                *stm_time -= d;
                *opponent_time += *inc_time;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn consume_byoyomi() {
        // (black|white)_time, byoyomi, consume, remaining
        let ok_cases = [
            (5000, 1000, 1000, 4000),
            (5000, 1000, 5000, 0),
            (5000, 1000, 6000, 0),
        ];

        // (black|white)_time, byoyomi, consume
        let ng_cases = [(5000, 1000, 6001), (5000, 0, 5001)];

        for case in ok_cases.iter() {
            let mut t = TimeControl::Byoyomi {
                black_time: Duration::from_millis(case.0),
                white_time: Duration::from_millis(case.0),
                byoyomi: Duration::from_millis(case.1),
            };

            assert!(t.consume(Color::Black, Duration::from_millis(case.2)));
            assert_eq!(Duration::from_millis(case.3), t.black_time());
            assert_eq!(Duration::from_millis(case.0), t.white_time());

            assert!(t.consume(Color::White, Duration::from_millis(case.2)));
            assert_eq!(Duration::from_millis(case.3), t.black_time());
            assert_eq!(Duration::from_millis(case.3), t.white_time());
        }

        for case in ng_cases.iter() {
            let mut t = TimeControl::Byoyomi {
                black_time: Duration::from_millis(case.0),
                white_time: Duration::from_millis(case.0),
                byoyomi: Duration::from_millis(case.1),
            };

            assert!(!t.consume(Color::Black, Duration::from_millis(case.2)));
            assert!(!t.consume(Color::White, Duration::from_millis(case.2)));
        }
    }

    #[test]
    fn consume_fischer() {
        // black_time, white_time, black_inc, white_inc, consume, remaining_black, remaining_white
        let ok_cases = [
            (50, 50, 5, 5, 10, 40, 55),
            (50, 50, 5, 5, 50, 0, 55),
            (50, 50, 0, 0, 50, 0, 50),
        ];

        // black_time, white_time, black_inc, white_inc, consume
        let ng_cases = [(50, 50, 5, 5, 51)];

        for case in ok_cases.iter() {
            let mut t = TimeControl::FischerClock {
                black_time: Duration::from_secs(case.0),
                white_time: Duration::from_secs(case.1),
                black_inc: Duration::from_secs(case.2),
                white_inc: Duration::from_secs(case.3),
            };

            assert!(t.consume(Color::Black, Duration::from_secs(case.4)));
            assert_eq!(Duration::from_secs(case.5), t.black_time());
            assert_eq!(Duration::from_secs(case.6), t.white_time());
        }

        for case in ng_cases.iter() {
            let mut t = TimeControl::FischerClock {
                black_time: Duration::from_secs(case.0),
                white_time: Duration::from_secs(case.1),
                black_inc: Duration::from_secs(case.2),
                white_inc: Duration::from_secs(case.3),
            };

            assert!(!t.consume(Color::Black, Duration::from_secs(case.4)));
            assert!(!t.consume(Color::White, Duration::from_secs(case.4)));
        }
    }
}
