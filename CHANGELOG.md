<a name="unreleased"></a>
## [Unreleased]


<a name="0.10.3"></a>
## [0.10.3] - 2021-10-03
### Fix
- add inc_time before the turn begins


<a name="0.10.2"></a>
## [0.10.2] - 2021-08-15

<a name="0.10.1"></a>
## [0.10.1] - 2021-08-15
### Fix
- broken compilation in perft ([#31](https://github.com/nozaq/shogi-rs/issues/31))


<a name="0.10.0"></a>
## [0.10.0] - 2020-11-14
### Fix
- use range expressions
- clippy warnings
- rename an unused variable
- unmaking of capturing move ([#27](https://github.com/nozaq/shogi-rs/issues/27))


<a name="0.9.0"></a>
## [0.9.0] - 2019-12-30
### Refactor
- fix clippy warnings and errors
- use dyn for specifying trait objects
- use ? instead of r#try


<a name="0.8.0"></a>
## [0.8.0] - 2019-01-24
### Feat
- add Position#player_bb()

### Fix
- add Cargo.toml
- Coveralls integration

### Refactor
- migrate to 2018 edition
- upgrade itertools to v0.8.0
- upgrade bitintr to v0.2


<a name="0.7.0"></a>
## [0.7.0] - 2017-05-14
### Refactor
- extract 'usi' module as an external crate


<a name="0.6.0"></a>
## [0.6.0] - 2017-03-05

<a name="0.5.0"></a>
## [0.5.0] - 2017-02-25
### Feat
- ensure Square not to return invalid values ([#7](https://github.com/nozaq/shogi-rs/issues/7))

### Fix
- check pinning status in Uchifuzume detection ([#8](https://github.com/nozaq/shogi-rs/issues/8))


<a name="0.4.0"></a>
## [0.4.0] - 2017-02-22
### Feat
- add Nyugyoku check ([#6](https://github.com/nozaq/shogi-rs/issues/6))


<a name="0.3.0"></a>
## [0.3.0] - 2017-02-19
### Feat
- add a helper type to manage time controls ([#5](https://github.com/nozaq/shogi-rs/issues/5))

### Refactor
- remove an unused variable


<a name="0.2.0"></a>
## [0.2.0] - 2017-02-19
### Feat
- USI protocol helpers ([#4](https://github.com/nozaq/shogi-rs/issues/4))


<a name="0.1.0"></a>
## 0.1.0 - 2017-02-18

[Unreleased]: https://github.com/nozaq/shogi-rs/compare/0.10.3...HEAD
[0.10.3]: https://github.com/nozaq/shogi-rs/compare/0.10.2...0.10.3
[0.10.2]: https://github.com/nozaq/shogi-rs/compare/0.10.1...0.10.2
[0.10.1]: https://github.com/nozaq/shogi-rs/compare/0.10.0...0.10.1
[0.10.0]: https://github.com/nozaq/shogi-rs/compare/0.9.0...0.10.0
[0.9.0]: https://github.com/nozaq/shogi-rs/compare/0.8.0...0.9.0
[0.8.0]: https://github.com/nozaq/shogi-rs/compare/0.7.0...0.8.0
[0.7.0]: https://github.com/nozaq/shogi-rs/compare/0.6.0...0.7.0
[0.6.0]: https://github.com/nozaq/shogi-rs/compare/0.5.0...0.6.0
[0.5.0]: https://github.com/nozaq/shogi-rs/compare/0.4.0...0.5.0
[0.4.0]: https://github.com/nozaq/shogi-rs/compare/0.3.0...0.4.0
[0.3.0]: https://github.com/nozaq/shogi-rs/compare/0.2.0...0.3.0
[0.2.0]: https://github.com/nozaq/shogi-rs/compare/0.1.0...0.2.0
