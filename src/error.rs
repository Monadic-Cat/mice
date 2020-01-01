use crate::parse::ParseError;
use crate::post::{EvaluatedTerm, TResult};
// use std::error::Error as StdError;
use std::ops::Neg;
use thiserror::Error;

/// Most general mice error type. Exported as `MiceError` in the prelude.
#[derive(Debug, Clone, Copy, Error)]
pub enum Error {
    /// This indicates the usage of a die with <= 0 sides
    #[error("Invalid die")]
    InvalidDie,
    /// The sum of terms is greater than what an `i64` can hold
    #[error("sum is too high for `i64`")]
    OverflowPositive,
    /// The sum of terms is lower than what an `i64` can hold
    #[error("sum is too low for `i64`")]
    OverflowNegative,
    /// The expression evaluated isn't a valid dice expression
    #[error("you've specified an invalid dice expression")]
    InvalidExpression,
}
impl Neg for Error {
    type Output = Self;
    fn neg(self) -> Self::Output {
        match self {
            Error::OverflowPositive => Error::OverflowNegative,
            Error::OverflowNegative => Error::OverflowPositive,
            x => x,
        }
    }
}
impl From<ParseError> for Error {
    fn from(e: ParseError) -> Self {
        match e {
            ParseError::InvalidExpression => Error::InvalidExpression,
        }
    }
}

pub(crate) enum MyResult<T, E> {
    Ok(T),
    Err(E),
}

impl<T: Neg<Output = T>, E: Neg<Output = E>> Neg for MyResult<T, E> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        match self {
            MyResult::Ok(x) => MyResult::Ok(-x),
            MyResult::Err(x) => MyResult::Err(-x),
        }
    }
}
type MTResult = MyResult<EvaluatedTerm, Error>;
impl From<MTResult> for TResult {
    fn from(r: MTResult) -> TResult {
        match r {
            MyResult::Ok(x) => Ok(x),
            MyResult::Err(x) => Err(x),
        }
    }
}
impl<T, J, E, R> From<Result<T, E>> for MyResult<J, R>
where
    T: Into<J>,
    E: Into<R>,
{
    fn from(r: Result<T, E>) -> MyResult<J, R> {
        match r {
            Ok(x) => MyResult::Ok(x.into()),
            Err(x) => MyResult::Err(x.into()),
        }
    }
}
