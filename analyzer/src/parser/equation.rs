use crate::parser::{
    expression::{Expr, expression},
    literal::identifier,
    span::{Ident, LSpan, Span},
    white_space::ws,
};
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    multi::{fold, many0},
    sequence::{separated_pair, terminated},
};

pub(crate) fn equation(input: LSpan) -> IResult<LSpan, (Ident, Expr)> {
    separated_pair(
        // ws(complete(identifier)),
        // ws(complete(tag("="))),
        ws(identifier),
        ws(tag("=")),
        ws(expression),
    )
    .parse(input)
}

pub(crate) fn equations(input: LSpan) -> IResult<LSpan, (Vec<(Ident, Expr)>, Vec<Span>)> {
    fold(
        0..,
        (ws(equation), ws(tag(";"))),
        || return (Vec::new(), Vec::new()),
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
    use crate::parser::{
        equation::equations,
        test::{error_test, ok_test},
    };

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
