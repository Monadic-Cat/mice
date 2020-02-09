//! # mice, messing with dice
//! The heading obviates the need for a body.
//!
//! This crate is written primarily for my own
//! usage, and will likely obtain extensions related
//! to games that I play.
//!
//! Some basic usage:
//!
//! ```
//! # use mice::{roll, Error};
//! println!("{}", roll("2d6 + 3")?);
//!
//! println!("{}", roll("2d6 + 3")?.total());
//!
//! let result = roll("2d6 + 3")?;
//! println!("{}\n{}", result, result.total());
//! # Ok::<(), Error>(())
//! ```
//!
//! The parser accepts an arbitrary number of terms in a dice expression.
//! ```
//! # use mice::{roll, Error};
//! println!("{}", roll("9d8 + 4d2 - 5 - 8d7")?);
//! # Ok::<(), Error>(())
//! ```
#![forbid(unsafe_code)]
use rand::Rng;
mod error;
pub use error::Error;
use error::MyResult;
mod post;
use post::{EResult, EvaluatedTerm, RolledDie, TResult};
pub use post::{ExpressionResult, FormatOptions};
mod expose;
#[cfg(not(target_arch = "wasm32"))]
pub use expose::{roll_tuples, tuple_vec};
mod parse;
use parse::{Die, Expr, Sign, Term};
pub use parse::ParseError;
pub mod builder;
use builder::RollBuilder;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
mod display;
pub mod prelude;
pub mod util;

/// Unstable methods I export for my use.
/// Developed in tandem with whatever I'm using it with.
/// NO guarantees are provided as to this module's behavior.
/// If you're not me, using this module may
/// cause failed builds or unpredictable runtime behavior.
/// Do not use this anywhere you cannot
/// guarantee the EXACT same version will be used on every build.
/// Copying `proc_macro2`'s Nightly-esque trick.
/// If you use this, you and all your reverse deps must
/// pass the mice_semver_exempt config flag to rustc.
/// ```text
/// RUSTFLAGS='--cfg mice_semver_exempt' cargo build
/// ```
/// This infectious nature is intentional, as it serves as
/// a reminder that you are outside of the normal semver guarantees.
/// Any sane person using this module would do well to specify a
/// maximally specific version. Not that you should use this module.
/// I'm just doing this because it's faster than spending the
/// time to iron out the kinks in what I'm exposing here before I do so.
#[cfg(mice_semver_exempt)]
#[doc(hidden)]
pub mod unstable {
    // So that all unstable stuff is clearly imported from `unstable`,
    // mirror the `parse` module inside `unstable`, instead of simply
    // cfg-ing `parse` public when using unstable features.
    pub mod parse {
        pub use crate::parse::*;
    }
}

fn roll_die_with<R>(a: &Die, rng: &mut R) -> Result<RolledDie, Error>
where
    R: Rng,
{
    if a.size == 1 {
        Ok(RolledDie {
            total: a.number,
            parts: (0..a.number).map(|_| 1).collect(),
            sign_part: Sign::Positive,
        })
    } else if a.size < 1 {
        Err(Error::InvalidDie)
    } else {
        let mut total: i64 = 0;
        let mut parts = Vec::new();
        // Rng::gen_range has an exlusive upper bound
        // Rng::gen includes the entire range of a type.
        for _ in 0..a.number {
            let random;
            if let Some(bound) = a.size.checked_add(1) {
                random = rng.gen_range(1, bound);
            } else {
                random = rng.gen();
            }
            total = total.checked_add(random).ok_or(Error::OverflowPositive)?;
            parts.push(random);
        }
        Ok(RolledDie {
            total,
            parts,
            sign_part: Sign::Positive,
        })
    }
}

fn eval_term_with<R>(a: &Expr, rng: &mut R) -> TResult
where
    R: Rng,
{
    let t = match a.term {
        Term::Die(x) => roll_die_with(&x, rng).into(),
        Term::Constant(x) => MyResult::Ok(EvaluatedTerm::Constant(x)),
    };
    // This can't trigger a panic with any inputs as of now,
    // but that may change. Switch to checked negation at some point.
    // This currently cannot panic because the maximum permitted
    // input is the `i64` positive max.
    // This could panic if `t` were the `i64` negative max.
    // See the `parse` module for the implementation of `Sign::mul`.
    // I suggest providing a `Sign::checked_mul` method for this usage.
    (a.sign * t).into()
}

/// Evaluate a dice expression!
/// This function takes the usual dice expression format,
/// and allows an arbitrary number of terms.
/// ```
/// # use mice::roll;
/// # use mice::Error;
/// let dice_expression = "d20 + 5 - d2";
/// println!("{}", roll(dice_expression)?);
/// # Ok::<(), Error>(())
/// ```
///
/// An `Err` is returned in the following cases:
///   - A d0 is used
///   - The sum of all terms is too high
///   - The sum of all terms is too low
///   - Nonsense input
#[cfg(not(target_arch = "wasm32"))]
pub fn roll(input: &str) -> EResult {
    Ok(RollBuilder::new().parse(input)?.into_roll()?.roll()?)
}

fn try_roll_expr_iter_with<I, R>(rng: &mut R, input: I) -> EResult
where
    I: Iterator<Item = Result<Expr, Error>>,
    R: Rng,
{
    // let mut rng = thread_rng(); // This doesn't work in WASM?
    let mut pairs = Vec::new();
    let mut total: i64 = 0;
    for x in input {
        match x {
            Ok(x) => {
                let res = eval_term_with(&x, rng)?;
                let res_val = res.value();
                pairs.push((x, res));
                total = total.checked_add(res_val).ok_or(if res_val > 0 {
                    Error::OverflowPositive
                } else {
                    Error::OverflowNegative
                })?;
            }
            Err(x) => return Err(x),
        }
    }
    Ok(ExpressionResult::new(pairs, total))
}

fn roll_expr_iter_with<I, R>(rng: &mut R, input: I) -> EResult
where
    I: Iterator<Item = Expr>,
    R: Rng,
{
    try_roll_expr_iter_with(rng, input.map(Ok))
}

// N
// dN1   (+/-) N2
// N1dN2 (+/-) N3
// N1dN2 (+/-) N3dN4 (+/-) [...] (+/-) NN

#[cfg(test)]
mod tests {
    use crate::{roll, Die};
    #[test]
    fn arithmetic() {
        assert_eq!(roll("5 + 3").unwrap().total(), 8);
        assert_eq!(roll("5 - 3").unwrap().total(), 2);
    }
    #[test]
    fn dice() {
        let mut good = true;
        match Die::new(0, 0) {
            Ok(_) => good = false,
            Err(_) => (),
        }
        if !good {
            panic!()
        }
    }
}
