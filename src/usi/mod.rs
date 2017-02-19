//! Types representing commands defined in USI protocol.
//!
//! USI protocol defines commands sent from either GUIs or engines.
//! Detail about USI protocol is found at http://www.geocities.jp/shogidokoro/usi.html.
//!
//! # Examples
//!
//! ```
//! use std::time::Duration;
//! use shogi::Move;
//! use shogi::usi::{GuiCommand, ThinkParams, EngineCommand, BestMoveParams};
//!
//! // GuiCommand can be converted into the USI compliant string.
//! let params = ThinkParams::new().btime(Duration::from_secs(1)).wtime(Duration::from_secs(2));
//! let cmd = GuiCommand::Go(params);
//! assert_eq!("go btime 1000 wtime 2000", cmd.to_string());
//!
//! // EngineCommand can be parsed from the command string sent from the USI engine.
//! let cmd = EngineCommand::parse("bestmove 7g7f ponder 8c8d").unwrap();
//! match cmd {
//!     EngineCommand::BestMove(BestMoveParams::MakeMove(ref m, Some(ref pm))) => {
//!         assert_eq!(Move::from_sfen("7g7f").unwrap(), *m);
//!         assert_eq!(Move::from_sfen("8c8d").unwrap(), *pm);
//!     },
//!     _ => unreachable!(),
//! }
//! ```

mod engine;
mod error;
mod gui;

pub use self::error::*;
pub use self::engine::*;
pub use self::gui::*;