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
//! # use mice::{roll, RollError};
//! println!("{}", roll("2d6 + 3")?);
//!
//! println!("{}", roll("2d6 + 3")?.total());
//!
//! let result = roll("2d6 + 3")?;
//! println!("{}\n{}", result, result.total());
//! # Ok::<(), RollError>(())
//! ```
//!
//! The parser accepts an arbitrary number of terms in a dice expression.
//! ```
//! # use mice::{roll, RollError};
//! println!("{}", roll("9d8 + 4d2 - 5 - 8d7")?);
//! # Ok::<(), RollError>(())
//! ```
#![forbid(unsafe_code)]
use rand::Rng;
use std::fmt::Display;
use std::fmt::Formatter;
mod error;
use error::MyResult;
pub use error::RollError;
mod post;
pub use post::ExpressionResult;
use post::{EResult, EvaluatedTerm, RolledDie, TResult};
mod expose;
pub use expose::{roll_tupls, tupl_vec};
mod parse;
use parse::{Die, Expr, Sign, Term};
pub mod builder;
use builder::RollBuilder;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
pub mod util;

impl Display for Term {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Term::Die(x) => write!(f, "{}d{}", x.number, x.size),
            Term::Constant(x) => write!(f, "{}", x),
        }
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        // N
        // -N
        // NdN
        // -NdN
        let mut nstr = String::new();
        match self.sign {
            Sign::Positive => (),
            Sign::Negative => nstr.push_str("-"),
        }
        nstr.push_str(&format!("{}", self.term));
        write!(f, "{}", nstr)
    }
}

impl Display for EvaluatedTerm {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}

fn roll_die_with<R>(a: &Die, rng: &mut R) -> Result<RolledDie, RollError>
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
        Err(RollError::InvalidDie)
    } else {
        let mut total: i64 = 0;
        let mut parts = Vec::new();
        // Rng::gen_range has an exlusive upper bound
        for n in (0..a.number).map(|_| rng.gen_range(1, a.size + 1)) {
            total = total.checked_add(n).ok_or(RollError::OverflowPositive)?;
            parts.push(n);
        }
        Ok(RolledDie {
            total,
            parts,
            sign_part: Sign::Positive,
        })
    }
}

impl From<RolledDie> for EvaluatedTerm {
    fn from(d: RolledDie) -> EvaluatedTerm {
        EvaluatedTerm::Die(d)
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
    (a.sign * t).into()
}

/// Evaluate a dice expression!
/// This function takes the usual dice expression format,
/// and allows an arbitrary number of terms.
/// ```
/// # use mice::roll;
/// # use mice::RollError;
/// let dice_expression = "d20 + 5 - d2";
/// println!("{}", roll(dice_expression)?);
/// # Ok::<(), RollError>(())
/// ```
///
/// An `Err` is returned in the following cases:
///   - A d0 is used
///   - The sum of all terms is too high
///   - The sum of all terms is too low
///   - Nonsense input
pub fn roll(input: &str) -> EResult {
    Ok(RollBuilder::new().parse(input)?.into_roll()?.roll()?)
}

fn try_roll_expr_iter_with<I, R>(rng: &mut R, input: I) -> EResult
where
    I: Iterator<Item = Result<Expr, RollError>>,
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
                    RollError::OverflowPositive
                } else {
                    RollError::OverflowNegative
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
