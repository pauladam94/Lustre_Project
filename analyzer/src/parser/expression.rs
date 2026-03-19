use crate::{
    ast::{
        binop::BinOp,
        expression::{Expr, Precedence},
        literal::Value,
        unary_op::UnaryOp,
    },
    parser::{
        array::array,
        func_call::func_call,
        if_then_else::ifthenelse,
        literal::{identifier, literal},
        span::{LSpan, Span},
        tuple::tuple,
        white_space::ws,
    },
};
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    combinator::{fail, map, opt, value},
    sequence::delimited,
};
use nom_language::precedence::{Assoc, Operation, binary_op, precedence, unary_op};

fn operand(input: LSpan) -> IResult<LSpan, Expr> {
    map(
        (
            alt((
                // value(Expr::Lit(Value::Unit), (ws(tag("(")), ws(tag(")")))),
                delimited(ws(tag("(")), ws(expression), ws(tag(")"))),
                map(array, Expr::Array),
                map(tuple, Expr::Tuple),
                map(ifthenelse, |(cond, yes, no)| Expr::If {
                    cond: Box::new(cond),
                    yes: Box::new(yes),
                    no: Box::new(no),
                }),
                map(func_call, |(name, args)| Expr::FCall { name, args }),
                map(ws(literal), Expr::Lit),
                map(ws(identifier), Expr::Variable),
            )),
            opt(delimited(ws(tag("[")), expression, ws(tag("]")))),
        ),
        |(expr, i)| match i {
            Some(index) => Expr::Index {
                expr: Box::new(expr),
                index: Box::new(index),
            },
            None => expr,
        },
    )
    .parse(input)
}
// todo parse expr[expr] for index stuff
pub(crate) fn expression(input: LSpan) -> IResult<LSpan, Expr> {
    use BinOp::*;

    precedence(
        alt((
            unary_op(UnaryOp::Inv.precedence(), ws(tag("-"))),
            unary_op(UnaryOp::Pre.precedence(), ws(tag("pre"))),
            unary_op(UnaryOp::Not.precedence(), ws(tag("not"))),
        )),
        fail(),
        alt((
            binary_op(Mult.precedence(), Assoc::Left, ws(tag("*"))),
            binary_op(Div.precedence(), Assoc::Left, ws(tag("/"))),
            // TODO Assoc::Right WARNING pretty printing expression
            binary_op(Arrow.precedence(), Assoc::Left, ws(tag("->"))),
            binary_op(Add.precedence(), Assoc::Left, ws(tag("+"))),
            binary_op(Sub.precedence(), Assoc::Left, ws(tag("-"))),
            binary_op(Fby.precedence(), Assoc::Left, ws(tag("fby"))),
            binary_op(Eq.precedence(), Assoc::Left, ws(tag("=="))),
            binary_op(Neq.precedence(), Assoc::Left, ws(tag("!="))),
            binary_op(Or.precedence(), Assoc::Left, ws(tag("or"))),
            binary_op(And.precedence(), Assoc::Left, ws(tag("and"))),
            binary_op(Caret.precedence(), Assoc::Left, ws(tag("^"))),
        )),
        operand,
        |op: Operation<LSpan, LSpan, LSpan, Expr>| {
            use nom_language::precedence::Operation::*;
            match op {
                Binary(lhs, op, rhs) => {
                    use BinOp::*;
                    let bin_op = match *op.fragment() {
                        "*" => Mult,
                        "+" => Add,
                        "/" => Div,
                        "-" => Sub,
                        "->" => Arrow,
                        "fby" => Fby,
                        "==" => Eq,
                        "!=" => Neq,
                        "or" => Or,
                        "and" => And,
                        "^" => Caret,
                        _ => return Err("Non supported binary operation"),
                    };
                    Ok(Expr::BinOp {
                        lhs: Box::new(lhs),
                        op: bin_op,
                        span_op: Span::from_lspan(op),
                        rhs: Box::new(rhs),
                    })
                }
                Prefix(op, rhs) => {
                    use UnaryOp::*;
                    let unary_op = match *op.fragment() {
                        "pre" => Pre,
                        "-" => Inv,
                        "not" => Not,
                        _ => return Err("Non supported unary operator"),
                    };
                    Ok(Expr::UnaryOp {
                        op: unary_op,
                        span_op: Span::from_lspan(op),
                        rhs: Box::new(rhs),
                    })
                }
                _ => Err("Invalid combination"),
            }
        },
    )(input)
}

#[cfg(test)]
mod tests {
    use crate::parser::{
        expression::expression,
        test::{error_test, ok_test},
    };

    #[test]
    fn basic_addition() {
        ok_test(expression, "a + 2");
        ok_test(expression, " abc + 2");
        error_test(expression, "a + ");
    }
}
