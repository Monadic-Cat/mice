/// Utilities for exposing crate internals to users
/// in a safe and stable fashion.
use std::convert::TryFrom;

use crate::{
    builder::RollBuilder,
    error::Error,
    parse::{wrap_dice, Die, Expr, Sign, Term},
    post::EResult,
};
pub(crate) type ExprTuple = (i64, i64);

impl TryFrom<ExprTuple> for Expr {
    type Error = Error;
    fn try_from(tup: ExprTuple) -> Result<Self, Error> {
        let (mut n, s) = tup;
        let sign = if n < 0 {
            n = -n;
            Sign::Negative
        } else {
            Sign::Positive
        };
        Ok(Self {
            term: if s > 1 {
                Term::Die(Die::new(n, s)?)
            } else if s == 1 {
                Term::Constant(n)
            } else {
                return Err(Error::InvalidDie);
            },
            sign,
        })
    }
}
impl From<Expr> for ExprTuple {
    fn from(e: Expr) -> ExprTuple {
        let t = match e.term {
            Term::Die(x) => (x.number, x.size),
            Term::Constant(x) => (x, 1),
        };
        match e.sign {
            Sign::Positive => t,
            Sign::Negative => (-t.0, t.1),
        }
    }
}

/// Get a `Vec` of tuples of the form:
/// (number of dice, number of faces)
///
/// Constant terms are expressed in the form: (value, 1)
///
/// There is no guarantee of the order of terms.
///
/// The only possible error here is `Error::InvalidExpression`.
/// Other errors may be encountered in this function's complement:
/// `roll_tuples`.
pub fn tuple_vec(input: &str) -> Result<Vec<ExprTuple>, Error> {
    let e = wrap_dice(input)?;
    Ok(e.into_iter().map(|x| x.into()).collect())
}
/// Roll and sum a slice of tuples, in the form
/// provided by this function's complement: `tuple_vec`
#[cfg(not(target_arch = "wasm32"))]
pub fn roll_tuples(input: &[ExprTuple]) -> EResult {
    Ok(RollBuilder::new().with_tuples(input)?.into_roll()?.roll()?)
}
