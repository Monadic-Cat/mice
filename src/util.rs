//! Nice to have utilities that aren't core to dice
//! manipulation itself, just handy for some reason.
#[cfg(not(target_arch = "wasm32"))]
use crate::{roll_tuples, tuple_vec};
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

/// For providing access to mice to irresponsible users
#[cfg(not(target_arch = "wasm32"))]
pub fn roll_capped(input: &str, cap: i64) -> UResult {
    let dice: Vec<(i64, i64)> = tuple_vec(input)?;
    let mut roll_count = 0;
    for d in dice.iter() {
        if d.1 > 1 {
            roll_count += d.0;
        } else {
            // This branch only saves time
            // in the worst case - when there's
            // a truly obscene number of terms.
            roll_count += 1;
        }
        // Prevent worst case performance
        if roll_count > cap {
            break;
        }
    }
    if roll_count > cap {
        Err(UtilError::ExceededCap)
    } else {
        Ok(roll_tuples(&dice)?)
    }
}
