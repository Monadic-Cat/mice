//! Nice to have utilities that aren't core to dice
//! manipulation itself, just handy for some reason.
#[cfg(feature = "thread_rng")]
use crate::parse::{Expression, Term};
use crate::{Error, ExpressionResult, parse::ParseError};
use thiserror::Error;

#[derive(Debug, Clone, Copy, Error)]
pub enum UtilError {
    #[error("tried to DOS me.")]
    ExceededCap,
    #[error("{0}")]
    RollError(Error),
}
impl From<Error> for UtilError {
    fn from(e: Error) -> Self {
        UtilError::RollError(e)
    }
}
impl From<ParseError> for UtilError {
    fn from(e: ParseError) -> Self {
        UtilError::RollError(e.into())
    }
}

type UResult = Result<ExpressionResult, UtilError>;

#[cfg(feature = "thread_rng")]
fn exceeds_cap(dice: &Expression, cap: i64) -> bool {
    let mut roll_count = 0;
    for term in dice.terms() {
        match term {
            Term::Dice(d) => if d.size > 1 {
                roll_count += d.number;
            } else {
                roll_count += 1;
            },
            // This branch only saves time
            // in the worst case - when there's
            // a truly obscene number of terms.
            Term::Constant(_) => roll_count += 1,
        }
        // Prevent worst case performance
        if roll_count > cap {
            return true;
        }
    }
    roll_count > cap
}

#[cfg(feature = "thread_rng")]
pub struct ExceededCap;

#[cfg(feature = "thread_rng")]
impl From<ExceededCap> for UtilError {
    fn from(_: ExceededCap) -> Self {
        Self::ExceededCap
    }
}

#[cfg(feature = "thread_rng")]
pub fn roll_exp_capped(dice: Expression, cap: i64) ->
    Result<Result<ExpressionResult, crate::Error>, ExceededCap>
{
    if !exceeds_cap(&dice, cap) {
        Ok(crate::roll_expr_iter_with(&mut rand::thread_rng(), dice.into_iter()))
    } else {
        Err(ExceededCap)
    }
}

/// For providing access to mice to irresponsible users
#[cfg(feature = "thread_rng")]
pub fn roll_capped(input: &str, cap: i64) -> UResult {
    let dice = crate::parse::wrap_dice(input)?;
    match roll_exp_capped(dice, cap) {
        Ok(Ok(x)) => Ok(x),
        Ok(Err(e)) => Err(e.into()),
        Err(e) => Err(e.into()),
    }
}
