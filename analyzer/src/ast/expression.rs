use crate::{
    ast::{binop::BinOp, literal::Value, unary_op::UnaryOp},
    parser::span::{Ident, Span},
};

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
    Tuple(Vec<Expr>),
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
            Expr::Tuple(exprs) | Expr::Array(exprs) => {
                let mut const_exprs = vec![];
                for expr in exprs.iter() {
                    match expr.get_value() {
                        Some(value_expr) => const_exprs.push(value_expr),
                        None => return None,
                    }
                }
                if let Expr::Tuple(_) = self {
                    Some(Value::Tuple(const_exprs))
                } else {
                    Some(Value::Array(const_exprs))
                }
            }
            Expr::Lit(lit) => Some(lit.clone()),
            _ => None,
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
            Expr::Tuple(v) | Expr::Array(v) => {
                if let Expr::Tuple(_) = self {
                    write!(f, "(")?;
                } else {
                    write!(f, "[")?;
                }
                for (i, expr) in v.iter().enumerate() {
                    write!(f, "{expr}")?;
                    if i != v.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                if let Expr::Tuple(_) = self {
                    write!(f, ")")
                } else {
                    write!(f, "]")
                }
            }
            Expr::If { cond, yes, no } => {
                write!(f, "if (")?;
                cond.fmt_parent(f, parent_op)?;
                write!(f, ") then (")?;
                yes.fmt_parent(f, parent_op)?;
                write!(f, ") else (")?;
                no.fmt_parent(f, parent_op)?;
                write!(f, ")")
            }
        }
    }
}
impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.fmt_parent(f, None)
    }
}
