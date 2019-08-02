use crate::RollError;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    combinator::opt,
    multi::many0,
    sequence::tuple,
    IResult,
};

#[derive(Debug, Copy, Clone)]
pub(crate) enum ParseError {
    InvalidExpression,
}
impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParseError::InvalidExpression => {
                write!(f, "you've specified an invalid dice expression.")
            }
        }
    }
}
impl std::error::Error for ParseError {}

#[derive(Debug, Copy, Clone)]
pub(crate) struct Die {
    pub(crate) number: u64,
    pub(crate) size: u64,
}
impl Die {
    #[allow(dead_code)]
    fn new(number: u64, size: u64) -> Result<Die, RollError> {
        // u64 type constraint means
        // we don't need to check if number < 0
        // Forbid d0. d1 is weird, but it
        // has a correct interpretation.
        if size < 1 {
            Err(RollError::InvalidDie)
        } else {
            Ok(Die { number, size })
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum Term {
    Die(Die),
    Constant(u64),
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum Sign {
    Positive,
    Negative,
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct Expr {
    pub(crate) term: Term,
    pub(crate) sign: Sign,
}

pub(crate) type Expression = Vec<Expr>;

fn is_dec_digit(c: char) -> bool {
    c.is_digit(10)
}
fn integer(input: &str) -> IResult<&str, u64> {
    let (input, int) = take_while1(is_dec_digit)(input)?;
    // Pretend to be a 63 bit unsigned integer.
    let i = match int.parse::<i64>() {
        // The only error possible here is
        // integer overflow.
        // This should emit a nom Failure
        Err(_) => {
            return Err(nom::Err::<(&str, nom::error::ErrorKind)>::Failure((
                input,
                nom::error::ErrorKind::TooLarge,
            )))
        }
        Ok(x) => x as u64,
    };
    Ok((input, i))
}

fn die(input: &str) -> IResult<&str, Term> {
    // number of dice : [integer]
    // separator      : "d"
    // size of dice   : integer
    let (input, d) = tuple((opt(integer), tag("d"), integer))(input)?;
    Ok((
        input,
        Term::Die(Die {
            number: match d.0 {
                Some(x) => x,
                None => 1,
            },
            size: d.2,
        }),
    ))
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

fn term(input: &str) -> IResult<&str, Term> {
    alt((die, constant))(input)
}

fn dice(input: &str) -> IResult<&str, Expression> {
    // [(+/-)] die ((+/-) die)*
    let (input, s) = tuple((opt(separator), term, many0(tuple((separator, term)))))(input)?;
    let mut expression = vec![Expr {
        term: s.1,
        sign: match s.0 {
            Some(x) => x,
            None => Sign::Positive,
        },
    }];
    for t in s.2 {
        expression.push(Expr {
            term: t.1,
            sign: t.0,
        });
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
