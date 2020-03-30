# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.7.0] - 2020-3-30
### Changed
 - `tuple_vec` now uses the more specific `ParseError` error type.
### Fixed
 - Sign factoring on single die terms

## [0.6.1] - 2020-3-30 [YANKED]
I accidentally published a planned breaking change together with a bugfix.

## [0.6.0]
### Changed
 - Rename `RollError` to `Error`.
### Fixed
 - `#[cfg]` away unusable functions on `wasm32` targets.
### Added
 - Expose `ParseError` so fine grained error handling on the builder is less bad.

## [0.5.3]
### Fixed
 - Actually fix the thing from `0.5.2`

## [0.5.2]
### Fixed
 - Arithmetic formatter incorrectly discarded negatives.

## [0.5.1]
### Fixed
 - `expose` module incorrectly accepted d0 terms. Fixed.

## [0.5.0]
### Changed
 - Changed default `ExpressionResult` format.
 - Names of `tupl_vec` and `roll_tupls` to
   `tuple_vec` and `roll_tuples`, respectively.
### Added
 - `FormatOptions`, to control the formatting of
   dice expression results.

## [0.4.0]
### Added
 - Interface for more involved operations: `builder::*`.
 - WASM compatibility through `RollBuilder`.
 - Basic usage examples on crate level doc page.
### Changed
- `ExpressionResult`s can now be passed to JS from WASM.
- Removed `.` from the end of
  `RollError::InvalidExpression`'s `Display` message.

## [0.3.1] - 2019-08-16
### Added
 - Added `util` module, which provides `roll_capped`.

## [0.3.0] - 2019-08-12
### Changed
 - Change `ExprTuple` to `(i64, i64)` from `(i64, u64)`
 - Replace public `total` field on `ExpressionResult`
   with `ExpressionResult::total()` method.

## [0.2.2] - 2019-08-07
### Changed
 - Improved formatting output

## [0.2.1] - 2019-08-06
### Fixed
 - Fixed usage of `gen_range` from the `rand` crate. Rolls should now
   produce outcomes in [1, n] for n-sided dice, as opposed to [1, n).

## [0.2.0] - 2019-08-02
### Changed
 - Removed `roll_vec` and replaced with the largely equivalent
   `roll_tupls`.
 - Removed `roll_dice` and replaced with the similar `roll`,
   which is more useful.
 - Renamed `dice_vec` to `tupl_vec`.

## [0.1.7] - 2019-08-08
### Fixed
 - Fixed usage of gen_range from the rand crate.
   Rolls should now produce outcomes in [1, n]
   for n-sided dice, as opposed to [1, n).
   **(Backported from 0.2.x)**

## [0.1.6] - 2019-07-18
### Fixed
 - Removed a `println!` I used for
   debugging in the `0.1.5` bugfix.

## [0.1.5] - 2019-07-18
### Fixed
 - Version `0.1.4` ignored signs of terms. "10 - 5",
   for example, evaluated to 15. Reverted to correct
   behavior.

## [0.1.4] - 2019-07-15
### Added
 - `dice_vec` and `roll_vec` functions, to allow
   manipulation of terms before evaluating an expression.

## [0.1.3] - 2019-07-07
### Fixed
 - Prior to this version, `roll_dice` could panic if
   the sum of terms overflowed an `i64`.

## [0.1.2] - 2019-07-07 [YANKED]
I forget why I yanked this version,
but there's no good reason to use it.

## [0.1.1] - 2019-07-07
### Fixed
 - Prior to this version, `roll_dice` would replace
   numbers too large to fit in an `i64` with `1`,
   which is misleading. Replaced with an error.

## [0.1.0] - 2019-07-07
### Added
 - `roll_dice`, a nice to have function for
   evaluating dice expression.

[Unreleased]: https://github.com/Monadic-Cat/mice/compare/0.7.0...HEAD
[0.7.0]: https://github.com/Monadic-Cat/mice/compare/0.6.1...0.7.0
[0.6.1]: https://github.com/Monadic-Cat/mice/compare/0.6.0...0.6.1
[0.6.0]: https://github.com/Monadic-Cat/mice/compare/0.5.3...0.6.0
[0.5.3]: https://github.com/Monadic-Cat/mice/compare/0.5.2...0.5.3
[0.5.2]: https://github.com/Monadic-Cat/mice/compare/0.5.1...0.5.2
[0.5.1]: https://github.com/Monadic-Cat/mice/compare/0.5.0...0.5.1
[0.5.0]: https://github.com/Monadic-Cat/mice/compare/0.4.0...0.5.0
[0.4.0]: https://github.com/Monadic-Cat/mice/compare/0.3.1...0.4.0
[0.3.1]: https://github.com/Monadic-Cat/mice/compare/0.3.0...0.3.1
[0.3.0]: https://github.com/Monadic-Cat/mice/compare/0.2.2...0.3.0
[0.2.2]: https://github.com/Monadic-Cat/mice/compare/0.2.1...0.2.2
[0.2.1]: https://github.com/Monadic-Cat/mice/compare/0.2.0...0.2.1
[0.2.0]: https://github.com/Monadic-Cat/mice/compare/0.1.7...0.2.0
[0.1.7]: https://github.com/Monadic-Cat/mice/compare/0.1.6...0.1.7
[0.1.6]: https://github.com/Monadic-Cat/mice/compare/0.1.5...0.1.6
[0.1.5]: https://github.com/Monadic-Cat/mice/compare/0.1.4...0.1.5
[0.1.4]: https://github.com/Monadic-Cat/mice/compare/0.1.3...0.1.4
[0.1.3]: https://github.com/Monadic-Cat/mice/compare/0.1.2...0.1.3
[0.1.2]: https://github.com/Monadic-Cat/mice/compare/0.1.1...0.1.2
[0.1.1]: https://github.com/Monadic-Cat/mice/compare/0.1.0...0.1.1
[0.1.0]: https://github.com/Monadic-Cat/mice/releases/tag/0.1.0
