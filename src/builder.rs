use crate::{
    parse::{wrap_dice, Expr, Expression},
    roll_expr_iter_with, EResult, ExprTuple, RollError,
};
use std::convert::TryFrom;
use std::error::Error;

use rand::{thread_rng, RngCore};
#[derive(Debug)]
pub enum BuildError {
    NoExpression,
}
impl std::fmt::Display for BuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "eh")
    }
}
impl Error for BuildError {}

impl From<BuildError> for RollError {
    fn from(e: BuildError) -> RollError {
        match e {
            BuildError::NoExpression => RollError::InvalidExpression,
        }
    }
}

pub struct RollBuilder {
    expression: Option<Expression>,
    generator: Option<Box<dyn RngCore>>,
}
impl RollBuilder {
    pub fn new() -> RollBuilder {
        RollBuilder {
            expression: None,
            generator: None,
        }
    }
    pub fn parse(mut self, input: &str) -> Result<RollBuilder, RollError> {
        let expression = wrap_dice(input)?;
        self.expression = Some(expression);
        Ok(self)
    }
    pub fn with_tuples(mut self, tuples: &[ExprTuple]) -> Result<RollBuilder, RollError> {
        let mut expression = Vec::new();
        for x in tuples {
            expression.push(Expr::try_from(*x)?)
        }
        self.expression = Some(expression);
        Ok(self)
    }
    #[allow(dead_code)]
    pub(crate) fn with_expression(mut self, expression: Expression) -> RollBuilder {
        self.expression = Some(expression);
        self
    }
    /// Pay specific attention to this when targeting WASM.
    /// Until thread local storage is supported, the default
    /// RNG will not work- provide your own.
    #[allow(dead_code)]
    pub fn with_rng(mut self, rng: Box<dyn RngCore>) -> RollBuilder {
        self.generator = Some(rng);
        self
    }
    pub fn into_roll(self) -> Result<Roll, BuildError> {
        Ok(Roll {
            expression: match self.expression {
                Some(x) => x,
                None => return Err(BuildError::NoExpression),
            },
            generator: match self.generator {
                Some(x) => x,
                None => Box::new(thread_rng()),
            },
        })
    }
}

pub struct Roll {
    expression: Expression,
    generator: Box<dyn RngCore>,
}

impl Roll {
    pub fn roll(&mut self) -> EResult {
        roll_expr_iter_with(&mut self.generator, self.expression.iter().copied())
    }
}
