<a name="unreleased"></a>
## [Unreleased]


<a name="0.12.2"></a>
## [0.12.2] - 2021-12-27
### Chore
- upgrade to 2021 edition ([#45](https://github.com/nozaq/shogi-rs/issues/45))
- upgrade checkout action to v2 ([#44](https://github.com/nozaq/shogi-rs/issues/44))


<a name="0.12.1"></a>
## [0.12.1] - 2021-12-09
### Fix
- Uchifuzume check ([#41](https://github.com/nozaq/shogi-rs/issues/41))
- typo and reference link ([#40](https://github.com/nozaq/shogi-rs/issues/40))


<a name="0.12.0"></a>
## [0.12.0] - 2021-11-14
### Feat
- make Position#find_king() public ([#39](https://github.com/nozaq/shogi-rs/issues/39))


<a name="0.11.0"></a>
## [0.11.0] - 2021-10-04
### Feat
- make Position#pinned_bb public ([#36](https://github.com/nozaq/shogi-rs/issues/36))
- modernize error types ([#35](https://github.com/nozaq/shogi-rs/issues/35))


<a name="0.10.3"></a>
## [0.10.3] - 2021-10-03
### Chore
- rename `master` branch to `main` ([#33](https://github.com/nozaq/shogi-rs/issues/33))

### Fix
- add inc_time before the turn begins ([#34](https://github.com/nozaq/shogi-rs/issues/34))


<a name="0.10.2"></a>
## [0.10.2] - 2021-08-15

<a name="0.10.1"></a>
## [0.10.1] - 2021-08-15
### Fix
- broken compilation in perft ([#31](https://github.com/nozaq/shogi-rs/issues/31))


<a name="0.10.0"></a>
## [0.10.0] - 2020-11-14
### Chore
- use tarpaulin to measure coverage

### Fix
- use range expressions
- clippy warnings
- rename an unused variable
- unmaking of capturing move ([#27](https://github.com/nozaq/shogi-rs/issues/27))


<a name="0.9.0"></a>
## [0.9.0] - 2019-12-30
### Chore
- delete unnecessary panic settings
- use Github Actions

### Refactor
- fix clippy warnings and errors
- use dyn for specifying trait objects
- use ? instead of r#try


<a name="0.8.0"></a>
## [0.8.0] - 2019-01-24
### Chore
- use git-chglog to generate CHANGELOG
- remove redundant builds
- add appveyor config

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
### Chore
- temporary workaround for coverall integration ([#3](https://github.com/nozaq/shogi-rs/issues/3))

### Feat
- USI protocol helpers ([#4](https://github.com/nozaq/shogi-rs/issues/4))


<a name="0.1.0"></a>
## 0.1.0 - 2017-02-18
### Chore
- add travis.yml


[Unreleased]: https://github.com/nozaq/shogi-rs/compare/0.12.2...HEAD
[0.12.2]: https://github.com/nozaq/shogi-rs/compare/0.12.1...0.12.2
[0.12.1]: https://github.com/nozaq/shogi-rs/compare/0.12.0...0.12.1
[0.12.0]: https://github.com/nozaq/shogi-rs/compare/0.11.0...0.12.0
[0.11.0]: https://github.com/nozaq/shogi-rs/compare/0.10.3...0.11.0
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
