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
    pub fn schedule(&self) -> CompiledNode {
        self.ast.schedule()
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
        vars: &mut HashMap<Span, Vec<ExprIndex>>,
        expr: &Expr,
    ) -> Vec<ExprIndex> {
        let iexpr = self.compile_expr(ast, node, inputs, outputs, vars, expr);

        iexpr
            .into_iter()
            .map(|ie| {
                self.ast
                    .push_expr(CompiledExpr::Pre { src: ie }, format!("set {expr}"))
            })
            .collect()
    }
    fn compile_expr(
        &mut self,
        ast: &Ast,
        node: &Node,
        inputs: &[ExprIndex],
        outputs: &[ExprIndex],
        vars: &mut HashMap<Span, Vec<ExprIndex>>,
        expr: &Expr,
    ) -> Vec<ExprIndex> {
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
                ilhs.into_iter()
                    .zip(irhs.into_iter())
                    .map(|(il, ir)| {
                        self.ast.push_expr(
                            CompiledExpr::BinOp {
                                lhs: il,
                                op: BinOp::Arrow,
                                rhs: ir,
                            },
                            info.clone(),
                        )
                    })
                    .collect()
            }
            Expr::BinOp {
                lhs,
                op,
                span_op: _,
                rhs,
            } => {
                let ilhs = self.compile_expr(ast, node, inputs, outputs, vars, lhs);
                let irhs = self.compile_expr(ast, node, inputs, outputs, vars, rhs);
                ilhs.into_iter()
                    .zip(irhs.into_iter())
                    .map(|(il, ir)| {
                        self.ast.push_expr(
                            CompiledExpr::BinOp {
                                lhs: il,
                                rhs: ir,
                                op: *op,
                            },
                            info.clone(),
                        )
                    })
                    .collect()
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
                irhs.into_iter()
                    .map(|ir| {
                        self.ast
                            .push_expr(CompiledExpr::UnaryOp { op: *op, rhs: ir }, info.clone())
                    })
                    .collect()
            }
            Expr::Array(exprs) | Expr::Tuple(exprs) => {
                // Flatten operation : todo check
                let mut res = vec![];
                for expr in exprs.iter() {
                    for index in self.compile_expr(ast, node, inputs, outputs, vars, expr) {
                        res.push(index)
                    }
                }
                res
            }
            Expr::FCall { name, args } => {
                let mut iargs = vec![];
                // Flatten operation : todo check
                for e in args.iter() {
                    for index in self.compile_expr(ast, node, inputs, outputs, vars, e) {
                        iargs.push(index)
                    }
                }
                for node in ast.nodes.iter() {
                    if &node.name == name {
                        let (inputs_node, outputs_node) = self.compile_node(ast, node);

                        for (input_node, arg) in inputs_node.iter().zip(iargs.iter()) {
                            self.ast
                                .replace_expr(CompiledExpr::Variable(*arg), *input_node);
                        }
                        return outputs_node;
                    }
                }
                // Thanks to type checking
                unreachable!()
            }
            Expr::Variable(var) => {
                if let Some(i) = node.outputs.iter().position(|(x, _)| x == var) {
                    return vec![outputs[i]];
                }
                if let Some(i) = node.inputs.iter().position(|(x, _)| x == var) {
                    return vec![inputs[i]];
                }
                self.compile_var(ast, node, inputs, outputs, vars, var)
            }
            Expr::Lit(value) => vec![self.ast.push_expr(CompiledExpr::Lit(value.clone()), info)],
            Expr::If { cond, yes, no } => {
                let cond = self.compile_expr(ast, node, inputs, outputs, vars, cond);
                let yes = self.compile_expr(ast, node, inputs, outputs, vars, yes);
                let no = self.compile_expr(ast, node, inputs, outputs, vars, no);

                cond.into_iter()
                    .zip(yes.into_iter())
                    .zip(no.into_iter())
                    .map(|((c, y), n)| {
                        self.ast.push_expr(
                            CompiledExpr::If {
                                cond: c,
                                yes: y,
                                no: n,
                            },
                            info.clone(),
                        )
                    })
                    .collect()
            }
        }
    }

    fn compile_var(
        &mut self,
        ast: &Ast,
        node: &Node,
        inputs: &[ExprIndex],
        outputs: &[ExprIndex],
        vars: &mut HashMap<Span, Vec<ExprIndex>>,
        var: &Span,
    ) -> Vec<ExprIndex> {
        if let Some(index) = vars.get(var) {
            return index.clone();
        }
        for (var_name, expr) in node.let_bindings.iter() {
            if var == var_name {
                let index = self.compile_expr(ast, node, inputs, outputs, vars, expr);
                vars.insert(var.clone(), index.clone());
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
                self.ast
                    .push_expr(CompiledExpr::Input, format!("{} : {}", "IN".green(), input)),
            );
        }
        let mut outputs_index = vec![];
        for (output, _) in node.outputs.iter() {
            outputs_index.push(self.ast.push_expr(
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

            // TODO better this has no chance to work
            self.ast
                .replace_expr(CompiledExpr::Variable(iexpr[0]), outputs_index[i]);
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
