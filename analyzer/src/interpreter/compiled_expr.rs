use crate::{
    interpreter::{expr_index::ExprIndex, instant::Instant},
    parser::{binop::BinOp, literal::Value, unary_op::UnaryOp},
};

#[derive(Debug, Clone, PartialEq)]
pub enum CompiledExpr {
    Input,
    Output,
    Set {
        src: ExprIndex,
    },
    Get {
        src: ExprIndex,
    },
    BinOp {
        lhs: ExprIndex,
        op: BinOp,
        rhs: ExprIndex,
    },
    UnaryOp {
        op: UnaryOp,
        rhs: ExprIndex,
    },
    Array(Vec<ExprIndex>), // TODO remove this
    Tuple(Vec<ExprIndex>), // TODO remove this
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
                write!(f, "{expr_index}")
            }
            CompiledExpr::Lit(value) => {
                write!(f, "val {value}")
            }
            CompiledExpr::Input => write!(f, "IN"),
            CompiledExpr::Output => write!(f, "OUT"),
            CompiledExpr::Set { src } => {
                write!(f, " = {src}")
            }
            CompiledExpr::Get { src } => {
                write!(f, " = {src}")
            }
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

    pub fn compute_one_step(&self, values: &[Option<Value>], instant: &Instant) -> Option<Value> {
        match self {
            CompiledExpr::Input => None,
            CompiledExpr::Output => {
                unreachable!()
            }
            CompiledExpr::Set { src } => values[*src].clone(),
            CompiledExpr::Get { src } => values[*src].clone(),
            CompiledExpr::BinOp { lhs, op, rhs } => {
                let lv = values[*lhs].clone()?;
                if op == &BinOp::Arrow && instant.is_init() {
                    return Some(lv.clone());
                }
                let rv = values[*rhs].clone()?;
                op.apply(&lv, &rv, Some(*instant))
            }
            CompiledExpr::UnaryOp { op, rhs } => {
                let rv = &values[*rhs].clone()?;
                op.apply(rv, Some(*instant))
            }
            CompiledExpr::Array(_) => todo!(),
            CompiledExpr::Tuple(_) => todo!(),
            CompiledExpr::Variable(expr_index) => values[*expr_index].clone(),
            CompiledExpr::Lit(value) => Some(value.clone()),
        }
    }
}
