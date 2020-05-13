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

pub(crate) mod unstable {
    use crate::builder::RollBuilder;
    use crate::expose::ExprTuple;
    use crate::parse::Expression;
    use crate::Error as MError;
    use crate::ExpressionResult;
    use mice_macro_lib::pub_if;
    use thiserror::Error;
    #[pub_if(mice_semver_exempt)]
    fn exceeds_cap(dice: &Expression, cap: i64) -> bool {
        let mut roll_count = 0;
        for d in dice.iter() {
            let d = ExprTuple::from(*d);
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

        roll_count > cap
    }
    #[pub_if(mice_semver_exempt)]
    #[derive(Error, Debug)]
    enum CappedRollError {
        #[error("tried to DOS me.")]
        ExceededCap,
        #[error(transparent)]
        Other(#[from] MError),
    }
    #[pub_if(mice_semver_exempt)]
    fn roll_exp_capped(dice: Expression, cap: i64) -> Result<ExpressionResult, CappedRollError> {
        if !exceeds_cap(&dice, cap) {
            // Since we know there's an expression, no `BuildError` will be produced.
            Ok(RollBuilder::new()
                .with_expression(dice)
                .into_roll()
                .unwrap()
                .roll()?)
        } else {
            Err(CappedRollError::ExceededCap)
        }
    }
}
