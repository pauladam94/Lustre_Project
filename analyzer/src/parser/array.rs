use crate::parser::{
    expression::{Expr, expression},
    span::LSpan,
    white_space::ws,
};
use nom::Parser;
use nom::{
    IResult,
    bytes::tag,
    combinator::opt,
    multi::many0,
    sequence::{delimited, terminated},
};

pub(crate) fn array(input: LSpan) -> IResult<LSpan, Vec<Expr>> {
    delimited(
        ws(tag("[")),
        terminated(
            (many0(terminated(expression, ws(tag(",")))), ws(expression)),
            opt(ws(tag(","))),
        ),
        ws(tag("]")),
    )
    .map(|(mut x, l)| {
        x.push(l);
        x
    })
    .parse(input)
}
