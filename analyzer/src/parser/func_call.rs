use crate::parser::{
    expression::{Expr, expression},
    literal::{Value, identifier},
    span::{LSpan, Span},
    white_space::ws,
};
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::tag,
    combinator::{opt, recognize, value},
    multi::many0,
    sequence::{delimited, terminated},
};

pub(crate) fn func_call(input: LSpan) -> IResult<LSpan, (Span, Vec<Expr>)> {
    (
        ws(identifier),
        alt((
            value(
                vec![Expr::Lit(Value::Unit)],
                recognize((ws(tag("(")), ws(tag(")")))),
            ),
            delimited(
                ws(tag("(")),
                (
                    many0(terminated(ws(expression), ws(tag(",")))),
                    opt(ws(expression)),
                )
                    .map(|(mut v, e)| {
                        // TODO
                        // Add unit as empty argument
                        // if v.is_empty() {
                        //     vec![]
                        // } else {
                        // }
                        match e {
                            Some(e) => {
                                v.push(e);
                                v
                            }
                            None => v,
                        }
                    }),
                ws(tag(")")),
            ),
        )),
    )
        .map(|(s, args)| (s, args))
        .parse(input)
}

#[cfg(test)]
mod tests {
    use crate::parser::{
        func_call::func_call,
        test::{error_test, ok_test},
    };

    #[test]
    fn empty_call() {
        ok_test(func_call, "f()");
        ok_test(func_call, " f ( ) ");
        error_test(func_call, " f (  ");
        error_test(func_call, " f  ) ");
    }
    #[test]
    fn two_integers_call() {
        ok_test(func_call, "f(1, 2)");
        ok_test(func_call, " f (3345 , 2 ) ");
        error_test(func_call, " f (2, 1234  ");
        error_test(func_call, " f 234, 123 ) ");
    }
}
