use crate::{
    interpreter::{compiled_ast::CompiledNode, compiled_expr::CompiledExpr, expr_index::ExprIndex},
    parser::{
        ast::Ast, expression::Expr, literal::Value, node::Node, span::Span, var_type::VarType,
    },
};
use colored::Colorize;

struct Compiler {
    ast: CompiledNode,
    // outputs: Vec<ExprIndex>,
    // inputs: Vec<ExprIndex>,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            ast: CompiledNode::new(),
            // outputs: vec![],
            // inputs: vec![],
        }
    }
}

impl Ast {
    pub fn compile(&self, node_name: Span) -> CompiledNode {
        let mut compiler = Compiler::new();
        compiler.compile_ast(self, node_name);
        compiler.ast
    }
}

impl Compiler {
    pub fn compile_expr(
        &mut self,
        ast: &Ast,
        node: &Node,
        inputs: &[ExprIndex],
        outputs: &[ExprIndex],
        expr: &Expr,
    ) -> ExprIndex {
        let info = format!("{expr}");
        match expr {
            Expr::BinOp {
                lhs,
                op,
                span_op: _,
                rhs,
            } => {
                let ilhs = self.compile_expr(ast, node, inputs, outputs, lhs);
                let irhs = self.compile_expr(ast, node, inputs, outputs, rhs);
                self.ast.add_expr(
                    CompiledExpr::BinOp {
                        lhs: ilhs,
                        rhs: irhs,
                        op: *op,
                    },
                    info,
                )
            }
            Expr::UnaryOp { op, rhs } => {
                let irhs = self.compile_expr(ast, node, inputs, outputs, rhs);
                self.ast
                    .add_expr(CompiledExpr::UnaryOp { op: *op, rhs: irhs }, info)
            }
            Expr::Array(exprs) => {
                let mut iexprs = vec![];
                for e in exprs.iter() {
                    iexprs.push(self.compile_expr(ast, node, inputs, outputs, e));
                }
                self.ast.add_expr(CompiledExpr::Array(iexprs), info)
            }
            Expr::FCall { name, args } => {
                let iargs: Vec<ExprIndex> = args
                    .iter()
                    .map(|e| self.compile_expr(ast, node, inputs, outputs, e))
                    .collect();
                for node in ast.nodes.iter() {
                    if &node.name == name {
                        let (inputs_node, outputs_node) = self.compile_node(ast, node);

                        for (input_node, arg) in inputs_node.iter().zip(iargs.iter()) {
                            self.ast
                                .replace_expr(CompiledExpr::Variable(*arg), *input_node);
                        }
                        match outputs_node[..] {
                            [i] => return i,
                            _ => {
                                return self
                                    .ast
                                    .add_expr(CompiledExpr::tuple_from_vec(outputs_node), info);
                            }
                        }
                    }
                }
                // Thanks to type checking
                unreachable!()
            }
            Expr::Variable(var) => {
                if let Some(i) = node.outputs.iter().position(|(x, _)| x == var) {
                    return outputs[i];
                }
                if let Some(i) = node.inputs.iter().position(|(x, _)| x == var) {
                    return inputs[i];
                }
                self.compile_var(ast, node, inputs, outputs, var)
            }
            Expr::Lit(value) => self.ast.add_expr(CompiledExpr::Lit(value.clone()), info),
        }
    }

    pub fn compile_var(
        &mut self,
        ast: &Ast,
        node: &Node,
        inputs: &[ExprIndex],
        outputs: &[ExprIndex],
        var: &Span,
    ) -> ExprIndex {
        for (var_name, expr) in node.let_bindings.iter() {
            if var == var_name {
                return self.compile_expr(ast, node, inputs, outputs, expr);
            }
        }
        // Thanks to type checking
        unreachable!()
    }

    pub fn compile_node(&mut self, ast: &Ast, node: &Node) -> (Vec<ExprIndex>, Vec<ExprIndex>) {
        let mut inputs_index = vec![];
        for (input, t) in node.inputs.iter() {
            inputs_index.push(
                self.ast
                    .add_expr(CompiledExpr::Input, format!("{} : {}", "IN".green(), input)),
            );
        }
        let mut outputs_index = vec![];
        for (output, t) in node.outputs.iter() {
            outputs_index.push(self.ast.add_expr(
                CompiledExpr::Output,
                format!("{} : {}", "OUT".blue(), output),
            ));
        }

        for (i, (var_name, _)) in node.outputs.iter().enumerate() {
            let expr_index = self.compile_var(ast, node, &inputs_index, &outputs_index, var_name);

            self.ast
                .replace_expr(CompiledExpr::Variable(expr_index), outputs_index[i]);
        }
        (inputs_index, outputs_index)
    }
    pub fn compile_ast(&mut self, ast: &Ast, node_name: Span) {
        for node in ast.nodes.iter() {
            if node_name == node.name {
                let (_, _) = self.compile_node(ast, node);
                return;
            }
        }
    }
}
