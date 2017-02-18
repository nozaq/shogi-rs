use std::error;
use std::fmt;
use std::io;
use std::num::ParseIntError;

/// The error type for SFEN serialize/deserialize operations.
#[derive(Debug, PartialEq, Eq)]
pub struct SfenError {}

impl fmt::Display for SfenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "illegal SFEN string")
    }
}

impl error::Error for SfenError {
    fn description(&self) -> &str {
        "illegal SFEN string"
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

impl From<ParseIntError> for SfenError {
    fn from(_: ParseIntError) -> SfenError {
        SfenError {}
    }
}

impl From<io::Error> for SfenError {
    fn from(_: io::Error) -> SfenError {
        SfenError {}
    }
}

/// Represents an error occurred during making a move.
#[derive(Debug, PartialEq, Eq)]
pub enum MoveError {
    InCheck,
    Nifu,
    Uchifuzume,
    PerpetualCheckWin,
    PerpetualCheckLose,
    EnemysTurn,
    NonMovablePiece,
    Inconsistent,
    Repetition,
}

impl fmt::Display for MoveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MoveError::InCheck => write!(f, "the king is in check"),
            MoveError::Nifu => write!(f, "nifu detected"),
            MoveError::Uchifuzume => write!(f, "uchifuzume detected"),
            MoveError::PerpetualCheckWin => write!(f, "perpetual check detected"),
            MoveError::PerpetualCheckLose => write!(f, "perpetual check detected"),
            MoveError::EnemysTurn => write!(f, "not your turn"),
            MoveError::NonMovablePiece => write!(f, "the piece can not move anymore"),
            MoveError::Inconsistent => {
                write!(f, "the move is inconsistent with the current positoin")
            }
            MoveError::Repetition => write!(f, "repetition detected"),
        }
    }
}

impl error::Error for MoveError {
    fn description(&self) -> &str {
        match *self {
            MoveError::InCheck => "the king is in check",
            MoveError::Nifu => "nifu detected",
            MoveError::Uchifuzume => "uchifuzume detected",
            MoveError::PerpetualCheckWin => "perpetual check detected",
            MoveError::PerpetualCheckLose => "perpetual check detected",
            MoveError::EnemysTurn => "not your turn",
            MoveError::NonMovablePiece => "the piece can not move anymore",
            MoveError::Inconsistent => "the move is inconsistent with the current positoin",
            MoveError::Repetition => "repetition detected",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            _ => None,
        }
    }
}
