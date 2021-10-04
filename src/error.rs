use thiserror::Error;

/// The error type for SFEN serialize/deserialize operations.
#[derive(Error, Debug, PartialEq, Eq)]
pub enum SfenError {
    #[error("data fields are missing")]
    MissingDataFields,

    #[error("an illegal piece notation is found")]
    IllegalPieceType,

    #[error("the side to move needs to be black or white")]
    IllegalSideToMove,

    #[error("an illegal move count notation is found")]
    IllegalMoveCount(#[from] std::num::ParseIntError),

    #[error("an illegal move notation is found")]
    IllegalMove,

    #[error("an illegal board state notation is found")]
    IllegalBoardState,
}

/// Represents an error occurred during making a move.
#[derive(Error, Debug, PartialEq, Eq)]
pub enum MoveError {
    #[error("the king is in check")]
    InCheck,

    #[error("nifu detected")]
    Nifu,

    #[error("uchifuzume detected")]
    Uchifuzume,

    #[error("perpetual check detected")]
    PerpetualCheckWin,

    #[error("perpetual check detected")]
    PerpetualCheckLose,

    #[error("not your turn")]
    EnemysTurn,

    #[error("the piece can not move anymor")]
    NonMovablePiece,

    #[error("the move is inconsistent with the current position: {0}")]
    Inconsistent(&'static str),

    #[error("repetition detected")]
    Repetition,
}
