use crate::{
    interpreter::{compiled_ast::CompiledNode, compiled_expr::CompiledExpr, expr_index::ExprIndex},
    parser::{ast::Ast, binop::BinOp, expression::Expr, node::Node, span::Span, unary_op::UnaryOp},
};
use colored::Colorize;
use std::collections::HashMap;

pub struct Compiler {
    pub ast: CompiledNode,
}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            ast: CompiledNode::new(),
        }
    }
}

impl Ast {
    pub fn compile(&self, node_name: Span) -> CompiledNode {
        let mut compiler = Compiler::new();
        compiler.compile_ast(self, node_name);
        eprintln!("{} >>\n{}\n", "RAW COMPILE :".blue(), compiler.ast);
        let result = compiler.schedule();
        eprintln!("{} >>\n{}\n", "COMPILE SCHEDULED :".green(), result);
        result
    }
}

impl Compiler {
    fn compile_pre(
        &mut self,
        ast: &Ast,
        node: &Node,
        inputs: &[ExprIndex],
        outputs: &[ExprIndex],
        vars: &mut HashMap<Span, ExprIndex>,
        expr: &Expr,
    ) -> ExprIndex {
        let expr_index = self.compile_expr(ast, node, inputs, outputs, vars, expr);

        let set = self
            .ast
            .push_back_expr_memo(CompiledExpr::Set { src: expr_index }, format!("set {expr}"));

        let get = self
            .ast
            .push_back_expr_memo(CompiledExpr::Get { src: set }, format!("get {}", expr));

        get
    }
    fn compile_expr(
        &mut self,
        ast: &Ast,
        node: &Node,
        inputs: &[ExprIndex],
        outputs: &[ExprIndex],
        vars: &mut HashMap<Span, ExprIndex>,
        expr: &Expr,
    ) -> ExprIndex {
        let info = format!("{expr}");
        match expr {
            Expr::BinOp {
                lhs,
                op: BinOp::Fby,
                span_op: _,
                rhs,
            } => {
                let ilhs = self.compile_expr(ast, node, inputs, outputs, vars, lhs);
                let irhs = self.compile_pre(ast, node, inputs, outputs, vars, rhs);
                self.ast.push_back_expr_memo(
                    CompiledExpr::BinOp {
                        lhs: ilhs,
                        op: BinOp::Arrow,
                        rhs: irhs,
                    },
                    info,
                )
            }
            Expr::BinOp {
                lhs,
                op,
                span_op: _,
                rhs,
            } => {
                let ilhs = self.compile_expr(ast, node, inputs, outputs, vars, lhs);
                let irhs = self.compile_expr(ast, node, inputs, outputs, vars, rhs);
                self.ast.push_back_expr_memo(
                    CompiledExpr::BinOp {
                        lhs: ilhs,
                        rhs: irhs,
                        op: *op,
                    },
                    info,
                )
            }
            Expr::UnaryOp {
                op: UnaryOp::Pre,
                span_op: _,
                rhs,
            } => self.compile_pre(ast, node, inputs, outputs, vars, rhs),
            Expr::UnaryOp {
                op,
                span_op: _,
                rhs,
            } => {
                let irhs = self.compile_expr(ast, node, inputs, outputs, vars, rhs);
                self.ast
                    .push_back_expr_memo(CompiledExpr::UnaryOp { op: *op, rhs: irhs }, info)
            }
            Expr::Array(exprs) | Expr::Tuple(exprs) => {
                let iexprs: Vec<ExprIndex> = exprs
                    .iter()
                    .map(|e| self.compile_expr(ast, node, inputs, outputs, vars, e))
                    .collect();

                if let Expr::Array(_) = expr {
                    self.ast
                        .push_back_expr_memo(CompiledExpr::Array(iexprs), info)
                } else {
                    self.ast
                        .push_back_expr_memo(CompiledExpr::tuple_from_vec(iexprs), info)
                }
            }
            Expr::FCall { name, args } => {
                let iargs: Vec<ExprIndex> = args
                    .iter()
                    .map(|e| self.compile_expr(ast, node, inputs, outputs, vars, e))
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
                                return self.ast.push_back_expr_memo(
                                    CompiledExpr::tuple_from_vec(outputs_node),
                                    info,
                                );
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
                self.compile_var(ast, node, inputs, outputs, vars, var)
            }
            Expr::Lit(value) => self
                .ast
                .push_back_expr_memo(CompiledExpr::Lit(value.clone()), info),
            Expr::If {
                cond: _,
                yes: _,
                no: _,
            } => todo!(),
        }
    }

    fn compile_var(
        &mut self,
        ast: &Ast,
        node: &Node,
        inputs: &[ExprIndex],
        outputs: &[ExprIndex],
        vars: &mut HashMap<Span, ExprIndex>,
        var: &Span,
    ) -> ExprIndex {
        if let Some(index) = vars.get(var) {
            return *index;
        }
        for (var_name, expr) in node.let_bindings.iter() {
            if var == var_name {
                let index = self.compile_expr(ast, node, inputs, outputs, vars, expr);
                vars.insert(var.clone(), index);
                return index;
            }
        }
        // Thanks to type checking
        unreachable!()
    }

    fn compile_node(&mut self, ast: &Ast, node: &Node) -> (Vec<ExprIndex>, Vec<ExprIndex>) {
        let mut inputs_index = vec![];
        for (input, _) in node.inputs.iter() {
            inputs_index.push(
                self.ast.push_back_expr_memo(
                    CompiledExpr::Input,
                    format!("{} : {}", "IN".green(), input),
                ),
            );
        }
        let mut outputs_index = vec![];
        for (output, _) in node.outputs.iter() {
            outputs_index.push(self.ast.push_back_expr_memo(
                CompiledExpr::Output,
                format!("{} : {}", "OUT".blue(), output),
            ));
        }

        let mut vars = HashMap::new();
        for (i, (var_name, _)) in node.outputs.iter().enumerate() {
            let iexpr = self.compile_var(
                ast,
                node,
                &inputs_index,
                &outputs_index,
                &mut vars,
                var_name,
            );

            self.ast
                .replace_expr(CompiledExpr::Variable(iexpr), outputs_index[i]);
        }
        (inputs_index, outputs_index)
    }
    pub fn compile_ast(&mut self, ast: &Ast, node_name: Span) {
        for node in ast.nodes.iter() {
            if node_name == node.name {
                let (inputs, outputs) = self.compile_node(ast, node);
                self.ast.set_inputs(inputs);
                self.ast.set_outputs(outputs);
                self.ast.set_values_to_none();
                return;
            }
        }
    }
}
