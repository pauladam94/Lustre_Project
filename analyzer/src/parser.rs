// Parsers
pub(crate) mod args;
pub(crate) mod array;
pub(crate) mod binop;
pub(crate) mod equation;
pub(crate) mod expression;
pub(crate) mod ftag;
pub(crate) mod func_call;
pub(crate) mod literal;
pub(crate) mod node;
pub(crate) mod tuple;
pub(crate) mod unary_op;
pub(crate) mod var_type;
pub(crate) mod white_space;

pub mod span;
pub mod test;

// LSP
pub mod hightlight;
pub mod semantic_token;

// Trait Stuff
pub(crate) mod double_visitor;
pub(crate) mod span_eq;
pub(crate) mod span_parse;
pub mod visitor;

pub mod ast;
pub mod lustre_parser;
