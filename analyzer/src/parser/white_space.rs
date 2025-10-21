use nom::sequence::delimited;
use nom::{Parser, character::complete::multispace0, error::ParseError};

use crate::parser::span::LSpan;


/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
pub(crate) fn ws<'a, O, E: ParseError<LSpan<'a>>, F>(
    inner: F,
) -> impl Parser<LSpan<'a>, Output = O, Error = E>
where
    F: Parser<LSpan<'a>, Output = O, Error = E>,
{
    delimited(multispace0, inner, multispace0)
}
