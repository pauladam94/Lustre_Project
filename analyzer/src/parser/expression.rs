use crate::parser::array::array;
use crate::parser::binop::BinOp;
use crate::parser::func_call::func_call;
use crate::parser::literal::Value;
use crate::parser::literal::identifier;
use crate::parser::literal::literal;
use crate::parser::span::Ident;
use crate::parser::span::LSpan;
use crate::parser::span::Span;
use crate::parser::unary_op::UnaryOp;
use crate::parser::white_space::ws;
use nom::IResult;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::fail;
use nom::combinator::map;
use nom::sequence::delimited;
use nom_language::precedence::Assoc;
use nom_language::precedence::Operation;
use nom_language::precedence::binary_op;
use nom_language::precedence::precedence;
use nom_language::precedence::unary_op;

pub(crate) trait Precedence {
    fn precedence(&self) -> usize;
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Expr {
    BinOp {
        lhs: Box<Expr>,
        op: BinOp,
        span_op: Span,
        rhs: Box<Expr>,
    },
    UnaryOp {
        op: UnaryOp,
        rhs: Box<Expr>,
    },
    Array(Vec<Expr>),
    FCall {
        name: Ident,
        args: Vec<Expr>,
    },
    Variable(Span),
    Lit(Value),
}

impl Expr {
    pub fn is_value(&self) -> bool {
        true
    }
    pub fn get_value(&self) -> Option<Value> {
        match self {
            Expr::BinOp { .. } | Expr::UnaryOp { .. } | Expr::FCall { .. } | Expr::Variable(_) => {
                None
            }
            Expr::Array(exprs) => {
                let mut const_exprs = vec![];
                for expr in exprs.iter() {
                    match expr.get_value() {
                        Some(value_expr) => {
                            const_exprs.push(value_expr)
                        }
                        None => return None,
                    }
                }
                return Some(Value::Array(const_exprs));
            }
            Expr::Lit(lit) => Some(lit.clone()),
        }
    }

    fn fmt_parent(
        &self,
        f: &mut std::fmt::Formatter,
        parent_op: Option<BinOp>,
    ) -> std::fmt::Result {
        match self {
            Expr::BinOp {
                lhs,
                op,
                span_op: _,
                rhs,
            } => match parent_op {
                Some(parent_op) => {
                    if parent_op.precedence() < op.precedence() {
                        write!(f, "(")?;
                    }

                    lhs.fmt_parent(f, Some(*op))?;
                    write!(f, " {} ", op)?;
                    rhs.fmt_parent(f, Some(*op))?;

                    if parent_op.precedence() < op.precedence() {
                        write!(f, ")")?;
                    }
                    Ok(())
                }
                None => {
                    lhs.fmt_parent(f, Some(*op))?;
                    write!(f, " {} ", op)?;
                    rhs.fmt_parent(f, Some(*op))
                }
            },
            Expr::Variable(s) => write!(f, "{}", s),
            Expr::UnaryOp { op, rhs } => {
                write!(f, "{op} {rhs}")
            }
            Expr::Lit(lt) => {
                write!(f, "{lt}")
            }
            Expr::FCall { name, args } => {
                write!(f, "{}(", name)?;
                for (i, arg) in args.iter().enumerate() {
                    write!(f, "{}", arg)?;
                    if i != args.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, ")")
            }
            Expr::Array(v) => {
                write!(f, "[")?;
                for (i, expr) in v.iter().enumerate() {
                    write!(f, "{expr}")?;
                    if i != v.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")
            }
        }
    }
}
impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.fmt_parent(f, None)
    }
}

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
            binary_op(Arrow.precedence(), Assoc::Left, ws(tag("->"))),
            binary_op(Add.precedence(), Assoc::Left, ws(tag("+"))),
            binary_op(Sub.precedence(), Assoc::Left, ws(tag("-"))),
            binary_op(Fby.precedence(), Assoc::Left, ws(tag("fby"))),
            binary_op(Eq.precedence(), Assoc::Left, ws(tag("=="))),
            binary_op(Neq.precedence(), Assoc::Left, ws(tag("!="))),
            binary_op(Or.precedence(), Assoc::Left, ws(tag("or"))),
            binary_op(And.precedence(), Assoc::Left, ws(tag("and"))),
        )),
        alt((
            delimited(ws(tag("(")), ws(expression), ws(tag(")"))),
            map(array, Expr::Array),
            map(func_call, |(name, args)| Expr::FCall { name, args }),
            map(ws(literal), Expr::Lit),
            map(ws(identifier), Expr::Variable),
        )),
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
                        _ => return Err("Non supported binary operation"),
                    };
                    Ok(Expr::BinOp {
                        lhs: Box::new(lhs),
                        op: bin_op,
                        span_op: Span::new(op),
                        rhs: Box::new(rhs),
                    })
                }
                Prefix(op, rhs) => {
                    use UnaryOp::*;
                    let op = match *op.fragment() {
                        "pre" => Pre,
                        "-" => Inv,
                        "not" => Not,
                        _ => return Err("Non supported unary operator"),
                    };
                    Ok(Expr::UnaryOp {
                        op,
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
