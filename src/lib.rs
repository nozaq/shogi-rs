//! A library for implementing Shogi application.
//!
//! `shogi` provides a various types and implementations for representing concepts and rules in Shogi.
//! Most types can be created programatically while they can also be deserialized from / serialized to SFEN format.
//! See [USIプロトコルとは (What is the USI protocol?)](http://shogidokoro.starfree.jp/usi.html) for more detail about UCI protocol specification and SFEN format.
//!
//! # Examples
//!
//! ```
//! use shogi::{Move, Position};
//! use shogi::bitboard::Factory as BBFactory;
//! use shogi::square::consts::*;
//!
//! BBFactory::init();
//! let mut pos = Position::new();
//!
//! // Position can be set from the SFEN formatted string.
//! pos.set_sfen("lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1").unwrap();
//!
//! // You can programatically create a Move instance.
//! let m = Move::Normal{from: SQ_7G, to: SQ_7F, promote: false};
//! pos.make_move(m).unwrap();
//!
//! // Move can be created from the SFEN formatted string as well.
//! let m = Move::from_sfen("7c7d").unwrap();
//! pos.make_move(m).unwrap();
//!
//! // Position can be converted back to the SFEN formatted string.
//! assert_eq!("lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1 moves 7g7f 7c7d", pos.to_sfen());
//! ```
#![recursion_limit = "81"]

pub mod bitboard;
pub mod color;
pub mod error;
pub mod hand;
pub mod moves;
pub mod piece;
pub mod piece_type;
pub mod position;
pub mod square;
pub mod time;

pub use self::bitboard::Bitboard;
pub use self::color::Color;
pub use self::error::{MoveError, SfenError};
pub use self::hand::Hand;
pub use self::moves::Move;
pub use self::piece::Piece;
pub use self::piece_type::PieceType;
pub use self::position::{MoveRecord, Position};
pub use self::square::Square;
pub use self::time::TimeControl;
