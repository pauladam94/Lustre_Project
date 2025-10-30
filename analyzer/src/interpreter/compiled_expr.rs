use crate::{
    interpreter::expr_index::ExprIndex,
    parser::{
        expression::{BinOp, UnaryOp},
        literal::Value,
    },
};

#[derive(Debug, Clone)]
pub enum CompiledExpr {
    BinOp {
        lhs: ExprIndex,
        rhs: ExprIndex,
        op: BinOp,
    },
    UnaryOp {
        op: UnaryOp,
        rhs: ExprIndex,
    },
    Array(Vec<ExprIndex>),
    Variable(ExprIndex),
    Lit(Value),
}

impl std::fmt::Display for CompiledExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompiledExpr::BinOp { lhs, rhs, op } => {
                write!(f, "({lhs} {rhs} {op})")
            }
            CompiledExpr::UnaryOp { op, rhs } => {
                write!(f, "{op} {rhs}")
            }
            CompiledExpr::Array(items) => {
                for (i, item) in items.iter().enumerate() {
                    write!(f, "{item}")?;
                    if i != items.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                Ok(())
            }
            CompiledExpr::Variable(expr_index) => {
                write!(f, "{expr_index}")
            }
            CompiledExpr::Lit(value) => {
                write!(f, "{value}")
            }
        }
    }
}
