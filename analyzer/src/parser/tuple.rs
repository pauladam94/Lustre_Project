use crate::parser::{
    expression::{Expr, expression},
    span::LSpan,
    white_space::ws,
};
use nom::{
    IResult, Parser,
    bytes::tag,
    combinator::opt,
    multi::many1,
    sequence::{delimited, terminated},
};

pub(crate) fn tuple(input: LSpan) -> IResult<LSpan, Vec<Expr>> {
    delimited(
        ws(tag("(")),
        (
            many1(terminated(expression, ws(tag(",")))),
            opt(ws(expression)),
        ),
        ws(tag(")")),
    )
    .map(|(mut x, l)| {
        if let Some(e) = l {
            x.push(e)
        }
        x
    })
    .parse(input)
}
