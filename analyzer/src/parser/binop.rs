use crate::{
    interpreter::instant::Instant,
    parser::{expression::Precedence, literal::Value},
};

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub enum BinOp {
    Add,
    Sub,
    Mult,
    Div,
    Fby,
    Arrow,
    Eq,
    Neq,
    Or,
    And,
}

impl BinOp {
    // Computing a binary operator on two values
    //
    // If instant is None we are not assuming any instant
    //
    // If instant is 0 or something else then we can deduce the value
    // of temporal operator
    //
    // todo const generics on instant
    pub fn apply(self, lhs: &Value, rhs: &Value, instant: Option<Instant>) -> Option<Value> {
        use BinOp::*;
        use Value::*;
        // TODO finish this for instant not initial
        match (self, lhs, rhs) {
            // Singular Values
            (Add, Int(l), Int(r)) => Some(Int(l + r)),
            (Add, Float(l), Float(r)) => Some(Float(l + r)),

            (Sub, Int(l), Int(r)) => Some(Int(l - r)),
            (Sub, Float(l), Float(r)) => Some(Float(l - r)),

            (Mult, Int(l), Int(r)) => Some(Int(l * r)),
            (Mult, Float(l), Float(r)) => Some(Float(l * r)),

            (Div, Int(l), Int(r)) => Some(Int(l / r)),
            (Div, Float(l), Float(r)) => Some(Float(l / r)),

            (Eq, Int(l), Int(r)) => Some(Bool(l == r)),
            (Eq, Float(l), Float(r)) => Some(Bool(l == r)),

            (Neq, Int(l), Int(r)) => Some(Bool(l != r)),
            (Neq, Float(l), Float(r)) => Some(Bool(l != r)),

            (Or, Bool(l), Bool(r)) => Some(Bool(*l || *r)),
            (And, Bool(l), Bool(r)) => Some(Bool(*l && *r)),

            (Arrow, lv, rv) => match instant {
                Some(Instant::Initial) => Some(lv.clone()),
                Some(Instant::NonInitial) => Some(rv.clone()),
                None => None,
            },

            // Tuple && Arrays
            (Eq | Neq, Tuple(l), Tuple(r)) | (Eq, Array(l), Array(r)) => {
                let mut res = true;
                for (lv, rv) in l.iter().zip(r.iter()) {
                    if let Some(Value::Bool(b)) = self.apply(lv, rv, instant) {
                        res = res && b
                    }
                }
                Some(Bool(res))
            }
            (Add | Sub | Mult | Div | Or | And, Tuple(l), Tuple(r))
            | (Add | Sub | Mult | Div | Or | And, Array(l), Array(r)) => {
                let mut res = vec![];
                for (lv, rv) in l.iter().zip(r.iter()) {
                    res.push(self.apply(lv, rv, instant)?);
                }
                if let Tuple(_) = lhs {
                    Some(Tuple(res))
                } else {
                    Some(Array(res))
                }
            }
            _ => None,
        }
    }
}

impl Precedence for BinOp {
    fn precedence(&self) -> usize {
        match self {
            BinOp::Arrow => 5,
            BinOp::Eq => 4,
            BinOp::Neq => 4,
            BinOp::Or => 4,
            BinOp::And => 4,
            BinOp::Add => 3,
            BinOp::Sub => 3,
            BinOp::Mult => 2,
            BinOp::Fby => 2,
            BinOp::Div => 2,
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
            BinOp::Or => write!(f, "or"),
            BinOp::And => write!(f, "and"),
        }
    }
}
