// Parsers
pub(crate) mod args;
pub(crate) mod array;
pub(crate) mod equation;
pub(crate) mod expression;
pub(crate) mod func_call;
pub(crate) mod if_then_else;
pub(crate) mod literal;
pub(crate) mod merge;
pub(crate) mod node;
pub(crate) mod tuple;
pub(crate) mod var_type;
pub(crate) mod white_space;

pub mod span;
pub mod test;

pub mod ast;
pub mod lustre_parser;

pub mod flatten;
pub mod parsed_ast;
pub mod parsed_node;

// This is a test
pub mod tokenize;
