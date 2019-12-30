use super::*;
use bitintr::*;

macro_rules! BitboardOr {
    ($lhs: expr, $rhs: expr) => {
        Bitboard {
            p: [$lhs.p[0] | $rhs.p[0], $lhs.p[1] | $rhs.p[1]],
        }
    };
}

/// Creates various bitboard instances.
///
/// `init` method needs to be called first for pre-calculation.
///
/// # Examples
///
/// ```
/// use shogi::bitboard::Factory;
/// use shogi::square::consts::*;
///
/// // init() shold be called before other method calls.
/// Factory::init();
/// let bb = Factory::between(SQ_1A, SQ_9I);
/// assert_eq!(7, bb.count());
/// ```
pub struct Factory {}

impl Factory {
    /// Pre-calculate complex bitboards for faster table lookup.
    /// This method needs to be called once before other methods in `Factory` get called.
    pub fn init() {
        init_rook_block();
        init_rook_attack();
        init_bishop_block();
        init_bishop_attack();
        init_king_attack();
        init_gold_attack();
        init_silver_attack();
        init_pawn_attack();
        init_knight_attack();
        init_lance_attack();
        init_between();
    }

    /// Returns a bitboard in which squares attacked by the given piece are filled.
    #[inline(always)]
    pub fn attacks_from(pt: PieceType, c: Color, sq: Square) -> Bitboard {
        unsafe { ATTACK_BB[pt as usize][c as usize][sq.index()] }
    }

    /// Returns a bitboard in which squares attacked by Rook at the given square are filled.
    #[inline(always)]
    pub fn rook_attack(sq: Square, occupied: &Bitboard) -> Bitboard {
        unsafe {
            let mask = &ROOK_BLOCK_MASK[sq.index()];
            let index = occupied_to_index(&(occupied & mask), mask);

            ROOK_ATTACK_BB[ROOK_ATTACK_INDEX[sq.index()] + index]
        }
    }

    /// Returns a bitboard in which squares attacked by Bishop at the given square are filled.
    #[inline(always)]
    pub fn bishop_attack(sq: Square, occupied: &Bitboard) -> Bitboard {
        unsafe {
            let mask = &BISHOP_BLOCK_MASK[sq.index()];
            let index = occupied_to_index(&(occupied & mask), mask);

            BISHOP_ATTACK_BB[BISHOP_ATTACK_INDEX[sq.index()] + index]
        }
    }

    /// Returns a bitboard in which squares attacked by Lance at the given square are filled.
    #[inline(always)]
    pub fn lance_attack(c: Color, sq: Square, occupied: &Bitboard) -> Bitboard {
        unsafe {
            let mask = &FILE_BB[sq.file() as usize] & &!&(&RANK1_BB | &RANK9_BB);
            let index = occupied_to_index(&(occupied & &mask), &mask);

            LANCE_ATTACK_BB[c as usize][sq.index()][index]
        }
    }

    /// Returns a bitboard in which squares in opposite player's area are filled.
    #[inline(always)]
    pub fn promote_zone(c: Color) -> Bitboard {
        match c {
            Color::Black => BitboardOr!(BitboardOr!(RANK1_BB, RANK2_BB), RANK3_BB),
            Color::White => BitboardOr!(BitboardOr!(RANK7_BB, RANK8_BB), RANK9_BB),
        }
    }

    /// Returns a bitboard in which squares between the given two squares are filled.
    #[inline(always)]
    pub fn between(sq1: Square, sq2: Square) -> Bitboard {
        unsafe { BETWEEN_BB[sq1.index()][sq2.index()] }
    }
}

const EMPTY_BB: Bitboard = Bitboard { p: [0, 0] };
const FULL_BB: Bitboard = Bitboard {
    p: [0x7fff_ffff_ffff_ffff, 0x0000_0000_0003_ffff],
};

const FILE1_BB: Bitboard = Bitboard { p: [0x1FF, 0] };
const FILE2_BB: Bitboard = Bitboard { p: [0x1FF << 9, 0] };
const FILE3_BB: Bitboard = Bitboard {
    p: [0x1FF << (9 * 2), 0],
};
const FILE4_BB: Bitboard = Bitboard {
    p: [0x1FF << (9 * 3), 0],
};
const FILE5_BB: Bitboard = Bitboard {
    p: [0x1FF << (9 * 4), 0],
};
const FILE6_BB: Bitboard = Bitboard {
    p: [0x1FF << (9 * 5), 0],
};
const FILE7_BB: Bitboard = Bitboard {
    p: [0x1FF << (9 * 6), 0],
};
const FILE8_BB: Bitboard = Bitboard { p: [0, 0x1FF] };
const FILE9_BB: Bitboard = Bitboard { p: [0, 0x1FF << 9] };
const FILE_BB: [Bitboard; 9] = [
    FILE1_BB, FILE2_BB, FILE3_BB, FILE4_BB, FILE5_BB, FILE6_BB, FILE7_BB, FILE8_BB, FILE9_BB,
];

const RANK1_BB: Bitboard = Bitboard {
    p: [0x0040_2010_0804_0201, 0x201],
};
const RANK2_BB: Bitboard = Bitboard {
    p: [0x0040_2010_0804_0201 << 1, 0x201 << 1],
};
const RANK3_BB: Bitboard = Bitboard {
    p: [0x0040_2010_0804_0201 << 2, 0x201 << 2],
};
const RANK4_BB: Bitboard = Bitboard {
    p: [0x0040_2010_0804_0201 << 3, 0x201 << 3],
};
const RANK5_BB: Bitboard = Bitboard {
    p: [0x0040_2010_0804_0201 << 4, 0x201 << 4],
};
const RANK6_BB: Bitboard = Bitboard {
    p: [0x0040_2010_0804_0201 << 5, 0x201 << 5],
};
const RANK7_BB: Bitboard = Bitboard {
    p: [0x0040_2010_0804_0201 << 6, 0x201 << 6],
};
const RANK8_BB: Bitboard = Bitboard {
    p: [0x0040_2010_0804_0201 << 7, 0x201 << 7],
};
const RANK9_BB: Bitboard = Bitboard {
    p: [0x0040_2010_0804_0201 << 8, 0x201 << 8],
};
const RANK_BB: [Bitboard; 9] = [
    RANK1_BB, RANK2_BB, RANK3_BB, RANK4_BB, RANK5_BB, RANK6_BB, RANK7_BB, RANK8_BB, RANK9_BB,
];

const IN_FRONT_BLACK_RANK1_BB: Bitboard = EMPTY_BB;
const IN_FRONT_BLACK_RANK2_BB: Bitboard = RANK1_BB;
const IN_FRONT_BLACK_RANK3_BB: Bitboard = BitboardOr!(IN_FRONT_BLACK_RANK2_BB, RANK2_BB);
const IN_FRONT_BLACK_RANK4_BB: Bitboard = BitboardOr!(IN_FRONT_BLACK_RANK3_BB, RANK3_BB);
const IN_FRONT_BLACK_RANK5_BB: Bitboard = BitboardOr!(IN_FRONT_BLACK_RANK4_BB, RANK4_BB);
const IN_FRONT_BLACK_RANK6_BB: Bitboard = BitboardOr!(IN_FRONT_BLACK_RANK5_BB, RANK5_BB);
const IN_FRONT_BLACK_RANK7_BB: Bitboard = BitboardOr!(IN_FRONT_BLACK_RANK6_BB, RANK6_BB);
const IN_FRONT_BLACK_RANK8_BB: Bitboard = BitboardOr!(IN_FRONT_BLACK_RANK7_BB, RANK7_BB);
const IN_FRONT_BLACK_RANK9_BB: Bitboard = BitboardOr!(IN_FRONT_BLACK_RANK8_BB, RANK8_BB);

const IN_FRONT_WHITE_RANK9_BB: Bitboard = EMPTY_BB;
const IN_FRONT_WHITE_RANK8_BB: Bitboard = RANK9_BB;
const IN_FRONT_WHITE_RANK7_BB: Bitboard = BitboardOr!(IN_FRONT_WHITE_RANK8_BB, RANK8_BB);
const IN_FRONT_WHITE_RANK6_BB: Bitboard = BitboardOr!(IN_FRONT_WHITE_RANK7_BB, RANK7_BB);
const IN_FRONT_WHITE_RANK5_BB: Bitboard = BitboardOr!(IN_FRONT_WHITE_RANK6_BB, RANK6_BB);
const IN_FRONT_WHITE_RANK4_BB: Bitboard = BitboardOr!(IN_FRONT_WHITE_RANK5_BB, RANK5_BB);
const IN_FRONT_WHITE_RANK3_BB: Bitboard = BitboardOr!(IN_FRONT_WHITE_RANK4_BB, RANK4_BB);
const IN_FRONT_WHITE_RANK2_BB: Bitboard = BitboardOr!(IN_FRONT_WHITE_RANK3_BB, RANK3_BB);
const IN_FRONT_WHITE_RANK1_BB: Bitboard = BitboardOr!(IN_FRONT_WHITE_RANK2_BB, RANK2_BB);

const IN_FRONT_BB: [[Bitboard; 9]; 2] = [
    [
        IN_FRONT_BLACK_RANK1_BB,
        IN_FRONT_BLACK_RANK2_BB,
        IN_FRONT_BLACK_RANK3_BB,
        IN_FRONT_BLACK_RANK4_BB,
        IN_FRONT_BLACK_RANK5_BB,
        IN_FRONT_BLACK_RANK6_BB,
        IN_FRONT_BLACK_RANK7_BB,
        IN_FRONT_BLACK_RANK8_BB,
        IN_FRONT_BLACK_RANK9_BB,
    ],
    [
        IN_FRONT_WHITE_RANK1_BB,
        IN_FRONT_WHITE_RANK2_BB,
        IN_FRONT_WHITE_RANK3_BB,
        IN_FRONT_WHITE_RANK4_BB,
        IN_FRONT_WHITE_RANK5_BB,
        IN_FRONT_WHITE_RANK6_BB,
        IN_FRONT_WHITE_RANK7_BB,
        IN_FRONT_WHITE_RANK8_BB,
        IN_FRONT_WHITE_RANK9_BB,
    ],
];

static mut ROOK_BLOCK_MASK: [Bitboard; 81] = [Bitboard { p: [0, 0] }; 81];
static mut ROOK_ATTACK_INDEX: [usize; 81] = [0; 81];
static mut ROOK_ATTACK_BB: [Bitboard; 495_616] = [Bitboard { p: [0, 0] }; 495_616];
const ROOK_BLOCK_BITS: [usize; 81] = [
    14, 13, 13, 13, 13, 13, 13, 13, 14, 13, 12, 12, 12, 12, 12, 12, 12, 13, 13, 12, 12, 12, 12, 12,
    12, 12, 13, 13, 12, 12, 12, 12, 12, 12, 12, 13, 13, 12, 12, 12, 12, 12, 12, 12, 13, 13, 12, 12,
    12, 12, 12, 12, 12, 13, 13, 12, 12, 12, 12, 12, 12, 12, 13, 13, 12, 12, 12, 12, 12, 12, 12, 13,
    14, 13, 13, 13, 13, 13, 13, 13, 14,
];

static mut BISHOP_BLOCK_MASK: [Bitboard; 81] = [Bitboard { p: [0, 0] }; 81];
static mut BISHOP_ATTACK_INDEX: [usize; 81] = [0; 81];
static mut BISHOP_ATTACK_BB: [Bitboard; 20224] = [Bitboard { p: [0, 0] }; 20224];
const BISHOP_BLOCK_BITS: [usize; 81] = [
    7, 6, 6, 6, 6, 6, 6, 6, 7, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 8, 8, 8, 8, 8, 6, 6, 6, 6, 8, 10,
    10, 10, 8, 6, 6, 6, 6, 8, 10, 12, 10, 8, 6, 6, 6, 6, 8, 10, 10, 10, 8, 6, 6, 6, 6, 8, 8, 8, 8,
    8, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 7, 6, 6, 6, 6, 6, 6, 6, 7,
];

static mut LANCE_ATTACK_BB: [[[Bitboard; 128]; 81]; 2] = [[[Bitboard { p: [0, 0] }; 128]; 81]; 2];
static mut ATTACK_BB: [[[Bitboard; 81]; 2]; 14] = [[[Bitboard { p: [0, 0] }; 81]; 2]; 14];

static mut BETWEEN_BB: [[Bitboard; 81]; 81] = [[Bitboard { p: [0, 0] }; 81]; 81];

#[inline(always)]
fn index_to_occupied(index: usize, bits: usize, mask: &Bitboard) -> Bitboard {
    let mut bb = Bitboard::empty();
    let mut mask_work = *mask;
    for i in 0..bits {
        let sq = mask_work.pop();
        if index & (1 << i) != 0 {
            bb |= sq;
        }
    }

    bb
}

#[inline(always)]
fn occupied_to_index(occupied: &Bitboard, mask: &Bitboard) -> usize {
    occupied.merge().pext(mask.merge()) as usize
}

#[inline(always)]
fn color2index(c: Color) -> usize {
    c as usize
}

fn init_rook_block() {
    for sq in Square::iter() {
        let file = sq.file();
        let rank = sq.rank();

        let mut bb = &FILE_BB[file as usize] ^ &RANK_BB[rank as usize];

        if file != 0 {
            bb &= &!&FILE1_BB;
        }
        if file != 8 {
            bb &= &!&FILE9_BB;
        }
        if rank != 0 {
            bb &= &!&RANK1_BB;
        }
        if rank != 8 {
            bb &= &!&RANK9_BB;
        }

        unsafe {
            ROOK_BLOCK_MASK[sq.index()] = bb;
        }
    }
}

fn init_bishop_block() {
    for bishop_sq in Square::iter() {
        let bf = bishop_sq.file() as i8;
        let br = bishop_sq.rank() as i8;

        let mut bb = Bitboard::empty();
        for sq in Square::iter() {
            let file = sq.file() as i8;
            let rank = sq.rank() as i8;

            if (file - bf).abs() == (rank - br).abs() {
                bb |= sq;
            }
        }
        bb &= &!&(&(&(&FILE1_BB | &FILE9_BB) | &RANK1_BB) | &RANK9_BB);
        bb &= &!&SQUARE_BB[bishop_sq.index()];

        unsafe {
            BISHOP_BLOCK_MASK[bishop_sq.index()] = bb;
        }
    }
}

fn calc_beam_attack(piece_sq: Square, dirs: &[(i8, i8)], occupied: &Bitboard) -> Bitboard {
    let mut bb = Bitboard::empty();
    for dir in dirs {
        let mut ptr = piece_sq;
        while let Some(sq) = ptr.shift(dir.0, dir.1) {
            bb |= sq;

            if (occupied & sq).is_any() {
                break;
            }

            ptr = sq;
        }
    }

    bb
}

fn init_rook_attack() {
    const ROOK_DIRS: [(i8, i8); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    let mut index = 0;
    for sq in Square::iter() {
        unsafe {
            ROOK_ATTACK_INDEX[sq.index()] = index;
            let block_mask = &ROOK_BLOCK_MASK[sq.index()];

            let bits = ROOK_BLOCK_BITS[sq.index()];
            for i in 0..(1 << bits) {
                let occupied = index_to_occupied(i, bits, block_mask);
                let masked_occupied = &occupied & block_mask;

                ROOK_ATTACK_BB[index + occupied_to_index(&masked_occupied, block_mask)] =
                    calc_beam_attack(sq, &ROOK_DIRS, &occupied);
            }

            index += 1 << bits;
        }
    }
}

fn init_bishop_attack() {
    const BISHOP_DIRS: [(i8, i8); 4] = [(1, 1), (-1, 1), (1, -1), (-1, -1)];

    let mut index = 0;
    for sq in Square::iter() {
        unsafe {
            BISHOP_ATTACK_INDEX[sq.index()] = index;
            let block_mask = &BISHOP_BLOCK_MASK[sq.index()];

            let bits = BISHOP_BLOCK_BITS[sq.index()];
            for i in 0..(1 << bits) {
                let occupied = index_to_occupied(i, bits, block_mask);
                let masked_occupied = &occupied & block_mask;

                BISHOP_ATTACK_BB[index + occupied_to_index(&masked_occupied, block_mask)] =
                    calc_beam_attack(sq, &BISHOP_DIRS, &occupied);
            }

            index += 1 << bits;
        }
    }
}

fn init_king_attack() {
    let index = PieceType::King as usize;

    for sq in Square::iter() {
        let bb = &Factory::rook_attack(sq, &FULL_BB) | &Factory::bishop_attack(sq, &FULL_BB);
        unsafe {
            ATTACK_BB[index][0][sq.index()] = bb;
            ATTACK_BB[index][1][sq.index()] = bb;
        }
    }
}

fn init_gold_attack() {
    let index = PieceType::Gold as usize;
    let king_index = PieceType::King as usize;

    for c in Color::iter() {
        let color_index = color2index(c);

        for sq in Square::iter() {
            unsafe {
                let bb = &(&ATTACK_BB[king_index][color_index][sq.index()]
                    & &IN_FRONT_BB[color_index][sq.rank() as usize])
                    | &Factory::rook_attack(sq, &FULL_BB);
                ATTACK_BB[index][color_index][sq.index()] = bb;
            }
        }
    }
}

fn init_silver_attack() {
    let index = PieceType::Silver as usize;
    let king_index = PieceType::King as usize;

    for c in Color::iter() {
        let color_index = color2index(c);

        for sq in Square::iter() {
            unsafe {
                let bb = &(&ATTACK_BB[king_index][color_index][sq.index()]
                    & &IN_FRONT_BB[color_index][sq.rank() as usize])
                    | &Factory::bishop_attack(sq, &FULL_BB);
                ATTACK_BB[index][color_index][sq.index()] = bb;
            }
        }
    }
}

fn init_pawn_attack() {
    let index = PieceType::Pawn as usize;
    let silver_index = PieceType::Silver as usize;

    for c in Color::iter() {
        let color_index = color2index(c);

        for sq in Square::iter() {
            unsafe {
                ATTACK_BB[index][color_index][sq.index()] = &ATTACK_BB[silver_index][color_index]
                    [sq.index()]
                    ^ &Factory::bishop_attack(sq, &FULL_BB);
            }
        }
    }
}

fn init_knight_attack() {
    let index = PieceType::Knight as usize;
    let pawn_index = PieceType::Pawn as usize;

    for c in Color::iter() {
        let color_index = color2index(c);

        for sq in Square::iter() {
            let mut bb = Bitboard::empty();
            unsafe {
                let mut pawn_bb = ATTACK_BB[pawn_index][color_index][sq.index()];

                if pawn_bb.is_any() {
                    let psq = pawn_bb.pop();
                    bb = &Factory::bishop_attack(psq, &FULL_BB)
                        & &IN_FRONT_BB[color_index][sq.rank() as usize];
                }
                ATTACK_BB[index][color_index][sq.index()] = bb;
            }
        }
    }
}

fn init_lance_attack() {
    for c in Color::iter() {
        let color_index = color2index(c);

        for sq in Square::iter() {
            let block_mask = &FILE_BB[sq.file() as usize] & &!&(&RANK1_BB | &RANK9_BB);

            const BITS: usize = 7;
            for i in 0..1 << BITS {
                let occupied = index_to_occupied(i, BITS, &block_mask);
                unsafe {
                    LANCE_ATTACK_BB[color_index][sq.index()][i] =
                        &Factory::rook_attack(sq, &occupied)
                            & &IN_FRONT_BB[color_index][sq.rank() as usize];
                }
            }
        }
    }
}

fn init_between() {
    for from in Square::iter() {
        for to in Square::iter() {
            if from == to {
                continue;
            }

            let df = from.file() as i8 - to.file() as i8;
            let dr = from.rank() as i8 - to.rank() as i8;
            unsafe {
                if df == 0 || dr == 0 {
                    BETWEEN_BB[from.index()][to.index()] =
                        &Factory::rook_attack(from, &square_bb(to))
                            & &Factory::rook_attack(to, &square_bb(from));
                } else if df.abs() == dr.abs() {
                    BETWEEN_BB[from.index()][to.index()] =
                        &Factory::bishop_attack(from, &square_bb(to))
                            & &Factory::bishop_attack(to, &square_bb(from));
                }
            }
        }
    }
}
