use crate::{
    interpreter::expr_index::ExprIndex,
    parser::{
        expression::{BinOp, UnaryOp},
        literal::Value,
    },
};

#[derive(Debug, Clone)]
pub enum CompiledExpr {
    Input,
    Output,
    BinOp {
        lhs: ExprIndex,
        op: BinOp,
        rhs: ExprIndex,
    },
    UnaryOp {
        op: UnaryOp,
        rhs: ExprIndex,
    },
    Array(Vec<ExprIndex>),
    Tuple(Vec<ExprIndex>),
    Variable(ExprIndex),
    Lit(Value),
}

impl std::fmt::Display for CompiledExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompiledExpr::BinOp { lhs, rhs, op } => {
                write!(f, "{lhs} {op} {rhs}")
            }
            CompiledExpr::UnaryOp { op, rhs } => {
                write!(f, "{op} {rhs}")
            }
            CompiledExpr::Tuple(items) => {
                write!(f, "(")?;
                for (i, item) in items.iter().enumerate() {
                    write!(f, "{item}")?;
                    if i != items.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, ")")
            }
            CompiledExpr::Array(items) => {
                write!(f, "[")?;
                for (i, item) in items.iter().enumerate() {
                    write!(f, "{item}")?;
                    if i != items.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")
            }
            CompiledExpr::Variable(expr_index) => {
                write!(f, "VAR {expr_index}")
            }
            CompiledExpr::Lit(value) => {
                write!(f, "{value}")
            }
            CompiledExpr::Input => write!(f, "IN"),
            CompiledExpr::Output => write!(f, "OUT"),
        }
    }
}

impl CompiledExpr {
    pub fn tuple_from_vec(vec: Vec<ExprIndex>) -> Self {
        match &vec[..] {
            [] => Self::Lit(Value::Unit),
            [t] => Self::Variable(*t),
            _ => Self::Tuple(vec),
        }
    }
}
