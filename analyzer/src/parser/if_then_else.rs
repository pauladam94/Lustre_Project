use crate::parser::{
    expression::{Expr, expression},
    span::LSpan,
    white_space::ws,
};
use nom::{IResult, Parser, bytes::tag};

pub(crate) fn ifthenelse(input: LSpan) -> IResult<LSpan, (Expr, Expr, Expr)> {
    (
        ws(tag("if")),
        ws(expression),
        ws(tag("then")),
        ws(expression),
        ws(tag("else")),
        ws(expression),
    )
        .map(|(_, cond, _, yes, _, no)| (cond, yes, no))
        .parse(input)
}
