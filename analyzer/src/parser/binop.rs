use crate::parser::expression::Precedence;

#[derive(Clone, Debug, PartialEq, Copy)]
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
