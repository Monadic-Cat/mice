use crate::parse::ParseError;
use crate::post::{EvaluatedTerm, TResult};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::ops::Neg;

/// Most general mice error type.
#[derive(Debug, Clone, Copy)]
pub enum RollError {
    /// This indicates the usage of a die with <= 0 sides
    InvalidDie,
    /// The sum of terms is greater than what an `i64` can hold
    OverflowPositive,
    /// The sum of terms is lower than what an `i64` can hold
    OverflowNegative,
    /// The expression evaluated isn't a valid dice expression
    InvalidExpression,
}
impl Neg for RollError {
    type Output = Self;
    fn neg(self) -> Self::Output {
        match self {
            RollError::OverflowPositive => RollError::OverflowNegative,
            RollError::OverflowNegative => RollError::OverflowPositive,
            x => x,
        }
    }
}
impl From<ParseError> for RollError {
    fn from(e: ParseError) -> Self {
        match e {
            ParseError::InvalidExpression => RollError::InvalidExpression,
        }
    }
}

impl Display for RollError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            RollError::InvalidDie => write!(f, "Invalid die"),
            RollError::OverflowPositive => write!(f, "sum is too high for `i64`"),
            RollError::OverflowNegative => write!(f, "sum is too low for `i64`"),
            RollError::InvalidExpression => {
                write!(f, "you've specified an invalid dice expression")
            }
        }
    }
}
impl Error for RollError {}

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
type MTResult = MyResult<EvaluatedTerm, RollError>;
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
