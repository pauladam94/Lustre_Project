use crate::{
    interpreter::{compiled_ast::CompiledAst, compiled_expr::CompiledExpr, expr_index::ExprIndex},
    parser::{ast::Ast, node::Node, span::Span},
};

struct Compiler {
    ast: CompiledAst,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            ast: CompiledAst::new(),
        }
    }
}

impl Ast {
    pub fn compile(&self, node_name: Span) -> CompiledAst {
        let mut compiler = Compiler::new();
        compiler.compile_ast(self, node_name);
        compiler.ast
    }
}

impl Compiler {
    pub fn compile_expr(&mut self, ast: &Ast, node: &Node, var: &Span) -> ExprIndex {
        todo!()
    }
    pub fn compile_node(&mut self, ast: &Ast, node: &Node) -> ExprIndex {
        for (index, (out_name, out_type)) in node.outputs.iter().enumerate() {
            let expr_index = self.compile_expr(ast, node, out_name);

            // TODO
            // self.ast.add_expr(
            //     // CompiledExpr::Variable()
            //     expr_index,
            //     format!("{node_name}.{out_name}"),
            // );
        }
        todo!()
    }
    pub fn compile_ast(&mut self, ast: &Ast, node_name: Span) {
        for node in ast.nodes.iter() {
            if node_name == node.name {
                self.compile_node(ast, node);
                // compile
            }
        }
    }
}
