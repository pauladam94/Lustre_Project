use crate::parser::span::LSpan;
use nom::IResult;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_till, take_until};
use nom::character::complete::anychar;
use nom::combinator::value;
use nom::multi::many_till;
use nom::sequence::delimited;
use nom::{Parser, character::complete::multispace0, error::ParseError};

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

// pub(crate) fn ws_lustre(mut input: LSpan) -> IResult<LSpan, ()> {
//     let mut depth = 0;
//     (input, _) = multispace0(input)?;
//     loop {
//         match alt((
//             value(-1, tag("*)")),
//             value(1, tag("(*"))
//         )).parse(input) {
//             Ok((new_input, val)) => {
//                 depth += val;
//                 if depth == 0 {
//                     break;
//                 }
//             }
//             Err(_) => {
//                 (input, _) = anychar(input)?;
//             }
//         }
//     }
//     value((), multispace0).parse(input)
// }
