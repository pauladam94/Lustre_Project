use crate::interpreter::{compiled_ast::CompiledNode, compiler::Compiler};

impl Compiler {
    pub fn schedule(&self) -> CompiledNode {
        self.ast.schedule()
    }
}
