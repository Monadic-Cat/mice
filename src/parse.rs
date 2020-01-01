use crate::post::FormatOptions;
use crate::Error;
use thiserror::Error;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    combinator::opt,
    error::ErrorKind::TooLarge,
    multi::many0,
    sequence::tuple,
    Err::Failure,
    IResult,
};
use std::fmt::Display;
use std::fmt::Formatter;
use std::ops::{Mul, Neg};
// use std::collections::HashMap;

#[derive(Debug, Copy, Clone, Error)]
pub(crate) enum ParseError {
    #[error("you've specified an invalid dice expression")]
    InvalidExpression,
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct Die {
    /// Negative numbers of dice are
    /// incorrect, but matching integer
    /// sizes is helpful.
    pub(crate) number: i64,
    /// Negative dice sizes are nonsense,
    /// but matching integer sizes are helpful.
    pub(crate) size: i64,
}
impl Die {
    /// Creation of a `Die` may fail if:
    ///  - number of sides < 1
    ///  - number of dice  < 0
    pub(crate) fn new(number: i64, size: i64) -> Result<Die, Error> {
        // Forbid d0 and below. d1 is weird, but it
        // has a correct interpretation.
        if size < 1 || number < 0 {
            Err(Error::InvalidDie)
        } else {
            Ok(Die { number, size })
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum Term {
    Die(Die),
    Constant(i64),
}
impl Display for Term {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Term::Die(x) => write!(f, "{}d{}", x.number, x.size),
            Term::Constant(x) => write!(f, "{}", x),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum Sign {
    Positive,
    Negative,
}
impl Neg for Sign {
    type Output = Sign;
    fn neg(self) -> Self::Output {
        match self {
            Sign::Positive => Sign::Negative,
            Sign::Negative => Sign::Positive,
        }
    }
}
impl<T: Neg<Output = T>> Mul<T> for Sign {
    type Output = T;
    fn mul(self, rhs: T) -> Self::Output {
        match self {
            Sign::Positive => rhs,
            Sign::Negative => -rhs,
        }
    }
}
impl Display for Sign {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Sign::Positive => "+",
                Sign::Negative => "-",
            }
        )
    }
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct Expr {
    pub(crate) term: Term,
    pub(crate) sign: Sign,
}
impl Expr {
    pub(crate) fn format(&self, options: FormatOptions) -> String {
        // N
        // -N
        // NdN
        // -NdN
        let mut nstr = String::new();
        let FormatOptions { ignore_sign, .. } = options;
        if !ignore_sign {
            match self.sign {
                Sign::Positive => (),
                Sign::Negative => nstr.push_str("-"),
            }
        }
        nstr.push_str(&format!("{}", self.term));
        nstr
    }
}
impl Display for Expr {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.format(FormatOptions::new()))
    }
}

pub(crate) type Expression = Vec<Expr>;

fn is_dec_digit(c: char) -> bool {
    c.is_digit(10)
}

fn integer(input: &str) -> IResult<&str, i64> {
    let (input, int) = take_while1(is_dec_digit)(input)?;
    // Pretend to be a 63 bit unsigned integer.
    let i = match int.parse::<i64>() {
        // The only error possible here is
        // integer overflow.
        // This should emit a nom Failure
        Err(_) => return Err(Failure((input, TooLarge))),
        Ok(x) => x,
    };
    Ok((input, i))
}

fn die(input: &str) -> IResult<&str, Term> {
    // number of dice : [integer]
    // separator      : "d"
    // size of dice   : integer
    let (input, (number, _, size)) = tuple((opt(integer), tag("d"), integer))(input)?;
    let number = number.unwrap_or(1);
    Ok((input, Term::Die(Die { number, size })))
}

fn addition(input: &str) -> IResult<&str, Sign> {
    let (input, _) = tag("+")(input)?;
    Ok((input, Sign::Positive))
}
fn subtraction(input: &str) -> IResult<&str, Sign> {
    let (input, _) = tag("-")(input)?;
    Ok((input, Sign::Negative))
}

fn operator(input: &str) -> IResult<&str, Sign> {
    alt((addition, subtraction))(input)
}

fn whitespace(input: &str) -> IResult<&str, &str> {
    alt((tag(" "), tag("\t")))(input)
}

fn separator(input: &str) -> IResult<&str, Sign> {
    let (input, t) = tuple((many0(whitespace), operator, many0(whitespace)))(input)?;
    Ok((input, t.1))
}

fn constant(input: &str) -> IResult<&str, Term> {
    let i = integer(input)?;
    Ok((i.0, Term::Constant(i.1)))
}

// /// Use like this, where map is a HashMap: `|x| variable(map, x)`
// fn variable<'a>(dict: HashMap<&str, i64>, input: &'a str) -> IResult<&'a str, Term> {
//     let (input, id) = take_while1(|c: char| c.is_alphabetic())(input)?;
//     let v = dict.get(id)?;
//     Ok((input, Term::Constant(v)))
// }

fn term(input: &str) -> IResult<&str, Term> {
    alt((die, constant))(input)
}

fn dice(input: &str) -> IResult<&str, Expression> {
    // [(+/-)] die ((+/-) die)*
    let (input, (sign, term, terms)) =
        tuple((opt(separator), term, many0(tuple((separator, term)))))(input)?;
    let sign = sign.unwrap_or(Sign::Positive);
    let mut expression = vec![Expr { term, sign }];
    for (sign, term) in terms {
        expression.push(Expr { term, sign });
    }
    Ok((input, expression))
}

/// Wrap up getting errors from parsing a dice expression.
pub(crate) fn wrap_dice(input: &str) -> Result<Expression, ParseError> {
    let (input, e) = match dice(input.trim()) {
        Ok(x) => x,
        Err(_) => return Err(ParseError::InvalidExpression),
    };
    // Prevent weirdness like "10dlol" => 10
    if !input.is_empty() {
        Err(ParseError::InvalidExpression)
    } else {
        Ok(e)
    }
}
