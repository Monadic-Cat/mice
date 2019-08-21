use crate::{
    parse::{wrap_dice, Expression, ParseError},
    roll_expr_iter_with, EResult,
};
use std::error::Error;

use rand::{thread_rng, RngCore};
#[derive(Debug)]
pub(crate) enum BuildError {
    NoExpression,
}
impl std::fmt::Display for BuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "eh")
    }
}
impl Error for BuildError {}

pub(crate) struct RollBuilder {
    expression: Option<Expression>,
    generator: Option<Box<RngCore>>,
}
impl RollBuilder {
    pub fn new() -> RollBuilder {
        RollBuilder {
            expression: None,
            generator: None,
        }
    }
    pub fn parse(mut self, input: &str) -> Result<RollBuilder, ParseError> {
        let expression = wrap_dice(input)?;
        self.expression = Some(expression);
        Ok(self)
    }
    pub fn with_expression(mut self, expression: crate::parse::Expression) -> RollBuilder {
        self.expression = Some(expression);
        self
    }
    /// Pay specific attention to this when targeting WASM.
    /// Until thread local storage is supported, the default
    /// RNG will not work- provide your own.
    pub fn with_rng(mut self, rng: Box<RngCore>) -> RollBuilder {
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

pub(crate) struct Roll {
    expression: Expression,
    generator: Box<RngCore>,
}

impl Roll {
    pub fn roll(&mut self) -> EResult {
        roll_expr_iter_with(&mut self.generator, self.expression.iter().copied())
    }
}
