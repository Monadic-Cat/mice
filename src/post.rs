use crate::error::RollError;
use crate::parse::{Expr, Sign, Term};
use std::fmt::{Display, Formatter};
use std::ops::Neg;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

pub(crate) type TResult = Result<EvaluatedTerm, RollError>;
pub(crate) type EResult = Result<ExpressionResult, RollError>;
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Debug, Clone)]
pub struct ExpressionResult {
    /// Private field because `Expr`'s layout isn't final.
    pairs: Vec<(Expr, EvaluatedTerm)>,
    total: i64,
}
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl ExpressionResult {
    pub fn total(&self) -> i64 {
        self.total
    }
    #[cfg(target_arch = "wasm32")]
    pub fn display(&self) -> String {
        format!("{}", self)
    }
}
impl ExpressionResult {
    pub(crate) fn new(pairs: Vec<(Expr, EvaluatedTerm)>, total: i64) -> Self {
        Self { pairs, total }
    }
}

impl Display for ExpressionResult {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let mut nstr = self.total().to_string();
        if self.pairs.len() > 1 {
            nstr.push_str(" = (");
            let mut iter = self.pairs.iter();
            // Keep unwrap local so I can see *why* it's safe.
            // It will be easier to remove later if I change
            // the above.
            let first = iter.next().unwrap();
            let form = |prior: Expr, val: &EvaluatedTerm| match prior.term {
                Term::Constant(_) => format!("{}", val),
                Term::Die(_) => format!("{} → {}", prior, val),
            };
            nstr.push_str(&form(first.0, &first.1));
            for x in iter {
                nstr.push_str(&format!(", {}", form(x.0, &x.1)));
            }
            nstr.push_str(")");
        }
        write!(f, "{}", nstr)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct RolledDie {
    pub(crate) total: i64,
    pub(crate) parts: Vec<i64>,
    pub(crate) sign_part: Sign,
}
impl Neg for RolledDie {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            total: -self.total,
            sign_part: -self.sign_part,
            parts: self.parts,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum EvaluatedTerm {
    Die(RolledDie),
    Constant(i64),
}
impl EvaluatedTerm {
    pub(crate) fn value(&self) -> i64 {
        match self {
            EvaluatedTerm::Die(x) => x.total,
            EvaluatedTerm::Constant(x) => *x,
        }
    }
}
impl Neg for EvaluatedTerm {
    type Output = Self;
    fn neg(self) -> Self::Output {
        match self {
            EvaluatedTerm::Die(x) => EvaluatedTerm::Die(-x),
            EvaluatedTerm::Constant(x) => EvaluatedTerm::Constant(-x),
        }
    }
}
impl From<RolledDie> for EvaluatedTerm {
    fn from(d: RolledDie) -> EvaluatedTerm {
        EvaluatedTerm::Die(d)
    }
}
impl Display for EvaluatedTerm {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}
