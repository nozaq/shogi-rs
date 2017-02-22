#![feature(test)]

extern crate test;
extern crate shogi;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    use shogi::Position;

    #[bench]
    fn perft(b: &mut Bencher) {
        let pos = Position::new();
        pos.set_sfen("l6nl/5+P1gk/2np1S3/p1p4Pp/3P2Sp1/1PPb2P1P/P5GS1/R8/LN4bKL w RGgsn5p 1");

        b.iter(|| pos.move_candidates());
    }
}