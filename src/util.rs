//! Nice to have utilities that aren't core to dice
//! manipulation itself, just handy for some reason.
use crate::{roll_tupls, ExpressionResult, RollError};
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy)]
pub enum UtilError {
    ExceededCap,
    RollError(RollError),
}
impl From<RollError> for UtilError {
    fn from(e: RollError) -> Self {
        UtilError::RollError(e)
    }
}
impl Display for UtilError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            UtilError::ExceededCap => write!(f, "tried to DOS me."),
            UtilError::RollError(e) => e.fmt(f),
        }
    }
}
impl Error for UtilError {}

type UResult = Result<ExpressionResult, UtilError>;

/// For providing access to mice to irresponsible users
pub fn roll_capped(input: &str, cap: i64) -> UResult {
    let dice: Vec<(i64, i64)> = crate::tupl_vec(input)?;
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
        Ok(roll_tupls(&dice)?)
    }
}
