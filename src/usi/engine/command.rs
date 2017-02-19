use std::time::Duration;

use Move;
use usi::Error;
use super::parser::EngineCommandParser;

/// Represents a kind of "option" command value.
#[derive(Debug)]
pub enum OptionKind {
    Check { default: Option<bool> },
    Spin {
        default: Option<i32>,
        min: Option<i32>,
        max: Option<i32>,
    },
    Combo {
        default: Option<String>,
        vars: Vec<String>,
    },
    Button { default: Option<String> },
    String { default: Option<String> },
    Filename { default: Option<String> },
}

/// Represents parameters of "option" command.
#[derive(Debug)]
pub struct OptionParams {
    pub name: String,
    pub value: OptionKind,
}

/// Represents a kind of "score" parameter value in "info" command.
#[derive(Debug)]
pub enum ScoreKind {
    CpExact,
    CpLowerbound,
    CpUpperbound,
    MateExact,
    MateSignOnly,
    MateLowerbound,
    MateUpperbound,
}

/// Represents parameters of "info" command.
#[derive(Debug)]
pub enum InfoParams {
    CurrMove(String),
    Depth(i32, Option<i32>),
    HashFull(i32),
    MultiPv(i32),
    Nodes(i32),
    Nps(i32),
    Pv(Vec<String>),
    Score(i32, ScoreKind),
    Text(String),
    Time(Duration),
}

/// Represents parameters of "checkmate" command.
#[derive(Debug)]
pub enum CheckmateParams {
    Mate(Vec<Move>),
    NoMate,
    NotImplemented,
    Timeout,
}

/// Represents parameters of "bestmove" command.
#[derive(Debug)]
pub enum BestMoveParams {
    MakeMove(Move, Option<Move>),
    Resign,
    Win,
}

/// Represents parameters of "id" command.
#[derive(Debug)]
pub enum IdParams {
    Name(String),
    Author(String),
}

/// Represents a USI command sent from the engine.
///
/// # Examples
///
/// ```
/// use shogi::Move;
/// use shogi::usi::{EngineCommand, BestMoveParams};
///
/// let cmd = EngineCommand::parse("bestmove 7g7f ponder 8c8d").unwrap();
/// match cmd {
///     EngineCommand::BestMove(BestMoveParams::MakeMove(ref m, Some(ref pm))) => {
///         assert_eq!(Move::from_sfen("7g7f").unwrap(), *m);
///         assert_eq!(Move::from_sfen("8c8d").unwrap(), *pm);
///     },
///     _ => unreachable!(),
/// }
/// ```
#[derive(Debug)]
pub enum EngineCommand {
    Id(IdParams),
    BestMove(BestMoveParams),
    Checkmate(CheckmateParams),
    Info(Vec<InfoParams>),
    Option(OptionParams),
    ReadyOk,
    UsiOk,
    Unknown,
}

impl EngineCommand {
    /// Parses a USI command string into a new instance of `EngineCommand`.
    pub fn parse(cmd: &str) -> Result<EngineCommand, Error> {
        let parser = EngineCommandParser::new(cmd);
        parser.parse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let ok_cases =
            ["id name Lesserkai",
             "id author Program Writer",
             "bestmove 7g7f",
             "bestmove 8h2b+ ponder 3a2b",
             "bestmove resign",
             "bestmove win",
             "checkmate nomate",
             "checkmate notimplemented",
             "checkmate timeout",
             "checkmate G*8f 9f9g 8f8g 9g9h 8g8h",
             "info time 1141 depth 3 seldepth 5 nodes 135125 score cp -1521 pv 3a3b L*4h 4c4d",
             "info nodes 120000 nps 116391 multipv 1 currmove 1 hashfull 104",
             "info string 7g7f (70%)",
             "info score cp 100 lowerbound",
             "info score cp 100 upperbound",
             "info score mate +",
             "info score mate -",
             "info score mate 5",
             "info score mate -5",
             "info score mate 5 lowerbound",
             "info score mate 5 upperbound",
             "option name UseBook type check default true",
             "option name Selectivity type spin default 2 min 0 max 4",
             "option name Style type combo default Normal var Solid var Normal var Risky",
             "option name ResetLearning type button",
             "option name BookFile type string default public.bin",
             "option name LearningFile type filename default <empty>",
             "readyok",
             "usiok",
             "unknown command"];

        let ng_cases = ["",
                        "bestmove foo",
                        "bestmove foo ponder bar",
                        "bestmove 7g7f ponder foo",
                        "bestmove foo ponder 7g7f",
                        "checkmate foo",
                        "checkmate",
                        "id foo bar",
                        "info depth foo",
                        "info depth 1 seldepth foo",
                        "info multipv foo",
                        "info score foo 1",
                        "info foo bar",
                        "option foo bar baz",
                        "option name foo bar"];

        for (i, c) in ok_cases.iter().enumerate() {
            assert!(EngineCommand::parse(c).is_ok(), "failed at #{}", i);
        }

        for (i, c) in ng_cases.iter().enumerate() {
            assert!(EngineCommand::parse(c).is_err(), "failed at #{}", i);
        }
    }
}