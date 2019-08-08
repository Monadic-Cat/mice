# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
 - Providing nicely formatted result strings via `util::roll_vec_nice`.

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

[Unreleased]: https://github.com/Monadic-Cat/mice/compare/0.1.7...HEAD
[0.1.7]: https://github.com/Monadic-Cat/mice/compare/0.1.6...0.1.7
[0.1.6]: https://github.com/Monadic-Cat/mice/compare/0.1.5...0.1.6
[0.1.5]: https://github.com/Monadic-Cat/mice/compare/0.1.4...0.1.5
[0.1.4]: https://github.com/Monadic-Cat/mice/compare/0.1.3...0.1.4
[0.1.3]: https://github.com/Monadic-Cat/mice/compare/0.1.2...0.1.3
[0.1.2]: https://github.com/Monadic-Cat/mice/compare/0.1.1...0.1.2
[0.1.1]: https://github.com/Monadic-Cat/mice/compare/0.1.0...0.1.1
[0.1.0]: https://github.com/Monadic-Cat/mice/releases/tag/0.1.0
