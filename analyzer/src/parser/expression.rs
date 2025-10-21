use crate::parser::func_call::func_call;
use crate::parser::literal::Literal;
use crate::parser::literal::identifier;
use crate::parser::literal::literal;
use crate::parser::parser::array;
use crate::parser::span::Ident;
use crate::parser::span::LSpan;
use crate::parser::span::Span;
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

trait Precedence {
    fn precedence(&self) -> usize;
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub(crate) enum BinOp {
    Add,
    Sub,
    Mult,
    Div,
    Fby,
    Arrow,
    Eq,
    Neq,
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
            BinOp::Eq => 1,
            BinOp::Neq => 1,
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
            BinOp::Eq => write!(f, "=="),
            BinOp::Neq => write!(f, "!="),
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
    Array(Vec<Expr>),
    FCall {
        name: Ident,
        args: Vec<Expr>,
    },
    Variable(Span),
    Lit(Literal),
}

impl Expr {
    fn fmt_parent(
        &self,
        f: &mut std::fmt::Formatter,
        parent_op: Option<BinOp>,
    ) -> std::fmt::Result {
        match self {
            Expr::BinOp { lhs, op, rhs } => match parent_op {
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
                    if i == args.len() - 1 {
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
        ws(alt((
            ws(binary_op(Mult.precedence(), Assoc::Left, ws(tag("*")))),
            ws(binary_op(Div.precedence(), Assoc::Left, ws(tag("/")))),
            ws(binary_op(Add.precedence(), Assoc::Left, ws(tag("+")))),
            ws(binary_op(Sub.precedence(), Assoc::Left, ws(tag("-")))),
            ws(binary_op(Arrow.precedence(), Assoc::Left, ws(tag("->")))),
            ws(binary_op(Fby.precedence(), Assoc::Left, ws(tag("fby")))),
            ws(binary_op(Eq.precedence(), Assoc::Left, ws(tag("==")))),
            ws(binary_op(Neq.precedence(), Assoc::Left, ws(tag("!=")))),
        ))),
        alt((
            delimited(ws(tag("(")), ws(expression), ws(tag(")"))),
            map(array, |s| Expr::Array(s)),
            map(func_call, |(name, args)| Expr::FCall { name, args }),
            map(ws(literal), |l| Expr::Lit(l)),
            map(ws(identifier), |s| Expr::Variable(s)),
        )),
        |op: Operation<LSpan, LSpan, LSpan, Expr>| {
            use BinOp::*;
            use nom_language::precedence::Operation::*;
            match op {
                Binary(lhs, op, rhs) => {
                    let op = match *op.fragment() {
                        "*" => Mult,
                        "+" => Add,
                        "/" => Div,
                        "-" => Sub,
                        "->" => Arrow,
                        "fby" => Fby,
                        "==" => Eq,
                        "!=" => Neq,
                        _ => return Err("Non supported binary operation"),
                    };
                    Ok(Expr::BinOp {
                        lhs: Box::new(lhs),
                        op,
                        rhs: Box::new(rhs),
                    })
                }
                _ => Err("Invalid combination"),
            }
        },
    )(input)
}
