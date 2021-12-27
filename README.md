# shogi-rs

[![Github Actions](https://github.com/nozaq/shogi-rs/workflows/build/badge.svg)](https://github.com/nozaq/shogi-rs/actions?workflow=build)
[![Coverage Status](https://coveralls.io/repos/github/nozaq/shogi-rs/badge.svg?branch=master)](https://coveralls.io/github/nozaq/shogi-rs?branch=master)
[![crates.io](https://img.shields.io/crates/v/shogi.svg)](https://crates.io/crates/shogi)
[![docs.rs](https://docs.rs/shogi/badge.svg)](https://docs.rs/shogi)

A Bitboard-based shogi library in Rust. Board representation, move generation/validation and time control utilities.

[Documentation](https://docs.rs/shogi)

## Usage

A library for implementing Shogi application.

`shogi` provides a various types and implementations for representing concepts and rules in Shogi.
Most types can be created programatically while they can also be deserialized from / serialized to SFEN format.
See [USIプロトコルとは (What is the USI protocol?)](http://shogidokoro.starfree.jp/usi.html) for more detail about UCI protocol specification and SFEN format.

## Examples

```rust
use shogi::{Move, Position};
use shogi::bitboard::Factory as BBFactory;
use shogi::square::consts::*;

BBFactory::init();
let mut pos = Position::new();

// Position can be set from the SFEN formatted string.
pos.set_sfen("lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1").unwrap();

// You can programatically create a Move instance.
let m = Move::Normal{from: SQ_7G, to: SQ_7F, promote: false};
pos.make_move(m).unwrap();

// Move can be created from the SFEN formatted string as well.
let m = Move::from_sfen("7c7d").unwrap();
pos.make_move(m).unwrap();

// Position can be converted back to the SFEN formatted string.
assert_eq!("lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1 moves 7g7f 7c7d", pos.to_sfen());
```

## Related crates

- [csa-rs](https://github.com/nozaq/csa-rs): A Shogi game serialization/deserialization library in CSA format. 
- [usi-rs](https://github.com/nozaq/usi-rs): A library to handle type-safe communication with USI-compatible shogi engines. 

## License

`shogi-rs` is licensed under the MIT license. Please read the [LICENSE](LICENSE) file in this repository for more information.
