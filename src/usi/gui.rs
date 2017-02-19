use std::fmt;
use std::time::Duration;

/// Represents parameters of "gameover" command.
pub enum GameOverKind {
    Win,
    Lose,
    Draw,
}

impl fmt::Display for GameOverKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GameOverKind::Win => write!(f, "win"),
            GameOverKind::Lose => write!(f, "lose"),
            GameOverKind::Draw => write!(f, "draw"),
        }
    }
}

/// Represents parameters of "go" command.
#[derive(Debug, Clone)]
pub struct ThinkParams {
    ponder: bool,
    btime: Option<Duration>,
    wtime: Option<Duration>,
    byoyomi: Option<Duration>,
    binc: Option<Duration>,
    winc: Option<Duration>,
    infinite: bool,
    mate: Option<Option<Duration>>,
}

impl ThinkParams {
    pub fn new() -> ThinkParams {
        ThinkParams {
            ponder: false,
            btime: None,
            wtime: None,
            byoyomi: None,
            binc: None,
            winc: None,
            infinite: false,
            mate: None,
        }
    }

    pub fn ponder(mut self) -> ThinkParams {
        self.ponder = true;
        self
    }

    pub fn btime(mut self, t: Duration) -> ThinkParams {
        self.btime = Some(t);
        self
    }

    pub fn wtime(mut self, t: Duration) -> ThinkParams {
        self.wtime = Some(t);
        self
    }

    pub fn byoyomi(mut self, t: Duration) -> ThinkParams {
        self.byoyomi = Some(t);
        self
    }

    pub fn binc(mut self, t: Duration) -> ThinkParams {
        self.binc = Some(t);
        self
    }

    pub fn winc(mut self, t: Duration) -> ThinkParams {
        self.winc = Some(t);
        self
    }

    pub fn infinite(mut self) -> ThinkParams {
        self.infinite = true;
        self
    }

    pub fn mate(mut self, t: Option<Duration>) -> ThinkParams {
        self.mate = Some(t);
        self
    }
}

impl fmt::Display for ThinkParams {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.ponder {
            try!(write!(f, " ponder"));
        }
        if let Some(t) = self.btime {
            try!(write!(f, " btime {}", to_ms(t)));
        }
        if let Some(t) = self.wtime {
            try!(write!(f, " wtime {}", to_ms(t)));
        }
        if let Some(t) = self.byoyomi {
            try!(write!(f, " byoyomi {}", to_ms(t)));
        }
        if let Some(t) = self.binc {
            try!(write!(f, " binc {}", to_ms(t)));
        }
        if let Some(t) = self.winc {
            try!(write!(f, " winc {}", to_ms(t)));
        }
        if self.infinite {
            try!(write!(f, " infinite"));
        }
        if let Some(mate_opts) = self.mate {
            if let Some(t) = mate_opts {
                try!(write!(f, " mate {}", to_ms(t)));
            } else {
                try!(write!(f, " mate infinite"));
            }
        }

        Ok(())
    }
}

/// Represents a USI command sent from the GUI.
///
/// # Examples
///
/// ```
/// use std::time::Duration;
/// use shogi::usi::{GuiCommand, ThinkParams};
///
/// let params = ThinkParams::new().btime(Duration::from_secs(1)).wtime(Duration::from_secs(2));
/// let cmd = GuiCommand::Go(params);
///
/// assert_eq!("go btime 1000 wtime 2000", cmd.to_string());
/// ```
pub enum GuiCommand {
    GameOver(GameOverKind),
    Go(ThinkParams),
    IsReady,
    Ponderhit,
    Position(String),
    SetOption(String, Option<String>),
    Stop,
    Usi,
    UsiNewGame,
    Quit,
}

impl fmt::Display for GuiCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GuiCommand::GameOver(ref r) => write!(f, "gameover {}", r),
            GuiCommand::Go(ref opt) => write!(f, "go{}", opt),
            GuiCommand::IsReady => write!(f, "isready"),
            GuiCommand::Ponderhit => write!(f, "ponderhit"),
            GuiCommand::Position(ref s) => write!(f, "position sfen {}", s),
            GuiCommand::SetOption(ref n, None) => write!(f, "setoption name {}", n),
            GuiCommand::SetOption(ref n, Some(ref v)) => {
                write!(f, "setoption name {} value {}", n, v)
            }
            GuiCommand::Stop => write!(f, "stop"),
            GuiCommand::Usi => write!(f, "usi"),
            GuiCommand::UsiNewGame => write!(f, "usinewgame"),
            GuiCommand::Quit => write!(f, "quit"),
        }
    }
}

fn to_ms(t: Duration) -> u64 {
    1000 * t.as_secs() + (t.subsec_nanos() as u64) / 1000_000
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_string() {
        let cases =
            [("gameover win", GuiCommand::GameOver(GameOverKind::Win)),
             ("gameover draw", GuiCommand::GameOver(GameOverKind::Draw)),
             ("gameover lose", GuiCommand::GameOver(GameOverKind::Lose)),
             ("go btime 60000 wtime 50000 byoyomi 10000",
              GuiCommand::Go(ThinkParams::new()
                  .btime(Duration::from_secs(60))
                  .wtime(Duration::from_secs(50))
                  .byoyomi(Duration::from_secs(10)))),
             ("go btime 40000 wtime 50000 binc 10000 winc 10000",
              GuiCommand::Go(ThinkParams::new()
                  .btime(Duration::from_secs(40))
                  .wtime(Duration::from_secs(50))
                  .binc(Duration::from_secs(10))
                  .winc(Duration::from_secs(10)))),
             ("go infinite", GuiCommand::Go(ThinkParams::new().infinite())),
             ("go mate 60000",
              GuiCommand::Go(ThinkParams::new().mate(Some(Duration::from_secs(60))))),
             ("go mate infinite", GuiCommand::Go(ThinkParams::new().mate(None))),
             ("go ponder", GuiCommand::Go(ThinkParams::new().ponder())),
             ("isready", GuiCommand::IsReady),
             ("ponderhit", GuiCommand::Ponderhit),
             ("position sfen lnsgkgsn1/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL w - 1",
              GuiCommand::Position("lnsgkgsn1/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL \
                                    w - 1"
                  .to_string())),
             ("setoption name foo", GuiCommand::SetOption("foo".to_string(), None)),
             ("setoption name foo value bar",
              GuiCommand::SetOption("foo".to_string(), Some("bar".to_string()))),
             ("stop", GuiCommand::Stop),
             ("usi", GuiCommand::Usi),
             ("usinewgame", GuiCommand::UsiNewGame),
             ("quit", GuiCommand::Quit)];

        for c in &cases {
            assert_eq!(c.0, c.1.to_string());
        }
    }
}