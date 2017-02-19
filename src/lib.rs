//! A library for implementing Shogi application.
//!
//! `shogi` provides a various types and implementations for representing concepts and rules in Shogi.
//! Most types can be created programatically while they can also be deserialized from / serialized to SFEN format.
//! See http://www.geocities.jp/shogidokoro/usi.html for more detail about SFEN format.
//!
//! # Examples
//!
//! ```
//! use shogi::{Color, Move, Position, Square};
//!
//! let mut pos = Position::new();
//!
//! // Position can be set from the SFEN formatted string.
//! pos.set_sfen("lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1").unwrap();
//!
//! // You can programatically create a Move instance.
//! let m = Move::Normal{from: Square::new(2, 6), to: Square::new(2, 5), promote: false};
//! pos.make_move(&m).unwrap();
//!
//! // Move can be created from the SFEN formatted string as well.
//! let m = Move::from_sfen("7c7d").unwrap();
//! pos.make_move(&m).unwrap();
//!
//! // Position can be converted back to the SFEN formatted string.
//! assert_eq!("lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1 moves 7g7f 7c7d", pos.to_sfen());
//! ```

#[macro_use()]
extern crate itertools;

mod color;
mod error;
mod square;
mod piece_type;
mod piece;
mod moves;
mod hand;
mod position;
pub mod usi;

pub use self::color::*;
pub use self::error::*;
pub use self::square::*;
pub use self::piece_type::*;
pub use self::piece::*;
pub use self::moves::*;
pub use self::hand::*;
pub use self::position::*;