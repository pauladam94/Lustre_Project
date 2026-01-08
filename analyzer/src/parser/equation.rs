use crate::{
    ast::expression::Expr,
    parser::{
        expression::expression,
        literal::identifier,
        span::{Ident, LSpan, Span},
        white_space::ws,
    },
};
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    combinator::opt,
    multi::{fold, many0},
    sequence::{delimited, separated_pair, terminated},
};

pub(crate) fn equation(input: LSpan) -> IResult<LSpan, (Vec<Ident>, Expr)> {
    separated_pair(
        alt((
            delimited(
                ws(tag("(")),
                (
                    many0(terminated(identifier, ws(tag(",")))),
                    opt(ws(identifier)),
                )
                    .map(|(mut v, s)| {
                        if let Some(s) = s {
                            v.push(s);
                        }
                        v
                    }),
                ws(tag(")")),
            ),
            ws(identifier.map(|ident| vec![ident])),
        )),
        ws(tag("=")),
        ws(expression),
    )
    .parse(input)
}

pub(crate) fn equations(input: LSpan) -> IResult<LSpan, (Vec<(Vec<Ident>, Expr)>, Vec<Span>)> {
    fold(
        0..,
        (ws(equation), ws(tag(";"))),
        || (Vec::new(), Vec::new()),
        |(mut acc, mut acc_span), ((name, expr), semi_colon_span)| {
            acc.push((name, expr));
            acc_span.push(Span::new(semi_colon_span));
            (acc, acc_span)
        },
    )
    .parse(input)
}

#[cfg(test)]
mod tests {
    use crate::parser::{equation::equations, test::ok_test};

    #[test]
    fn one_equation() {
        ok_test(
            equations,
            "x=5;
            ",
        );
    }
    #[test]
    fn no_equation() {
        ok_test(equations, "");
    }
    #[test]
    fn no_end_comma() {
        // does not manage to parse but does not crash !
        ok_test(equations, "x=5");
    }
}
