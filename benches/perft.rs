use criterion::{criterion_group, criterion_main, Criterion};
use shogi::bitboard::Factory;
use shogi::{Color, Position, Square};

fn bench_perft(c: &mut Criterion) {
    Factory::init();

    let mut pos = Position::new();
    pos.set_sfen("l6nl/5+P1gk/2np1S3/p1p4Pp/3P2Sp1/1PPb2P1P/P5GS1/R8/LN4bKL w RGgsn5p 1")
        .expect("failed to parse SFEN string");

    c.bench_function("perft", |b| b.iter(|| perft(&pos)));
}

fn perft(pos: &Position) {
    let mut cnt = 0;

    for sq in Square::iter() {
        let pc = pos.piece_at(sq);

        if let Some(ref pc) = *pc {
            if pc.color == Color::White {
                let bb = pos.move_candidates(sq, *pc);
                cnt += bb.count();
            }
        }
    }

    assert_eq!(32, cnt);
}

criterion_group!(benches, bench_perft);
criterion_main!(benches);
