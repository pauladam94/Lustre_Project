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
pub enum Expr {
    BinOp {
        lhs: Box<Expr>,
        op: BinOp,
        span_op: Span,
        rhs: Box<Expr>,
    },
    UnaryOp {
        op: UnaryOp,
        span_op: Span,
        rhs: Box<Expr>,
    },
    If {
        cond: Box<Expr>,
        yes: Box<Expr>,
        no: Box<Expr>,
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
    pub fn get_value(&self) -> Option<Value> {
        match self {
            Expr::If { .. } // TODO maybe do better if the condition is true
            | Expr::BinOp { .. }
            | Expr::FCall { .. }
            | Expr::Variable(_) => None,
            Expr::UnaryOp { op, rhs, .. } => {
                let rv = rhs.get_value()?;
                match op {
                    UnaryOp::Inv => match rv {
                        Value::Integer(i) => Some(Value::Integer(-i)),
                        Value::Float(f) => Some(Value::Float(-f)),
                        _ => todo!()
                    },
                    UnaryOp::Pre => None,
                    UnaryOp::Not => match rv {
                        Value::Unit => todo!(),
                        Value::Integer(_) => todo!(),
                        Value::Float(_) => todo!(),
                        Value::Bool(b) => Some(Value::Bool(!b)),
                        Value::Tuple(_) => None, // TODO the not operation
                        Value::Array(values) => Some(Value::Array(
                            values
                                .into_iter()
                                .map(|v| if let Value::Bool(b) = v {
                                    Value::Bool(!b)
                                } else {
                                        unreachable!()
                                })
                                .collect()
                        )),
                    },
                }
            }
            Expr::Array(exprs) => {
                let mut const_exprs = vec![];
                for expr in exprs.iter() {
                    match expr.get_value() {
                        Some(value_expr) => const_exprs.push(value_expr),
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
                    let should_put_parentheses = parent_op.precedence() < op.precedence();
                    // let should_put_parentheses = true;
                    if should_put_parentheses {
                        write!(f, "(")?;
                    }

                    lhs.fmt_parent(f, Some(*op))?;
                    write!(f, " {} ", op)?;
                    rhs.fmt_parent(f, Some(*op))?;

                    if should_put_parentheses {
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
            Expr::UnaryOp {
                op,
                span_op: _,
                rhs,
            } => {
                write!(f, "{op} {rhs}")
            }
            Expr::Lit(lt) => {
                write!(f, "{lt}")
            }
            Expr::FCall { name, args } => {
                write!(f, "{}(", name)?;
                if args.len() != 1 || args[0] != Expr::Lit(Value::Unit) {
                    for (i, arg) in args.iter().enumerate() {
                        write!(f, "{}", arg)?;
                        if i != args.len() - 1 {
                            write!(f, ", ")?;
                        }
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
            Expr::If { cond, yes, no } => {
                write!(f, "if")?;
                cond.fmt_parent(f, parent_op)?;
                write!(f, "then (\n")?;
                yes.fmt_parent(f, parent_op)?;
                write!(f, ") else (\n")?;
                no.fmt_parent(f, parent_op)?;
                write!(f, "\n)")
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
            // TODO Assoc::Right WARNING pretty printing expression
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
                    let unary_op = match *op.fragment() {
                        "pre" => Pre,
                        "-" => Inv,
                        "not" => Not,
                        _ => return Err("Non supported unary operator"),
                    };
                    Ok(Expr::UnaryOp {
                        op: unary_op,
                        span_op: Span::new(op),
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
