use crate::parser::span::Ident;
use std::collections::HashMap;

pub struct CompiledAst {
    graph: HashMap<Ident, CompiledExpr>,
}
pub enum CompiledExpr {
    Add {
        lhs: Box<CompiledExpr>,
        rhs: Box<CompiledExpr>,
    },
}
