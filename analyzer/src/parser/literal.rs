use crate::parser::span::LSpan;
use crate::parser::span::Span;
use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::bytes::tag;
use nom::character::complete::alpha1;
use nom::character::complete::alphanumeric1;
use nom::character::complete::digit1;
use nom::combinator::recognize;
use nom::multi::many0;
use nom::multi::many0_count;
use nom::multi::many1;
use nom::multi::many1_count;
use nom::sequence::pair;
use nom::sequence::terminated;
use nom::{
    character::complete::{char, one_of},
    combinator::opt,
    sequence::preceded,
};

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Literal {
    Ident(Span),
    Integer(i64),
    Float(f64),
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Literal::Ident(s) => write!(f, "{s}"),
            Literal::Integer(i) => write!(f, "{i}"),
            Literal::Float(fl) => write!(f, "{fl}"),
        }
    }
}

pub(crate) fn identifier(input: LSpan) -> IResult<LSpan, Span> {
    recognize(pair(
        alt((alpha1, tag("_"))),
        many0_count(alt((alphanumeric1, tag("_")))),
    ))
    .map(|s| Span::new(s))
    .parse(input)
}

pub(crate) fn integer(input: LSpan) -> IResult<LSpan, i64> {
    recognize(many1_count(digit1))
        .map_res(|s: LSpan| s.parse::<i64>())
        .parse(input)
}

fn decimal(input: LSpan) -> IResult<LSpan, LSpan> {
    recognize(many1(terminated(one_of("0123456789"), many0(char('_')))))
        .parse(input)
}

fn float(input: LSpan) -> IResult<LSpan, f64> {
    alt((
        // Case one: .42
        recognize((
            char('.'),
            decimal,
            opt((one_of("eE"), opt(one_of("+-")), decimal)),
        )), // Case two: 42e42 and 42.42e42
        recognize((
            decimal,
            opt(preceded(char('.'), decimal)),
            one_of("eE"),
            opt(one_of("+-")),
            decimal,
        )), // Case three: 42. and 42.42
        recognize((decimal, char('.'), opt(decimal))),
    ))
    .map_res(|s| s.parse::<f64>())
    .parse(input)
}

pub(crate) fn literal<'a>(input: LSpan) -> IResult<LSpan, Literal> {
    alt((
        float.map(|f| Literal::Float(f)),
        integer.map(|i| Literal::Integer(i)),
        identifier.map(|s| Literal::Ident(s)),
    ))
    .parse(input)
}
