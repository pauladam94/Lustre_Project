// Parsers
pub(crate) mod args;
pub(crate) mod array;
pub(crate) mod binop;
pub(crate) mod equation;
pub(crate) mod expression;
pub(crate) mod ftag;
pub(crate) mod func_call;
pub(crate) mod if_then_else;
pub(crate) mod literal;
pub(crate) mod node;
pub(crate) mod tuple;
pub(crate) mod unary_op;
pub(crate) mod var_type;
pub(crate) mod white_space;
pub(crate) mod merge;

pub mod span;
pub mod test;

pub mod ast;
pub mod lustre_parser;
