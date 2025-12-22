use crate::{
    interpreter::{expr_index::ExprIndex, instant::Instant},
    parser::{binop::BinOp, literal::Value, unary_op::UnaryOp},
};

#[derive(Debug, Clone, PartialEq)]
pub enum CompiledExpr {
    Input,
    Output,
    Pre {
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
    If {
        cond: ExprIndex,
        yes: ExprIndex,
        no: ExprIndex,
    },
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
            CompiledExpr::Variable(expr_index) => write!(f, "{expr_index}"),
            CompiledExpr::Lit(value) => write!(f, "val {value}"),
            CompiledExpr::Input => write!(f, "IN"),
            CompiledExpr::Output => write!(f, "OUT"),
            CompiledExpr::Pre { src } => write!(f, " = {src}"),
            CompiledExpr::If { cond, yes, no } => {
                write!(f, "if {} then {} else {}", cond, yes, no)
            }
        }
    }
}

impl CompiledExpr {
    pub fn get_neighbours(&self) -> Vec<ExprIndex> {
        match self {
            CompiledExpr::Lit(_) | CompiledExpr::Input | CompiledExpr::Output => vec![],
            CompiledExpr::Variable(i)
            | CompiledExpr::UnaryOp { rhs: i, .. }
            | CompiledExpr::Pre { src: i } => {
                vec![*i]
            }
            CompiledExpr::BinOp {
                lhs: i1, rhs: i2, ..
            } => vec![*i1, *i2],
            CompiledExpr::If { cond, yes, no } => vec![*cond, *yes, *no],
            // CompiledExpr::Array(items) | CompiledExpr::Tuple(items) => items.clone(),
        }
    }

    pub fn compute_one_step(&self, values: &[Option<Value>], instant: &Instant) -> Option<Value> {
        match self {
            CompiledExpr::Input => None,
            CompiledExpr::Output => {
                unreachable!()
            }
            CompiledExpr::Pre { src } => values[*src].clone(),
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
            CompiledExpr::Variable(expr_index) => values[*expr_index].clone(),
            CompiledExpr::Lit(value) => Some(value.clone()),
            CompiledExpr::If { cond, yes, no } => {
                let cv = values[*cond].clone()?;
                let yes = values[*yes].clone()?;
                let no = values[*no].clone()?;

                match cv {
                    Value::Bool(true) => Some(yes),
                    Value::Bool(false) => Some(no),
                    _ => None,
                }
            }
        }
    }
}
