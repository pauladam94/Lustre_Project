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

use crate::parser::literal::Literal;
use crate::parser::literal::literal;
use crate::parser::span::LSpan;
use crate::parser::white_space::ws;

trait Precedence {
    fn precedence(&self) -> usize;
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum BinOp {
    Add,
    Sub,
    Mult,
    Div,
    Fby,
    Arrow,
}

impl Precedence for BinOp {
    fn precedence(&self) -> usize {
        match self {
            BinOp::Add => 3,
            BinOp::Sub => 3,
            BinOp::Mult => 2,
            BinOp::Div => 2,
            BinOp::Fby => 2,
            BinOp::Arrow => 2,
        }
    }
}

impl std::fmt::Display for BinOp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BinOp::Add => write!(f, "+"),
            BinOp::Sub => write!(f, "-"),
            BinOp::Mult => write!(f, "*"),
            BinOp::Div => write!(f, "/"),
            BinOp::Fby => write!(f, "fby"),
            BinOp::Arrow => write!(f, "->"),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum UnaryOp {
    Inv,
    Pre,
    Not,
}

impl Precedence for UnaryOp {
    fn precedence(&self) -> usize {
        match self {
            UnaryOp::Inv => 1,
            UnaryOp::Pre => 1,
            UnaryOp::Not => 1,
        }
    }
}

impl std::fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnaryOp::Inv => write!(f, "-"),
            UnaryOp::Pre => write!(f, "pre"),
            UnaryOp::Not => write!(f, "not"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Expr {
    BinOp {
        lhs: Box<Expr>,
        op: BinOp,
        rhs: Box<Expr>,
    },
    UnaryOp {
        op: UnaryOp,
        rhs: Box<Expr>,
    },
    Array {
        arr: Vec<Expr>,
    },
    Lit(Literal),
}

impl Expr {
    fn fmt_parent(
        &self,
        f: &mut std::fmt::Formatter,
        parent: Option<BinOp>,
    ) -> std::fmt::Result {
        match self {
            Expr::BinOp { lhs, op, rhs } => {
                write!(f, "({} {} {})", lhs, op, rhs)
            }
            Expr::UnaryOp { op, rhs } => {
                write!(f, "{op} {rhs}")
            }
            Expr::Lit(lt) => {
                write!(f, "{lt}")
            }
            Expr::Array { arr } => {
                write!(f, "[")?;
                for (i, expr) in arr.iter().enumerate() {
                    write!(f, "{expr}")?;
                    if i != arr.len() - 1 {
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
        ws(alt((
            ws(binary_op(Fby.precedence(), Assoc::Left, ws(tag("fby")))),
            ws(binary_op(Mult.precedence(), Assoc::Left, ws(tag("*")))),
            ws(binary_op(Div.precedence(), Assoc::Left, ws(tag("/")))),
            ws(binary_op(Add.precedence(), Assoc::Left, ws(tag("+")))),
            ws(binary_op(Sub.precedence(), Assoc::Left, ws(tag("-")))),
        ))),
        alt((
            map(ws(literal), |l| Expr::Lit(l)),
            delimited(ws(tag("(")), ws(expression), ws(tag(")"))),
        )),
        |op: Operation<LSpan, LSpan, LSpan, Expr>| {
            use nom_language::precedence::Operation::*;
            match op {
                Binary(lhs, op, rhs) => match *op.fragment() {
                    "*" => Ok(Expr::BinOp {
                        lhs: Box::new(lhs),
                        op: BinOp::Mult,
                        rhs: Box::new(rhs),
                    }),
                    "+" => Ok(Expr::BinOp {
                        lhs: Box::new(lhs),
                        op: BinOp::Add,
                        rhs: Box::new(rhs),
                    }),
                    "/" => Ok(Expr::BinOp {
                        lhs: Box::new(lhs),
                        op: BinOp::Div,
                        rhs: Box::new(rhs),
                    }),
                    "-" => Ok(Expr::BinOp {
                        lhs: Box::new(lhs),
                        op: BinOp::Sub,
                        rhs: Box::new(rhs),
                    }),
                    _ => Err("Non supported binary operation"),
                },

                _ => Err("Invalid combination"),
            }
        },
    )(input)
}
