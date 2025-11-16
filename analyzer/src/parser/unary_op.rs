use crate::parser::expression::Precedence;

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum UnaryOp {
    Inv,
    Pre,
    Not,
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
