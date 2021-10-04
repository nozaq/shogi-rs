use itertools::Itertools;
use std::fmt;

use crate::bitboard::Factory as BBFactory;
use crate::{Bitboard, Color, Hand, Move, MoveError, Piece, PieceType, SfenError, Square};

/// MoveRecord stores information necessary to undo the move.
#[derive(Debug)]
pub enum MoveRecord {
    Normal {
        from: Square,
        to: Square,
        placed: Piece,
        captured: Option<Piece>,
        promoted: bool,
    },
    Drop {
        to: Square,
        piece: Piece,
    },
}

impl MoveRecord {
    /// Converts the move into SFEN formatted string.
    pub fn to_sfen(&self) -> String {
        match *self {
            MoveRecord::Normal {
                from, to, promoted, ..
            } => format!("{}{}{}", from, to, if promoted { "+" } else { "" }),
            MoveRecord::Drop {
                to,
                piece: Piece { piece_type, .. },
            } => format!("{}*{}", piece_type.to_string().to_uppercase(), to),
        }
    }
}

impl PartialEq<Move> for MoveRecord {
    fn eq(&self, other: &Move) -> bool {
        match (self, other) {
            (
                &MoveRecord::Normal {
                    from: f1,
                    to: t1,
                    promoted,
                    ..
                },
                &Move::Normal {
                    from: f2,
                    to: t2,
                    promote,
                },
            ) => f1 == f2 && t1 == t2 && promote == promoted,
            (&MoveRecord::Drop { to: t1, piece, .. }, &Move::Drop { to: t2, piece_type }) => {
                t1 == t2 && piece.piece_type == piece_type
            }
            _ => false,
        }
    }
}

struct PieceGrid([Option<Piece>; 81]);

impl PieceGrid {
    pub fn get(&self, sq: Square) -> &Option<Piece> {
        &self.0[sq.index()]
    }

    pub fn set(&mut self, sq: Square, pc: Option<Piece>) {
        self.0[sq.index()] = pc;
    }
}

impl fmt::Debug for PieceGrid {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "PieceGrid {{ ")?;

        for pc in self.0.iter() {
            write!(fmt, "{:?} ", pc)?;
        }
        write!(fmt, "}}")
    }
}

/// Represents a state of the game.
///
/// # Examples
///
/// ```
/// use shogi::{Move, Position};
/// use shogi::bitboard::Factory as BBFactory;
/// use shogi::square::consts::*;
///
/// BBFactory::init();
/// let mut pos = Position::new();
/// pos.set_sfen("lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1").unwrap();
///
/// let m = Move::Normal{from: SQ_7G, to: SQ_7F, promote: false};
/// pos.make_move(m).unwrap();
///
/// assert_eq!("lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1 moves 7g7f", pos.to_sfen());
/// ```
#[derive(Debug)]
pub struct Position {
    board: PieceGrid,
    hand: Hand,
    ply: u16,
    side_to_move: Color,
    move_history: Vec<MoveRecord>,
    sfen_history: Vec<(String, u16)>,
    occupied_bb: Bitboard,
    color_bb: [Bitboard; 2],
    type_bb: [Bitboard; 14],
}

/////////////////////////////////////////////////////////////////////////////
// Type implementation
/////////////////////////////////////////////////////////////////////////////

impl Position {
    /// Creates a new instance of `Position` with an empty board.
    pub fn new() -> Position {
        Default::default()
    }

    /////////////////////////////////////////////////////////////////////////
    // Accessors
    /////////////////////////////////////////////////////////////////////////

    /// Returns a piece at the given square.
    pub fn piece_at(&self, sq: Square) -> &Option<Piece> {
        self.board.get(sq)
    }

    /// Returns a bitboard containing pieces of the given player.
    pub fn player_bb(&self, c: Color) -> &Bitboard {
        &self.color_bb[c.index()]
    }

    /// Returns the number of the given piece in hand.
    pub fn hand(&self, p: Piece) -> u8 {
        self.hand.get(p)
    }

    /// Returns the side to make a move next.
    pub fn side_to_move(&self) -> Color {
        self.side_to_move
    }

    /// Returns the number of plies already completed by the current state.
    pub fn ply(&self) -> u16 {
        self.ply
    }

    /// Returns a history of all moves made since the beginning of the game.
    pub fn move_history(&self) -> &[MoveRecord] {
        &self.move_history
    }

    /// Checks if a player with the given color can declare winning.
    ///
    /// See the section 25 in http://www.computer-shogi.org/wcsc26/rule.pdf for more detail.
    pub fn try_declare_winning(&self, c: Color) -> bool {
        if c != self.side_to_move {
            return false;
        }

        let king_pos = self.find_king(c);
        if king_pos.is_none() {
            return false;
        }

        let king_pos = king_pos.unwrap();
        if king_pos.relative_rank(c) >= 3 {
            return false;
        }

        let (mut point, count) =
            PieceType::iter()
                .filter(|&pt| pt != PieceType::King)
                .fold((0, 0), |accum, pt| {
                    let unit = match pt {
                        PieceType::Rook
                        | PieceType::Bishop
                        | PieceType::ProRook
                        | PieceType::ProBishop => 5,
                        _ => 1,
                    };

                    let bb = &(&self.type_bb[pt.index()] & &self.color_bb[c.index()])
                        & &BBFactory::promote_zone(c);
                    let count = bb.count() as u8;
                    let point = count * unit;

                    (accum.0 + point, accum.1 + count)
                });

        if count < 10 {
            return false;
        }

        point += PieceType::iter()
            .filter(|pt| pt.is_hand_piece())
            .fold(0, |acc, pt| {
                let num = self.hand.get(Piece {
                    piece_type: pt,
                    color: c,
                });
                let pp = match pt {
                    PieceType::Rook | PieceType::Bishop => 5,
                    _ => 1,
                };

                acc + num * pp
            });

        let lowerbound = match c {
            Color::Black => 28,
            Color::White => 27,
        };
        if point < lowerbound {
            return false;
        }

        if self.in_check(c) {
            return false;
        }

        true
    }

    /// Checks if the king with the given color is in check.
    pub fn in_check(&self, c: Color) -> bool {
        if let Some(king_sq) = self.find_king(c) {
            self.is_attacked_by(king_sq, c.flip())
        } else {
            false
        }
    }

    /// Sets a piece at the given square.
    fn set_piece(&mut self, sq: Square, p: Option<Piece>) {
        self.board.set(sq, p);
    }

    fn is_attacked_by(&self, sq: Square, c: Color) -> bool {
        PieceType::iter().any(|pt| self.get_attackers_of_type(pt, sq, c).is_any())
    }

    fn get_attackers_of_type(&self, pt: PieceType, sq: Square, c: Color) -> Bitboard {
        let bb = &self.type_bb[pt.index()] & &self.color_bb[c.index()];

        if bb.is_empty() {
            return bb;
        }

        let attack_pc = Piece {
            piece_type: pt,
            color: c,
        };

        &bb & &self.move_candidates(sq, attack_pc.flip())
    }

    fn find_king(&self, c: Color) -> Option<Square> {
        let mut bb = &self.type_bb[PieceType::King.index()] & &self.color_bb[c.index()];
        if bb.is_any() {
            Some(bb.pop())
        } else {
            None
        }
    }

    fn log_position(&mut self) {
        // TODO: SFEN string is used to represent a state of position, but any transformation which uniquely distinguish positions can be used here.
        // Consider light-weight option if generating SFEN string for each move is time-consuming.
        let sfen = self.generate_sfen().split(' ').take(3).join(" ");
        let in_check = self.in_check(self.side_to_move());

        let continuous_check = if in_check {
            let past = if self.sfen_history.len() >= 2 {
                let record = self.sfen_history.get(self.sfen_history.len() - 2).unwrap();
                record.1
            } else {
                0
            };
            past + 1
        } else {
            0
        };

        self.sfen_history.push((sfen, continuous_check));
    }

    /////////////////////////////////////////////////////////////////////////
    // Making a move
    /////////////////////////////////////////////////////////////////////////

    /// Makes the given move. Returns `Err` if the move is invalid or any special condition is met.
    pub fn make_move(&mut self, m: Move) -> Result<(), MoveError> {
        let res = match m {
            Move::Normal { from, to, promote } => self.make_normal_move(from, to, promote)?,
            Move::Drop { to, piece_type } => self.make_drop_move(to, piece_type)?,
        };

        self.move_history.push(res);
        Ok(())
    }

    fn make_normal_move(
        &mut self,
        from: Square,
        to: Square,
        promoted: bool,
    ) -> Result<MoveRecord, MoveError> {
        let stm = self.side_to_move();
        let opponent = stm.flip();

        let moved = self
            .piece_at(from)
            .ok_or(MoveError::Inconsistent("No piece found"))?;

        let captured = *self.piece_at(to);

        if moved.color != stm {
            return Err(MoveError::Inconsistent(
                "The piece is not for the side to move",
            ));
        }

        if promoted && !from.in_promotion_zone(stm) && !to.in_promotion_zone(stm) {
            return Err(MoveError::Inconsistent("The piece cannot promote"));
        }

        if !self.move_candidates(from, moved).any(|sq| sq == to) {
            return Err(MoveError::Inconsistent("The piece cannot move to there"));
        }

        if !promoted && !moved.is_placeable_at(to) {
            return Err(MoveError::NonMovablePiece);
        }

        let placed = if promoted {
            match moved.promote() {
                Some(promoted) => promoted,
                None => return Err(MoveError::Inconsistent("This type of piece cannot promote")),
            }
        } else {
            moved
        };

        self.set_piece(from, None);
        self.set_piece(to, Some(placed));
        self.occupied_bb ^= from;
        self.occupied_bb ^= to;
        self.type_bb[moved.piece_type.index()] ^= from;
        self.type_bb[placed.piece_type.index()] ^= to;
        self.color_bb[moved.color.index()] ^= from;
        self.color_bb[placed.color.index()] ^= to;

        if let Some(ref cap) = captured {
            self.occupied_bb ^= to;
            self.type_bb[cap.piece_type.index()] ^= to;
            self.color_bb[cap.color.index()] ^= to;
            let pc = cap.flip();
            let pc = match pc.unpromote() {
                Some(unpromoted) => unpromoted,
                None => pc,
            };
            self.hand.increment(pc);
        }

        if self.in_check(stm) {
            // Undo-ing the move.
            self.set_piece(from, Some(moved));
            self.set_piece(to, captured);
            self.occupied_bb ^= from;
            self.occupied_bb ^= to;
            self.type_bb[moved.piece_type.index()] ^= from;
            self.type_bb[placed.piece_type.index()] ^= to;
            self.color_bb[moved.color.index()] ^= from;
            self.color_bb[placed.color.index()] ^= to;

            if let Some(ref cap) = captured {
                self.occupied_bb ^= to;
                self.type_bb[cap.piece_type.index()] ^= to;
                self.color_bb[cap.color.index()] ^= to;
                let pc = cap.flip();
                let pc = match pc.unpromote() {
                    Some(unpromoted) => unpromoted,
                    None => pc,
                };
                self.hand.decrement(pc);
            }

            return Err(MoveError::InCheck);
        }

        self.side_to_move = opponent;
        self.ply += 1;

        self.log_position();
        self.detect_repetition()?;

        Ok(MoveRecord::Normal {
            from,
            to,
            placed,
            captured,
            promoted,
        })
    }

    fn make_drop_move(&mut self, to: Square, pt: PieceType) -> Result<MoveRecord, MoveError> {
        let stm = self.side_to_move();
        let opponent = stm.flip();

        if self.piece_at(to).is_some() {
            return Err(MoveError::Inconsistent("There is already a piece in `to`"));
        }

        let pc = Piece {
            piece_type: pt,
            color: stm,
        };

        if self.hand(pc) == 0 {
            return Err(MoveError::Inconsistent("The piece is not in the hand"));
        }

        if !pc.is_placeable_at(to) {
            return Err(MoveError::NonMovablePiece);
        }

        if pc.piece_type == PieceType::Pawn {
            // Nifu check.
            for i in 0..9 {
                if let Some(fp) = *self.piece_at(Square::new(to.file(), i).unwrap()) {
                    if fp == pc {
                        return Err(MoveError::Nifu);
                    }
                }
            }

            // Uchifuzume check.
            if let Some(king_sq) = to.shift(0, if stm == Color::Black { -1 } else { 1 }) {
                // Is the dropped pawn attacking the opponent's king?
                if let Some(
                    pc
                    @
                    Piece {
                        piece_type: PieceType::King,
                        ..
                    },
                ) = *self.piece_at(king_sq)
                {
                    if pc.color == opponent {
                        // can any opponent's piece attack the dropped pawn?
                        let pinned = self.pinned_bb(opponent);

                        let not_attacked = PieceType::iter()
                            .filter(|&pt| pt != PieceType::King)
                            .flat_map(|pt| self.get_attackers_of_type(pt, to, opponent))
                            .all(|sq| (&pinned & sq).is_any());

                        if not_attacked {
                            // can the opponent's king evade?
                            let is_attacked = |sq| {
                                if let Some(pc) = *self.piece_at(sq) {
                                    if pc.color == opponent {
                                        return true;
                                    }
                                }

                                self.is_attacked_by(sq, stm)
                            };

                            if self.move_candidates(king_sq, pc).all(is_attacked) {
                                return Err(MoveError::Uchifuzume);
                            }
                        }
                    }
                }
            }
        }

        self.set_piece(to, Some(pc));
        self.occupied_bb ^= to;
        self.type_bb[pc.piece_type.index()] ^= to;
        self.color_bb[pc.color.index()] ^= to;

        if self.in_check(stm) {
            // Undo-ing the move.
            self.set_piece(to, None);
            self.occupied_bb ^= to;
            self.type_bb[pc.piece_type.index()] ^= to;
            self.color_bb[pc.color.index()] ^= to;
            return Err(MoveError::InCheck);
        }

        self.hand.decrement(pc);
        self.side_to_move = opponent;
        self.ply += 1;

        self.log_position();
        self.detect_repetition()?;

        Ok(MoveRecord::Drop { to, piece: pc })
    }

    /// Returns a list of squares at which a piece of the given color is pinned.
    pub fn pinned_bb(&self, c: Color) -> Bitboard {
        let ksq = self.find_king(c);
        if ksq.is_none() {
            return Bitboard::empty();
        }
        let ksq = ksq.unwrap();

        [
            (
                PieceType::Rook,
                BBFactory::rook_attack(ksq, &Bitboard::empty()),
            ),
            (
                PieceType::ProRook,
                BBFactory::rook_attack(ksq, &Bitboard::empty()),
            ),
            (
                PieceType::Bishop,
                BBFactory::bishop_attack(ksq, &Bitboard::empty()),
            ),
            (
                PieceType::ProBishop,
                BBFactory::bishop_attack(ksq, &Bitboard::empty()),
            ),
            (
                PieceType::Lance,
                BBFactory::lance_attack(c, ksq, &Bitboard::empty()),
            ),
        ]
        .iter()
        .fold(Bitboard::empty(), |mut accum, &(pt, ref mask)| {
            let bb = &(&self.type_bb[pt.index()] & &self.color_bb[c.flip().index()]) & mask;

            for psq in bb {
                let between = &BBFactory::between(ksq, psq) & &self.occupied_bb;
                if between.count() == 1 && (&between & &self.color_bb[c.index()]).is_any() {
                    accum |= &between;
                }
            }

            accum
        })
    }

    /// Undoes the last move.
    pub fn unmake_move(&mut self) -> Result<(), MoveError> {
        if self.move_history.is_empty() {
            // TODO: error?
            return Ok(());
        }

        let last = self.move_history.pop().unwrap();
        match last {
            MoveRecord::Normal {
                from,
                to,
                ref placed,
                ref captured,
                promoted,
            } => {
                if *self.piece_at(from) != None {
                    return Err(MoveError::Inconsistent(
                        "`from` of the move is filled by another piece",
                    ));
                }

                let moved = if promoted {
                    match placed.unpromote() {
                        Some(unpromoted) => unpromoted,
                        None => return Err(MoveError::Inconsistent("Cannot unpromoted the piece")),
                    }
                } else {
                    *placed
                };
                if *self.piece_at(to) != Some(*placed) {
                    return Err(MoveError::Inconsistent(
                        "Expected piece is not found in `to`",
                    ));
                }

                self.set_piece(from, Some(moved));
                self.set_piece(to, *captured);
                self.occupied_bb ^= from;
                self.occupied_bb ^= to;
                self.type_bb[moved.piece_type.index()] ^= from;
                self.type_bb[placed.piece_type.index()] ^= to;
                self.color_bb[moved.color.index()] ^= from;
                self.color_bb[placed.color.index()] ^= to;

                if let Some(ref cap) = *captured {
                    self.occupied_bb ^= to;
                    self.type_bb[cap.piece_type.index()] ^= to;
                    self.color_bb[cap.color.index()] ^= to;
                    let unpromoted_cap = cap.unpromote().unwrap_or(*cap);
                    self.hand.decrement(unpromoted_cap.flip());
                }
            }
            MoveRecord::Drop { to, piece } => {
                if *self.piece_at(to) != Some(piece) {
                    return Err(MoveError::Inconsistent(
                        "Expected piece is not found in `to`",
                    ));
                }

                self.set_piece(to, None);
                self.occupied_bb ^= to;
                self.type_bb[piece.piece_type.index()] ^= to;
                self.color_bb[piece.color.index()] ^= to;
                self.hand.increment(piece);
            }
        };

        self.side_to_move = self.side_to_move.flip();
        self.ply -= 1;
        self.sfen_history.pop();

        Ok(())
    }

    /// Returns a list of squares to where the given pieve at the given square can move.
    pub fn move_candidates(&self, sq: Square, p: Piece) -> Bitboard {
        let bb = match p.piece_type {
            PieceType::Rook => BBFactory::rook_attack(sq, &self.occupied_bb),
            PieceType::Bishop => BBFactory::bishop_attack(sq, &self.occupied_bb),
            PieceType::Lance => BBFactory::lance_attack(p.color, sq, &self.occupied_bb),
            PieceType::ProRook => {
                &BBFactory::rook_attack(sq, &self.occupied_bb)
                    | &BBFactory::attacks_from(PieceType::King, p.color, sq)
            }
            PieceType::ProBishop => {
                &BBFactory::bishop_attack(sq, &self.occupied_bb)
                    | &BBFactory::attacks_from(PieceType::King, p.color, sq)
            }
            PieceType::ProSilver
            | PieceType::ProKnight
            | PieceType::ProLance
            | PieceType::ProPawn => BBFactory::attacks_from(PieceType::Gold, p.color, sq),
            pt => BBFactory::attacks_from(pt, p.color, sq),
        };

        &bb & &!&self.color_bb[p.color.index()]
    }

    fn detect_repetition(&self) -> Result<(), MoveError> {
        if self.sfen_history.len() < 9 {
            return Ok(());
        }

        let cur = self.sfen_history.last().unwrap();

        let mut cnt = 0;
        for (i, entry) in self.sfen_history.iter().rev().enumerate() {
            if entry.0 == cur.0 {
                cnt += 1;

                if cnt == 4 {
                    let prev = self.sfen_history.get(self.sfen_history.len() - 2).unwrap();

                    if cur.1 * 2 >= (i as u16) {
                        return Err(MoveError::PerpetualCheckLose);
                    } else if prev.1 * 2 >= (i as u16) {
                        return Err(MoveError::PerpetualCheckWin);
                    } else {
                        return Err(MoveError::Repetition);
                    }
                }
            }
        }

        Ok(())
    }

    /////////////////////////////////////////////////////////////////////////
    // SFEN serialization / deserialization
    /////////////////////////////////////////////////////////////////////////

    /// Parses the given SFEN string and updates the game state.
    pub fn set_sfen(&mut self, sfen_str: &str) -> Result<(), SfenError> {
        let mut parts = sfen_str.split_whitespace();

        // Build the initial position, all parts are required.
        parts
            .next()
            .ok_or(SfenError::MissingDataFields)
            .and_then(|s| self.parse_sfen_board(s))?;
        parts
            .next()
            .ok_or(SfenError::MissingDataFields)
            .and_then(|s| self.parse_sfen_stm(s))?;
        parts
            .next()
            .ok_or(SfenError::MissingDataFields)
            .and_then(|s| self.parse_sfen_hand(s))?;
        parts
            .next()
            .ok_or(SfenError::MissingDataFields)
            .and_then(|s| self.parse_sfen_ply(s))?;

        self.sfen_history.clear();
        self.log_position();

        // Make moves following the initial position, optional.
        if let Some("moves") = parts.next() {
            for m in parts {
                if let Some(m) = Move::from_sfen(m) {
                    // Stop if any error occurrs.
                    match self.make_move(m) {
                        Ok(_) => {
                            self.log_position();
                        }
                        Err(_) => break,
                    }
                } else {
                    return Err(SfenError::IllegalMove);
                }
            }
        }

        Ok(())
    }

    /// Converts the current state into SFEN formatted string.
    pub fn to_sfen(&self) -> String {
        if self.sfen_history.is_empty() {
            return self.generate_sfen();
        }

        if self.move_history.is_empty() {
            return format!("{} {}", self.sfen_history.first().unwrap().0, self.ply);
        }

        let mut sfen = format!(
            "{} {} moves",
            &self.sfen_history.first().unwrap().0,
            self.ply - self.move_history.len() as u16
        );

        for m in self.move_history.iter() {
            sfen.push_str(&format!(" {}", &m.to_sfen()));
        }

        sfen
    }

    fn parse_sfen_board(&mut self, s: &str) -> Result<(), SfenError> {
        let rows = s.split('/');

        self.occupied_bb = Bitboard::empty();
        self.color_bb = Default::default();
        self.type_bb = Default::default();

        for (i, row) in rows.enumerate() {
            if i >= 9 {
                return Err(SfenError::IllegalBoardState);
            }

            let mut j = 0;

            let mut is_promoted = false;
            for c in row.chars() {
                match c {
                    '+' => {
                        is_promoted = true;
                    }
                    n if n.is_digit(10) => {
                        if let Some(n) = n.to_digit(10) {
                            for _ in 0..n {
                                if j >= 9 {
                                    return Err(SfenError::IllegalBoardState);
                                }

                                let sq = Square::new(8 - j, i as u8).unwrap();
                                self.set_piece(sq, None);

                                j += 1;
                            }
                        }
                    }
                    s => match Piece::from_sfen(s) {
                        Some(mut piece) => {
                            if j >= 9 {
                                return Err(SfenError::IllegalBoardState);
                            }

                            if is_promoted {
                                if let Some(promoted) = piece.piece_type.promote() {
                                    piece.piece_type = promoted;
                                } else {
                                    return Err(SfenError::IllegalPieceType);
                                }
                            }

                            let sq = Square::new(8 - j, i as u8).unwrap();
                            self.set_piece(sq, Some(piece));
                            self.occupied_bb |= sq;
                            self.color_bb[piece.color.index()] |= sq;
                            self.type_bb[piece.piece_type.index()] |= sq;
                            j += 1;

                            is_promoted = false;
                        }
                        None => return Err(SfenError::IllegalPieceType),
                    },
                }
            }
        }

        Ok(())
    }

    fn parse_sfen_stm(&mut self, s: &str) -> Result<(), SfenError> {
        self.side_to_move = match s {
            "b" => Color::Black,
            "w" => Color::White,
            _ => return Err(SfenError::IllegalSideToMove),
        };
        Ok(())
    }

    fn parse_sfen_hand(&mut self, s: &str) -> Result<(), SfenError> {
        if s == "-" {
            self.hand.clear();
            return Ok(());
        }

        let mut num_pieces: u8 = 0;
        for c in s.chars() {
            match c {
                n if n.is_digit(10) => {
                    if let Some(n) = n.to_digit(10) {
                        num_pieces = num_pieces * 10 + (n as u8);
                    }
                }
                s => {
                    match Piece::from_sfen(s) {
                        Some(p) => self
                            .hand
                            .set(p, if num_pieces == 0 { 1 } else { num_pieces }),
                        None => return Err(SfenError::IllegalPieceType),
                    };
                    num_pieces = 0;
                }
            }
        }

        Ok(())
    }

    fn parse_sfen_ply(&mut self, s: &str) -> Result<(), SfenError> {
        self.ply = s.parse()?;
        Ok(())
    }

    fn generate_sfen(&self) -> String {
        let board = (0..9)
            .map(|row| {
                let mut s = String::new();
                let mut num_spaces = 0;
                for file in (0..9).rev() {
                    match *self.piece_at(Square::new(file, row).unwrap()) {
                        Some(pc) => {
                            if num_spaces > 0 {
                                s.push_str(&num_spaces.to_string());
                                num_spaces = 0;
                            }

                            s.push_str(&pc.to_string());
                        }
                        None => num_spaces += 1,
                    }
                }

                if num_spaces > 0 {
                    s.push_str(&num_spaces.to_string());
                }

                s
            })
            .join("/");

        let color = if self.side_to_move == Color::Black {
            "b"
        } else {
            "w"
        };

        let mut hand = [Color::Black, Color::White]
            .iter()
            .map(|c| {
                PieceType::iter()
                    .filter(|pt| pt.is_hand_piece())
                    .map(|pt| {
                        let pc = Piece {
                            piece_type: pt,
                            color: *c,
                        };
                        let n = self.hand.get(pc);

                        if n == 0 {
                            "".to_string()
                        } else if n == 1 {
                            format!("{}", pc)
                        } else {
                            format!("{}{}", n, pc)
                        }
                    })
                    .join("")
            })
            .join("");

        if hand.is_empty() {
            hand = "-".to_string();
        }

        format!("{} {} {} {}", board, color, hand, self.ply)
    }
}

/////////////////////////////////////////////////////////////////////////////
// Trait implementations
/////////////////////////////////////////////////////////////////////////////

impl Default for Position {
    fn default() -> Position {
        Position {
            side_to_move: Color::Black,
            board: PieceGrid([None; 81]),
            hand: Default::default(),
            ply: 1,
            move_history: Default::default(),
            sfen_history: Default::default(),
            occupied_bb: Default::default(),
            color_bb: Default::default(),
            type_bb: Default::default(),
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "   9   8   7   6   5   4   3   2   1")?;
        writeln!(f, "+---+---+---+---+---+---+---+---+---+")?;

        for row in 0..9 {
            write!(f, "|")?;
            for file in (0..9).rev() {
                if let Some(ref piece) = *self.piece_at(Square::new(file, row).unwrap()) {
                    write!(f, "{:>3}|", piece.to_string())?;
                } else {
                    write!(f, "   |")?;
                }
            }

            writeln!(f, " {}", (('a' as usize + row as usize) as u8) as char)?;
            writeln!(f, "+---+---+---+---+---+---+---+---+---+")?;
        }

        writeln!(
            f,
            "Side to move: {}",
            if self.side_to_move == Color::Black {
                "Black"
            } else {
                "White"
            }
        )?;

        let fmt_hand = |color: Color, f: &mut fmt::Formatter| -> fmt::Result {
            for pt in PieceType::iter().filter(|pt| pt.is_hand_piece()) {
                let pc = Piece {
                    piece_type: pt,
                    color,
                };
                let n = self.hand.get(pc);

                if n > 0 {
                    write!(f, "{}{} ", pc, n)?;
                }
            }
            Ok(())
        };
        write!(f, "Hand (Black): ")?;
        fmt_hand(Color::Black, f)?;
        writeln!(f)?;

        write!(f, "Hand (White): ")?;
        fmt_hand(Color::White, f)?;
        writeln!(f)?;

        write!(f, "Ply: {}", self.ply)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::square::consts::*;

    fn setup() {
        BBFactory::init();
    }

    #[test]
    fn new() {
        setup();

        let pos = Position::new();

        for i in 0..9 {
            for j in 0..9 {
                let sq = Square::new(i, j).unwrap();
                assert_eq!(None, *pos.piece_at(sq));
            }
        }
    }

    #[test]
    fn in_check() {
        setup();

        let test_cases = [
            (
                "lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1",
                false,
                false,
            ),
            ("9/3r5/9/9/6B2/9/9/9/3K5 b P 1", true, false),
            (
                "ln2r1knl/2gb1+Rg2/4Pp1p1/p1pp1sp1p/1N2pN1P1/2P2PP2/PP1G1S2R/1SG6/LK6L w 2PSp 1",
                false,
                true,
            ),
            (
                "lnsg1gsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSG1GSNL b - 1",
                false,
                false,
            ),
        ];

        let mut pos = Position::new();
        for case in test_cases.iter() {
            pos.set_sfen(case.0).expect("failed to parse SFEN string");
            assert_eq!(case.1, pos.in_check(Color::Black));
            assert_eq!(case.2, pos.in_check(Color::White));
        }
    }

    #[test]
    fn player_bb() {
        setup();

        let cases: &[(&str, &[Square], &[Square])] = &[
            (
                "R6gk/9/8p/9/4p4/9/9/8L/B8 b - 1",
                &[SQ_9A, SQ_1H, SQ_9I],
                &[SQ_2A, SQ_1A, SQ_1C, SQ_5E],
            ),
            ("9/3r5/9/9/6B2/9/9/9/3K5 b P 1", &[SQ_3E, SQ_6I], &[SQ_6B]),
        ];

        let mut pos = Position::new();
        for case in cases {
            pos.set_sfen(case.0).expect("faled to parse SFEN string");
            let black = pos.player_bb(Color::Black);
            let white = pos.player_bb(Color::White);

            assert_eq!(case.1.len(), black.count() as usize);
            for sq in case.1 {
                assert!((black & *sq).is_any());
            }

            assert_eq!(case.2.len(), white.count() as usize);
            for sq in case.2 {
                assert!((white & *sq).is_any());
            }
        }
    }

    #[test]
    fn pinned_bb() {
        setup();

        let cases: &[(&str, &[Square], &[Square])] = &[(
            "R6gk/9/8p/9/4p4/9/9/8L/B8 b - 1",
            &[],
            &[SQ_2A, SQ_1C, SQ_5E],
        )];

        let mut pos = Position::new();
        for case in cases {
            pos.set_sfen(case.0).expect("faled to parse SFEN string");
            let black = pos.pinned_bb(Color::Black);
            let white = pos.pinned_bb(Color::White);

            assert_eq!(case.1.len(), black.count());
            for sq in case.1 {
                assert!((&black & *sq).is_any());
            }

            assert_eq!(case.2.len(), white.count());
            for sq in case.2 {
                assert!((&white & *sq).is_any());
            }
        }
    }

    #[test]
    fn move_candidates() {
        setup();

        let mut pos = Position::new();
        pos.set_sfen("lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1")
            .expect("failed to parse SFEN string");

        let mut sum = 0;
        for sq in Square::iter() {
            let pc = pos.piece_at(sq);

            if let Some(pc) = *pc {
                if pc.color == pos.side_to_move() {
                    sum += pos.move_candidates(sq, pc).count();
                }
            }
        }

        assert_eq!(30, sum);
    }

    #[test]
    fn make_normal_move() {
        setup();

        let base_sfen = "l6nl/5+P1gk/2np1S3/p1p4Pp/3P2Sp1/1PPb2P1P/P5GS1/R8/LN4bKL w GR5pnsg 1";
        let test_cases = [
            (SQ_2B, SQ_2C, false, true),
            (SQ_7C, SQ_6E, false, true),
            (SQ_3I, SQ_4H, true, true),
            (SQ_6F, SQ_9I, true, true),
            (SQ_2B, SQ_2C, false, true),
            (SQ_9C, SQ_9D, false, false),
            (SQ_9B, SQ_8B, false, false),
            (SQ_9B, SQ_9D, false, false),
            (SQ_2B, SQ_2C, true, false),
        ];

        let mut pos = Position::new();
        for case in test_cases.iter() {
            pos.set_sfen(base_sfen)
                .expect("failed to parse SFEN string");
            assert_eq!(case.3, pos.make_normal_move(case.0, case.1, case.2).is_ok());
        }

        // Leaving the checked king is illegal.
        pos.set_sfen("9/3r5/9/9/6B2/9/9/9/3K5 b P 1")
            .expect("failed to parse SFEN string");
        assert!(pos.make_normal_move(SQ_6I, SQ_6H, false).is_err());
        pos.set_sfen("9/3r5/9/9/6B2/9/9/9/3K5 b P 1")
            .expect("failed to parse SFEN string");
        assert!(pos.make_normal_move(SQ_6I, SQ_7I, false).is_ok());
    }

    #[test]
    fn make_drop_move() {
        setup();

        let base_sfen = "l6nl/5+P1gk/2np1S3/p1p4Pp/3P2Sp1/1PPb2P1P/P5GS1/R8/LN4bKL w GR5pnsg 1";
        let test_cases = [
            (SQ_5E, PieceType::Pawn, true),
            (SQ_5E, PieceType::Rook, false),
            (SQ_9A, PieceType::Pawn, false),
            (SQ_6F, PieceType::Pawn, false),
            (SQ_9B, PieceType::Pawn, false),
            (SQ_5I, PieceType::Pawn, false),
        ];

        let mut pos = Position::new();
        for case in test_cases.iter() {
            pos.set_sfen(base_sfen)
                .expect("failed to parse SFEN string");
            assert_eq!(
                case.2,
                pos.make_move(Move::Drop {
                    to: case.0,
                    piece_type: case.1,
                })
                .is_ok()
            );
        }
    }

    #[test]
    fn nifu() {
        setup();

        let ng_cases = [(
            "ln1g5/1ks1g3l/1p2p1n2/p1pGs2rp/1P1N1ppp1/P1SB1P2P/1S1p1bPP1/LKG6/4R2NL \
             w 2Pp 91",
            SQ_6C,
        )];
        let ok_cases = [(
            "ln1g5/1ks1g3l/1p2p1n2/p1pGs2rp/1P1N1ppp1/P1SB1P2P/1S1+p1bPP1/LKG6/4R2NL \
             w 2Pp 91",
            SQ_6C,
        )];

        let mut pos = Position::new();
        for (i, case) in ng_cases.iter().enumerate() {
            pos.set_sfen(case.0).expect("failed to parse SFEN string");
            assert_eq!(
                Some(MoveError::Nifu),
                pos.make_move(Move::Drop {
                    to: case.1,
                    piece_type: PieceType::Pawn,
                })
                .err(),
                "failed at #{}",
                i
            );
        }

        for (i, case) in ok_cases.iter().enumerate() {
            pos.set_sfen(case.0).expect("failed to parse SFEN string");
            assert!(
                pos.make_move(Move::Drop {
                    to: case.1,
                    piece_type: PieceType::Pawn,
                })
                .is_ok(),
                "failed at #{}",
                i
            );
        }
    }

    #[test]
    fn uchifuzume() {
        setup();

        let ng_cases = [
            ("9/9/7sp/6ppk/9/7G1/9/9/9 b P 1", SQ_1E),
            ("7nk/9/7S1/6b2/9/9/9/9/9 b P 1", SQ_1B),
            ("7nk/7g1/6BS1/9/9/9/9/9/9 b P 1", SQ_1B),
            ("R6gk/9/7S1/9/9/9/9/9/9 b P 1", SQ_1B),
        ];
        let ok_cases = [
            ("9/9/7pp/6psk/9/7G1/7N1/9/9 b P 1", SQ_1E),
            ("7nk/9/7Sg/6b2/9/9/9/9/9 b P 1", SQ_1B),
            (
                "9/8p/3pG1gp1/2p2kl1N/3P1p1s1/lPP6/2SGBP3/PK1S2+p2/LN7 w RSL3Prbg2n4p 1",
                SQ_8G,
            ),
        ];

        let mut pos = Position::new();
        for (i, case) in ng_cases.iter().enumerate() {
            pos.set_sfen(case.0).expect("failed to parse SFEN string");
            assert_eq!(
                Some(MoveError::Uchifuzume),
                pos.make_move(Move::Drop {
                    to: case.1,
                    piece_type: PieceType::Pawn,
                })
                .err(),
                "failed at #{}",
                i
            );
        }

        for (i, case) in ok_cases.iter().enumerate() {
            pos.set_sfen(case.0).expect("failed to parse SFEN string");
            assert!(
                pos.make_move(Move::Drop {
                    to: case.1,
                    piece_type: PieceType::Pawn,
                })
                .is_ok(),
                "failed at #{}",
                i
            );
        }
    }

    #[test]
    fn repetition() {
        setup();

        let mut pos = Position::new();
        pos.set_sfen("ln7/ks+R6/pp7/9/9/9/9/9/9 b Ss 1")
            .expect("failed to parse SFEN string");

        for _ in 0..2 {
            assert!(pos.make_drop_move(SQ_7A, PieceType::Silver).is_ok());
            assert!(pos.make_drop_move(SQ_7C, PieceType::Silver).is_ok());
            assert!(pos.make_normal_move(SQ_7A, SQ_8B, true).is_ok());
            assert!(pos.make_normal_move(SQ_7C, SQ_8B, false).is_ok());
        }

        assert!(pos.make_drop_move(SQ_7A, PieceType::Silver).is_ok());
        assert!(pos.make_drop_move(SQ_7C, PieceType::Silver).is_ok());
        assert!(pos.make_normal_move(SQ_7A, SQ_8B, true).is_ok());
        assert_eq!(
            Some(MoveError::Repetition),
            pos.make_normal_move(SQ_7C, SQ_8B, false).err()
        );
    }

    #[test]
    fn percetual_check() {
        setup();

        // Case 1. Starting from a check move.
        let mut pos = Position::new();
        pos.set_sfen("8l/6+P2/6+Rpk/8p/9/7S1/9/9/9 b - 1")
            .expect("failed to parse SFEN string");

        for _ in 0..2 {
            assert!(pos.make_normal_move(SQ_3C, SQ_2B, false).is_ok());
            assert!(pos.make_normal_move(SQ_1C, SQ_2D, false).is_ok());
            assert!(pos.make_normal_move(SQ_2B, SQ_3C, false).is_ok());
            assert!(pos.make_normal_move(SQ_2D, SQ_1C, false).is_ok());
        }
        assert!(pos.make_normal_move(SQ_3C, SQ_2B, false).is_ok());
        assert!(pos.make_normal_move(SQ_1C, SQ_2D, false).is_ok());
        assert!(pos.make_normal_move(SQ_2B, SQ_3C, false).is_ok());
        assert_eq!(
            Some(MoveError::PerpetualCheckWin),
            pos.make_normal_move(SQ_2D, SQ_1C, false).err()
        );

        // Case 2. Starting from an escape move.
        pos.set_sfen("6p1k/9/8+R/9/9/9/9/9/9 w - 1")
            .expect("failed to parse SFEN string");

        for _ in 0..2 {
            assert!(pos.make_normal_move(SQ_1A, SQ_2A, false).is_ok());
            assert!(pos.make_normal_move(SQ_1C, SQ_2C, false).is_ok());
            assert!(pos.make_normal_move(SQ_2A, SQ_1A, false).is_ok());
            assert!(pos.make_normal_move(SQ_2C, SQ_1C, false).is_ok());
        }
        assert!(pos.make_normal_move(SQ_1A, SQ_2A, false).is_ok());
        assert!(pos.make_normal_move(SQ_1C, SQ_2C, false).is_ok());
        assert!(pos.make_normal_move(SQ_2A, SQ_1A, false).is_ok());
        assert_eq!(
            Some(MoveError::PerpetualCheckLose),
            pos.make_normal_move(SQ_2C, SQ_1C, false).err()
        );
    }

    #[test]
    fn unmake_move() {
        setup();

        let mut pos = Position::new();
        let base_sfen = "l6nl/4+p+P1gk/2n2S3/p1p4Pp/3P2Sp1/1PPb2P1P/4+P1GS1/R8/LN4bKL w RG5gsnp 1";
        pos.set_sfen(base_sfen)
            .expect("failed to parse SFEN string");
        let base_state = format!("{}", pos);
        println!("{}", base_state);
        let test_cases = [
            Move::Drop {
                to: SQ_5E,
                piece_type: PieceType::Pawn,
            },
            // No capture by unpromoted piece
            Move::Normal {
                from: SQ_6F,
                to: SQ_7G,
                promote: false,
            },
            // No capture by promoting piece
            Move::Normal {
                from: SQ_6F,
                to: SQ_7G,
                promote: true,
            },
            // No capture by promoted piece
            Move::Normal {
                from: SQ_5B,
                to: SQ_5A,
                promote: false,
            },
            // Capture of unpromoted piece by unpromoted piece
            Move::Normal {
                from: SQ_6F,
                to: SQ_9I,
                promote: false,
            },
            // Capture of unpromoted piece by promoting piece
            Move::Normal {
                from: SQ_6F,
                to: SQ_9I,
                promote: true,
            },
            // Capture of unpromoted piece by promoted piece
            Move::Normal {
                from: SQ_5B,
                to: SQ_4C,
                promote: false,
            },
            // Capture of promoted piece by unpromoted piece
            Move::Normal {
                from: SQ_6F,
                to: SQ_5G,
                promote: false,
            },
            // Capture of promoted piece by promoting piece
            Move::Normal {
                from: SQ_6F,
                to: SQ_5G,
                promote: true,
            },
            // Capture of promoted piece by promoted piece
            Move::Normal {
                from: SQ_5B,
                to: SQ_4B,
                promote: false,
            },
        ];

        for case in test_cases.iter() {
            pos.set_sfen(base_sfen)
                .expect("failed to parse SFEN string");
            pos.make_move(*case)
                .unwrap_or_else(|_| panic!("failed to make a move: {}", case));
            pos.unmake_move()
                .unwrap_or_else(|_| panic!("failed to unmake a move: {}", case));
            assert_eq!(
                base_sfen,
                pos.to_sfen(),
                "{}",
                format!("sfen unmatch for {}", case).as_str()
            );
            assert_eq!(
                base_state,
                format!("{}", pos),
                "{}",
                format!("state unmatch for {}", case).as_str()
            );
        }
    }

    #[test]
    fn try_declare_winning() {
        setup();

        let mut pos = Position::new();

        pos.set_sfen("lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1")
            .expect("failed to parse SFEN string");
        assert!(!pos.try_declare_winning(Color::Black));
        assert!(!pos.try_declare_winning(Color::White));

        pos.set_sfen("1K7/+NG+N+NGG3/P+S+P+P+PS3/9/7s1/9/+b+rppp+p+s1+p/3+p1+bk2/9 b R4L7Pgnp 1")
            .expect("failed to parse SFEN string");
        assert!(pos.try_declare_winning(Color::Black));
        assert!(!pos.try_declare_winning(Color::White));

        pos.set_sfen(
            "1K6l/1+N7/+PG2+Ns1p1/2+N5p/6p2/3+b4P/4+p+p+bs1/+r1s4+lk/1g1g3+r1 w \
             Gns2l11p 1",
        )
        .expect("failed to parse SFEN string");
        assert!(!pos.try_declare_winning(Color::Black));
        assert!(pos.try_declare_winning(Color::White));

        pos.set_sfen(
            "1K6l/1+N7/+PG2+Ns1p1/2+N5p/6p2/3+b4P/4+p+p+bs1/+r1s4+lk/1g1g3+r1 b \
             Gns2l11p 1",
        )
        .expect("failed to parse SFEN string");
        assert!(!pos.try_declare_winning(Color::Black));
        assert!(!pos.try_declare_winning(Color::White));

        pos.set_sfen(
            "1K6l/1+N7/+PG2+Ns1p1/2+N5p/6p2/3+b4P/4+p+p+bs1/+r1s4+l1/1g1g3+r1 b \
             Gns2l11p 1",
        )
        .expect("failed to parse SFEN string");
        assert!(!pos.try_declare_winning(Color::Black));
        assert!(!pos.try_declare_winning(Color::White));

        pos.set_sfen(
            "1K6l/1+N7/+PG2+Ns1p1/2+N5p/6p2/1k1+b4P/4+p+p+bs1/+r1s4+l1/1g1g3+r1 b \
             Gns2l11p 1",
        )
        .expect("failed to parse SFEN string");
        assert!(!pos.try_declare_winning(Color::Black));
        assert!(!pos.try_declare_winning(Color::White));

        pos.set_sfen(
            "1K6l/1+N7/+PG2+Ns1p1/2+N5p/6p2/3+b4P/4+p+p+bs1/+r1s4+lk/1g1g3+rG w \
             ns2l11p 1",
        )
        .expect("failed to parse SFEN string");
        assert!(!pos.try_declare_winning(Color::Black));
        assert!(!pos.try_declare_winning(Color::White));

        pos.set_sfen("1K6l/1+N7/+PG2+Ns1p1/2+N5p/6p2/3+b4P/5+p+bs1/+r1s4+lk/1g1g3+rG w ns2l12p 1")
            .expect("failed to parse SFEN string");
        assert!(!pos.try_declare_winning(Color::Black));
        assert!(!pos.try_declare_winning(Color::White));
    }

    #[test]
    fn set_sfen_normal() {
        setup();

        let mut pos = Position::new();

        pos.set_sfen("lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1")
            .expect("failed to parse SFEN string");

        let filled_squares = [
            (0, 0, PieceType::Lance, Color::White),
            (1, 0, PieceType::Knight, Color::White),
            (2, 0, PieceType::Silver, Color::White),
            (3, 0, PieceType::Gold, Color::White),
            (4, 0, PieceType::King, Color::White),
            (5, 0, PieceType::Gold, Color::White),
            (6, 0, PieceType::Silver, Color::White),
            (7, 0, PieceType::Knight, Color::White),
            (8, 0, PieceType::Lance, Color::White),
            (7, 1, PieceType::Rook, Color::White),
            (1, 1, PieceType::Bishop, Color::White),
            (0, 2, PieceType::Pawn, Color::White),
            (1, 2, PieceType::Pawn, Color::White),
            (2, 2, PieceType::Pawn, Color::White),
            (3, 2, PieceType::Pawn, Color::White),
            (4, 2, PieceType::Pawn, Color::White),
            (5, 2, PieceType::Pawn, Color::White),
            (6, 2, PieceType::Pawn, Color::White),
            (7, 2, PieceType::Pawn, Color::White),
            (8, 2, PieceType::Pawn, Color::White),
            (0, 6, PieceType::Pawn, Color::Black),
            (1, 6, PieceType::Pawn, Color::Black),
            (2, 6, PieceType::Pawn, Color::Black),
            (3, 6, PieceType::Pawn, Color::Black),
            (4, 6, PieceType::Pawn, Color::Black),
            (5, 6, PieceType::Pawn, Color::Black),
            (6, 6, PieceType::Pawn, Color::Black),
            (7, 6, PieceType::Pawn, Color::Black),
            (8, 6, PieceType::Pawn, Color::Black),
            (7, 7, PieceType::Bishop, Color::Black),
            (1, 7, PieceType::Rook, Color::Black),
            (0, 8, PieceType::Lance, Color::Black),
            (1, 8, PieceType::Knight, Color::Black),
            (2, 8, PieceType::Silver, Color::Black),
            (3, 8, PieceType::Gold, Color::Black),
            (4, 8, PieceType::King, Color::Black),
            (5, 8, PieceType::Gold, Color::Black),
            (6, 8, PieceType::Silver, Color::Black),
            (7, 8, PieceType::Knight, Color::Black),
            (8, 8, PieceType::Lance, Color::Black),
        ];

        let empty_squares = [
            (0, 1, 1),
            (2, 1, 5),
            (8, 1, 1),
            (0, 3, 9),
            (0, 4, 9),
            (0, 5, 9),
            (0, 7, 1),
            (2, 7, 5),
            (8, 7, 1),
        ];

        let hand_pieces = [
            (PieceType::Pawn, 0),
            (PieceType::Lance, 0),
            (PieceType::Knight, 0),
            (PieceType::Silver, 0),
            (PieceType::Gold, 0),
            (PieceType::Rook, 0),
            (PieceType::Bishop, 0),
        ];

        for case in filled_squares.iter() {
            let (file, row, pt, c) = *case;
            assert_eq!(
                Some(Piece {
                    piece_type: pt,
                    color: c,
                }),
                *pos.piece_at(Square::new(file, row).unwrap())
            );
        }

        for case in empty_squares.iter() {
            let (file, row, len) = *case;
            for i in file..(file + len) {
                assert_eq!(None, *pos.piece_at(Square::new(i, row).unwrap()));
            }
        }

        for case in hand_pieces.iter() {
            let (pt, n) = *case;
            assert_eq!(
                n,
                pos.hand(Piece {
                    piece_type: pt,
                    color: Color::Black,
                })
            );
            assert_eq!(
                n,
                pos.hand(Piece {
                    piece_type: pt,
                    color: Color::White,
                })
            );
        }

        assert_eq!(Color::Black, pos.side_to_move());
        assert_eq!(1, pos.ply());
    }

    #[test]
    fn to_sfen() {
        setup();

        let test_cases = [
            "7k1/9/7P1/9/9/9/9/9/9 b G2r2b3g4s4n4l17p 1",
            "lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1",
            "lnsgk+Lpnl/1p5+B1/p1+Pps1ppp/9/9/9/P+r1PPpPPP/1R7/LNSGKGSN1 w BGP2p \
             1024",
        ];

        let mut pos = Position::new();
        for case in test_cases.iter() {
            pos.set_sfen(case).expect("failed to parse SFEN string");
            assert_eq!(*case, pos.to_sfen());
        }
    }

    #[test]
    fn set_sfen_custom() {
        setup();

        let mut pos = Position::new();
        pos.set_sfen("lnsgk+Lpnl/1p5+B1/p1+Pps1ppp/9/9/9/P+r1PPpPPP/1R7/LNSGKGSN1 w BGP2p 1024")
            .expect("failed to parse SFEN string");

        let filled_squares = [
            (8, 0, PieceType::Lance, Color::White),
            (7, 0, PieceType::Knight, Color::White),
            (6, 0, PieceType::Silver, Color::White),
            (5, 0, PieceType::Gold, Color::White),
            (4, 0, PieceType::King, Color::White),
            (3, 0, PieceType::ProLance, Color::Black),
            (2, 0, PieceType::Pawn, Color::White),
            (1, 0, PieceType::Knight, Color::White),
            (0, 0, PieceType::Lance, Color::White),
            (7, 1, PieceType::Pawn, Color::White),
            (1, 1, PieceType::ProBishop, Color::Black),
            (8, 2, PieceType::Pawn, Color::White),
            (6, 2, PieceType::ProPawn, Color::Black),
            (5, 2, PieceType::Pawn, Color::White),
            (4, 2, PieceType::Silver, Color::White),
            (2, 2, PieceType::Pawn, Color::White),
            (1, 2, PieceType::Pawn, Color::White),
            (0, 2, PieceType::Pawn, Color::White),
            (8, 6, PieceType::Pawn, Color::Black),
            (7, 6, PieceType::ProRook, Color::White),
            (5, 6, PieceType::Pawn, Color::Black),
            (4, 6, PieceType::Pawn, Color::Black),
            (3, 6, PieceType::Pawn, Color::White),
            (2, 6, PieceType::Pawn, Color::Black),
            (1, 6, PieceType::Pawn, Color::Black),
            (0, 6, PieceType::Pawn, Color::Black),
            (7, 7, PieceType::Rook, Color::Black),
            (8, 8, PieceType::Lance, Color::Black),
            (7, 8, PieceType::Knight, Color::Black),
            (6, 8, PieceType::Silver, Color::Black),
            (5, 8, PieceType::Gold, Color::Black),
            (4, 8, PieceType::King, Color::Black),
            (3, 8, PieceType::Gold, Color::Black),
            (2, 8, PieceType::Silver, Color::Black),
            (1, 8, PieceType::Knight, Color::Black),
        ];

        let empty_squares = [
            (0, 1, 1),
            (2, 1, 5),
            (8, 1, 1),
            (3, 2, 1),
            (7, 2, 1),
            (0, 3, 9),
            (0, 4, 9),
            (0, 5, 9),
            (6, 6, 1),
            (0, 7, 7),
            (8, 7, 1),
            (0, 8, 1),
        ];

        let hand_pieces = [
            (
                Piece {
                    piece_type: PieceType::Pawn,
                    color: Color::Black,
                },
                1,
            ),
            (
                Piece {
                    piece_type: PieceType::Gold,
                    color: Color::Black,
                },
                1,
            ),
            (
                Piece {
                    piece_type: PieceType::Bishop,
                    color: Color::Black,
                },
                1,
            ),
            (
                Piece {
                    piece_type: PieceType::Pawn,
                    color: Color::White,
                },
                2,
            ),
        ];

        for case in filled_squares.iter() {
            let (file, row, pt, c) = *case;
            assert_eq!(
                Some(Piece {
                    piece_type: pt,
                    color: c,
                }),
                *pos.piece_at(Square::new(file, row).unwrap())
            );
        }

        for case in empty_squares.iter() {
            let (file, row, len) = *case;
            for i in file..(file + len) {
                assert_eq!(None, *pos.piece_at(Square::new(i, row).unwrap()));
            }
        }

        for case in hand_pieces.iter() {
            let (p, n) = *case;
            assert_eq!(n, pos.hand(p));
        }

        assert_eq!(Color::White, pos.side_to_move());
        assert_eq!(1024, pos.ply());
    }
}
