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
        (
            many0(terminated(expression, ws(tag(",")))),
            opt(ws(expression)),
        ),
        ws(tag("]")),
    )
    .map(|(mut x, l)| {
        if let Some(e) = l {
            x.push(e)
        }
        x
    })
    .parse(input)
}

#[cfg(test)]
mod tests {
    use crate::parser::{
        array::array,
        test::{error_test, ok_test},
    };

    #[test]
    fn empty_array() {
        ok_test(array, "[ ]");
        error_test(array, "[( ]");
        ok_test(array, "[ ]");
        ok_test(array, " [   a     ]");
    }
    #[test]
    fn basic_array() {
        ok_test(array, "[12, a, b, 1234]");
        error_test(array, "[12, a b, 1234]");
        ok_test(array, "        [AZAZHJAKJZB ,kdcdkc, kkjndkjd    ]");
        error_test(array, "        [AZAZHJAKJZB ,kdcdkc kkjndkjd    ]");
        error_test(array, "        [[AZAZHJAKJZB ,kdcdkc kkjndkjd    ]");
        ok_test(array, " [   a     ]");
        error_test(array, " [   a )    ]");
    }
    #[test]
    fn complex_array() {
        ok_test(array, "[12, a, b, 1234, ]");
        ok_test(array, "        [AZAZHJAKJZB ,kdcdkc, kkjndkjd ,    ]");
        error_test(array, "        [AZAZHJAKJZB kdcdkc, kkjndkjd ,    ]");
        error_test(array, "        [AZAZHJAKJZB ,kdcdkc, kkjndkjd ,  ,  ]");
    }
}
