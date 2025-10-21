use crate::parser::{
    expression::{Expr, expression},
    literal::identifier,
    span::{LSpan, Span},
    white_space::ws,
};
use nom::{
    IResult, Parser, bytes::tag, combinator::opt, multi::many0,
    sequence::terminated,
};

pub(crate) fn func_call(input: LSpan) -> IResult<LSpan, (Span, Vec<Expr>)> {
    (
        ws(identifier),
        ws(tag("(")),
        (
            many0(terminated(ws(expression), ws(tag(",")))),
            opt(ws(expression)),
        )
            .map(|(mut v, e)| match e {
                Some(e) => {
                    v.push(e);
                    v
                }
                None => v,
            }),
        ws(tag(")")),
    )
        .map(|(s, _, args, _)| (s, args))
        .parse(input)
}
