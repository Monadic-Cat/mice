use crate::error::RollError;
use crate::parse::{Expr, Sign, Term};
use std::fmt::{Display, Formatter};
use std::ops::Neg;
use std::slice;
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
    pub(crate) fn pairs(&self) -> &Vec<(Expr, EvaluatedTerm)> {
        &self.pairs
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

/// Formatting options for dice expressions.
/// All the useful tweaks and such.
/// There may be crate internal fields.
pub(crate) struct FormatOptions {
    pub(crate) ignore_sign: bool,
}
impl FormatOptions {
    pub(crate) fn new() -> FormatOptions {
        FormatOptions { ignore_sign: false }
    }
    pub(crate) fn exclude_sign(mut self) -> Self {
        self.ignore_sign = true;
        self
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
impl RolledDie {
    fn format(&self, options: &FormatOptions) -> String {
        if self.parts.len() > 1 {
            let mut iter = self.parts.iter();
            let first_sign;
            let FormatOptions { ignore_sign } = options;
            if !ignore_sign {
                first_sign = match self.sign_part {
                    Sign::Positive => "",
                    Sign::Negative => "-",
                };
            } else {
                first_sign = "";
            }
            let mut nstr = format!(
                "{}{}",
                if !ignore_sign { first_sign } else { "" },
                iter.next().unwrap()
            );
            let sign_part = if !ignore_sign {
                self.sign_part
            } else {
                Sign::Positive
            };
            for x in iter {
                nstr.push_str(&format!(" {} {}", sign_part, x))
            }
            // nstr.push_str(&format!(" = {}", self.total));
            nstr
        } else {
            format!("{}", self.total)
        }
    }
}
impl Display for RolledDie {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.format(&FormatOptions::new()))
    }
}

#[derive(Debug, Clone)]
pub(crate) enum EvaluatedTerm {
    Die(RolledDie),
    Constant(i64),
}
fn format_i64(s: i64, options: &FormatOptions) -> String {
    let FormatOptions { ignore_sign } = options;
    if *ignore_sign {
        format!("{}", if s > 0 { s } else { -s })
    } else {
        format!("{}", s)
    }
}
impl EvaluatedTerm {
    pub(crate) fn value(&self) -> i64 {
        match self {
            EvaluatedTerm::Die(x) => x.total,
            EvaluatedTerm::Constant(x) => *x,
        }
    }
    pub(crate) fn parts(&self) -> &[i64] {
        match self {
            EvaluatedTerm::Die(x) => &x.parts,
            EvaluatedTerm::Constant(x) => slice::from_ref(x),
        }
    }
    pub(crate) fn sign(&self) -> Sign {
        match self {
            EvaluatedTerm::Die(x) => x.sign_part,
            EvaluatedTerm::Constant(x) => {
                if *x >= 0 {
                    Sign::Positive
                } else {
                    Sign::Negative
                }
            }
        }
    }
    pub(crate) fn format(&self, options: &FormatOptions) -> String {
        match self {
            EvaluatedTerm::Die(x) => x.format(options),
            EvaluatedTerm::Constant(x) => format_i64(*x, options),
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
        // write!(f, "{}", self.value())
        match self {
            EvaluatedTerm::Die(x) => write!(f, "{}", x),
            EvaluatedTerm::Constant(x) => write!(f, "{}", x),
        }
    }
}
