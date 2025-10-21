use crate::parser::{
    expression::{Expr, expression},
    literal::identifier,
    span::{Ident, LSpan},
    white_space::ws,
};
use nom::{
    IResult, Parser,
    bytes::tag,
    multi::fold,
    sequence::{separated_pair, terminated},
};

pub(crate) fn equation(input: LSpan) -> IResult<LSpan, (Ident, Expr)> {
    separated_pair(ws(identifier), ws(tag("=")), ws(expression)).parse(input)
}

pub(crate) fn equations(input: LSpan) -> IResult<LSpan, Vec<(Ident, Expr)>> {
    fold(
        0..,
        terminated(ws(equation), ws(tag(";"))),
        // preallocates a vector of the max size
        || Vec::new(),
        |mut acc: Vec<_>, item| {
            acc.push(item);
            acc
        },
    )
    .parse(input)
}
