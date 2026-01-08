use lsp_types::{Position, Range};

use crate::{
    ast::{
        binop::BinOp,
        literal::Value,
        to_range::{Merge, ToRange},
        unary_op::UnaryOp,
    },
    parser::span::{Ident, Span, ZERORANGE},
};

pub(crate) trait Precedence {
    fn precedence(&self) -> usize;
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    /// `lhs` `op` `rhs`
    BinOp {
        lhs: Box<Expr>,
        op: BinOp,
        span_op: Span,
        rhs: Box<Expr>,
    },
    /// `op` `expr`
    UnaryOp {
        op: UnaryOp,
        span_op: Span,
        rhs: Box<Expr>,
    },
    /// if `cond` then `yes` else `no`
    If {
        cond: Box<Expr>,
        yes: Box<Expr>,
        no: Box<Expr>,
    },
    /// expr[index]
    Index {
        expr: Box<Expr>,
        index: Box<Expr>,
    },
    /// [`e1`, ..., `en`]
    Array(Vec<Expr>),
    Tuple(Vec<Expr>),
    /// `name`(`arg1`, ..., `argn`)
    FCall {
        name: Ident,
        args: Vec<Expr>,
    },
    /// `var`
    Variable(Span),
    /// `val`
    Lit(Value),
}

impl Expr {
    pub fn index(&self, index: i64) -> Option<Self> {
        match self {
            Expr::Tuple(exprs) | Expr::Array(exprs) => {
                let len = exprs.len();
                if index < (len as i64) && index >= 0 {
                    dbg!("here");
                    Some(exprs[index as usize].clone())
                } else if index < 0 && index + (len as i64) > 0 {
                    Some(exprs[(len as i64 + index) as usize].clone())
                } else {
                    None
                }
            }
            Expr::Lit(Value::Tuple(vals)) | Expr::Lit(Value::Array(vals)) => {
                // todo refactor this duplicate code
                let len = vals.len();
                if index < (len as i64) && index >= 0 {
                    dbg!("here");
                    Some(Expr::Lit(vals[index as usize].clone()))
                } else if index < 0 && index + (len as i64) > 0 {
                    Some(Expr::Lit(vals[(len as i64 + index) as usize].clone()))
                } else {
                    None
                }
            }
            _ => None,
        }
    }
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
            // todo better
            // fix this issue in parsing
            Expr::UnaryOp {
                op: UnaryOp::Inv,
                span_op: _,
                rhs,
            } => {
                if let Expr::Lit(Value::Int(i)) = **rhs {
                    Some(Value::Int(-i))
                } else {
                    None
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
            Expr::Index { expr, index } => {
                expr.fmt_parent(f, parent_op)?;
                write!(f, "[")?;
                index.fmt_parent(f, None)?;
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

impl ToRange for Expr {
    fn to_range(&self) -> Range {
        match self {
            Expr::BinOp {
                lhs,
                op: _,
                span_op: _,
                rhs,
            } => {
                let lrange = lhs.to_range();
                let rrange = rhs.to_range();
                lrange.merge(rrange)
            }
            Expr::UnaryOp {
                op: _,
                span_op,
                rhs,
            } => {
                let rrange = rhs.to_range();
                let op_range = span_op.to_range();
                rrange.merge(op_range)
            }
            Expr::If { cond, yes, no } => {
                let cond_range = cond.to_range();
                let yes_range = yes.to_range();
                let no_range = no.to_range();
                cond_range.merge(yes_range.merge(no_range))
            }
            Expr::Index { expr, index } => expr.to_range().merge(index.to_range()),
            Expr::Tuple(exprs) | Expr::Array(exprs) => {
                if exprs.is_empty() {
                    // todo better by having the span () and [] while parsing
                    return ZERORANGE;
                }
                exprs
                    .iter()
                    .fold(None, |acc: Option<Range>, e| match acc {
                        Some(r) => Some(r.merge(e.to_range())),
                        None => Some(e.to_range()),
                    })
                    .unwrap()
            }
            Expr::FCall { name, args } => args
                .iter()
                .fold(name.to_range(), |acc, arg| acc.merge(arg.to_range())),
            Expr::Variable(span) => span.to_range(),
            Expr::Lit(value) => ZERORANGE, // todo better
        }
    }
}
