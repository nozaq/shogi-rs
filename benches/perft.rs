#![feature(test)]

extern crate test;
extern crate shogi;

#[cfg(test)]
mod tests {
    use test::Bencher;
    use shogi::{Color, Position, Square};
    use shogi::bitboard::Factory;

    #[bench]
    fn perft(b: &mut Bencher) {
        Factory::init();

        let mut pos = Position::new();
        pos.set_sfen("l6nl/5+P1gk/2np1S3/p1p4Pp/3P2Sp1/1PPb2P1P/P5GS1/R8/LN4bKL w RGgsn5p 1")
            .expect("failed to parse SFEN string");

        b.iter(|| {
            let mut cnt = 0;

            for sq in Square::iter() {
                let pc = pos.piece_at(sq);

                if let Some(ref pc) = *pc {
                    if pc.color == Color::White {
                        let bb = pos.move_candidates(sq, pc);
                        cnt += bb.count();
                    }
                }
            }

            assert_eq!(32, cnt);
        });
    }
}