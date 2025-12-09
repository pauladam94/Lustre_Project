use crate::{
    interpreter::instant::Instant,
    parser::{expression::Precedence, literal::Value},
};

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub enum UnaryOp {
    Inv,
    Pre,
    Not,
}

impl UnaryOp {
    pub fn apply(&self, rhs: &Value, instant: Option<Instant>) -> Option<Value> {
        use UnaryOp::*;
        use Value::*;
        match (self, rhs) {
            (Inv, Int(i)) => Some(Int(-i)),
            (Inv, Float(i)) => Some(Float(-i)),

            (Pre, Int(i)) => match instant {
                Some(Instant::NonInitial) => Some(Int(*i)),
                _ => None,
            },
            (Pre, Float(f)) => match instant {
                Some(Instant::NonInitial) => Some(Float(*f)),
                _ => None,
            },

            (Not, Bool(b)) => Some(Bool(!b)),
            (_, Tuple(l)) | (_, Array(l)) => {
                let mut res = vec![];
                for v in l.iter() {
                    res.push(self.apply(v, instant)?);
                }
                Some(if let Tuple(_) = rhs {
                    Tuple(res)
                } else {
                    Array(res)
                })
            }
            _ => None,
        }
    }
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
