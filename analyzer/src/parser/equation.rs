use crate::parser::{
    expression::{Expr, expression},
    literal::identifier,
    span::{Ident, LSpan},
    white_space::ws,
};
use nom::{
    IResult, Parser,
    bytes::tag,
    multi::{fold, many0},
    sequence::{separated_pair, terminated},
};

pub(crate) fn equation(input: LSpan) -> IResult<LSpan, (Ident, Expr)> {
    separated_pair(ws(identifier), ws(tag("=")), ws(expression)).parse(input)
}

pub(crate) fn equations(input: LSpan) -> IResult<LSpan, Vec<(Ident, Expr)>> {
    many0(terminated(ws(equation), ws(tag(";")))).parse(input)
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
        error_test(equations, "x=5");
    }
}
