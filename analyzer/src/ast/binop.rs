use crate::{
    ast::{expression::Precedence, literal::Value},
    interpreter::instant::Instant,
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
    Caret,
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
        match (lhs, self, rhs) {
            // Singular Values
            (Int(l), Add, Int(r)) => Some(Int(l + r)),
            (Float(l), Add, Float(r)) => Some(Float(l + r)),

            (Int(l), Sub, Int(r)) => Some(Int(l - r)),
            (Float(l), Sub, Float(r)) => Some(Float(l - r)),

            (Int(l), Mult, Int(r)) => Some(Int(l * r)),
            (Float(l), Mult, Float(r)) => Some(Float(l * r)),

            (Int(l), Div, Int(r)) => Some(Int(l / r)),
            (Float(l), Div, Float(r)) => Some(Float(l / r)),

            (Int(l), Eq, Int(r)) => Some(Bool(l == r)),
            (Bool(l), Eq, Bool(r)) => Some(Bool(l == r)),
            (Float(l), Eq, Float(r)) => Some(Bool(l == r)),

            (Int(l), Neq, Int(r)) => Some(Bool(l != r)),
            (Bool(l), Neq, Bool(r)) => Some(Bool(l != r)),
            (Float(l), Neq, Float(r)) => Some(Bool(l != r)),

            (Bool(l), Or, Bool(r)) => Some(Bool(*l || *r)),
            (Bool(l), And, Bool(r)) => Some(Bool(*l && *r)),

            (lhs, Caret, Int(r)) => match usize::try_from(*r) {
                Ok(i) => Some(Array(vec![lhs.clone(); i])),
                Err(_) => None,
            },

            (_, Fby, _) => None, // todo maybe put unreachable!(),
            (_, Arrow, _) => match instant {
                Some(Instant::Initial) => Some(lhs.clone()),
                Some(Instant::NonInitial) => Some(rhs.clone()),
                None => None,
            },

            // Tuple && Arrays
            (Tuple(l), Eq | Neq, Tuple(r)) | (Array(l), Eq | Neq, Array(r)) => {
                let mut res = true; // op.apply(Value::Bool(true), Value::Bool(true));
                for (lv, rv) in l.iter().zip(r.iter()) {
                    if let Some(Value::Bool(b)) = self.apply(lv, rv, instant) {
                        res = res && b
                    }
                }
                Some(Bool(res))
            }
            (Tuple(l), Add | Sub | Mult | Div | Or | And, Tuple(r))
            | (Array(l), Add | Sub | Mult | Div | Or | And, Array(r)) => {
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
            BinOp::Caret => 2,
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
            BinOp::Caret => write!(f, "^"),
        }
    }
}
