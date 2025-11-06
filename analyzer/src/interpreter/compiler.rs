use std::collections::HashMap;

use crate::{
    interpreter::{compiled_ast::CompiledNode, compiled_expr::CompiledExpr, expr_index::ExprIndex},
    parser::{ast::Ast, binop::BinOp, expression::Expr, node::Node, span::Span, unary_op::UnaryOp},
};
use colored::Colorize;

struct Compiler {
    ast: CompiledNode,
    sets: Vec<CompiledExpr>,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            ast: CompiledNode::new(),
            sets: vec![],
        }
    }
}

impl Ast {
    pub fn compile(&self, node_name: Span) -> CompiledNode {
        let mut compiler = Compiler::new();
        compiler.compile_ast(self, node_name);

        // Offset every expression for adding sets
        let Compiler { mut ast, sets } = compiler;
        let offset = sets.len();
        ast.offset_index(offset);

        for mut set in sets.into_iter().rev() {
            set.offset_index(offset);
            ast.insert_expr(0, set);
            ast.insert_info(0, format!("set"));
        }
        ast
    }
}

impl Compiler {
    pub fn compile_pre(
        &mut self,
        ast: &Ast,
        node: &Node,
        inputs: &[ExprIndex],
        outputs: &[ExprIndex],
        vars: &mut HashMap<Span, ExprIndex>,
        expr: &Expr,
    ) -> ExprIndex {
        let get_index = self
            .ast
            .push_back_expr(CompiledExpr::Output, format!("get {}", expr));

        let expr_index = self.compile_expr(ast, node, inputs, outputs, vars, expr);

        let set = CompiledExpr::Set { src: expr_index };
        let set_index = ExprIndex::new(self.sets.len());
        self.sets.push(set);

        let get = CompiledExpr::Get { src: set_index };

        self.ast.replace_expr(get, get_index);

        return get_index;
    }
    pub fn compile_expr(
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
                let res = self.ast.push_back_expr(CompiledExpr::Output, info);
                let temp_var = self.compile_pre(ast, node, inputs, outputs, vars, rhs);
                let ilhs = self.compile_expr(ast, node, inputs, outputs, vars, lhs);
                self.ast.replace_expr(
                    CompiledExpr::BinOp {
                        lhs: ilhs,
                        op: BinOp::Arrow,
                        rhs: temp_var,
                    },
                    res,
                );
                return res;
            }
            Expr::BinOp {
                lhs,
                op,
                span_op: _,
                rhs,
            } => {
                let res = self.ast.push_back_expr(CompiledExpr::Output, info);
                let ilhs = self.compile_expr(ast, node, inputs, outputs, vars, lhs);
                let irhs = self.compile_expr(ast, node, inputs, outputs, vars, rhs);
                self.ast.replace_expr(
                    CompiledExpr::BinOp {
                        lhs: ilhs,
                        rhs: irhs,
                        op: *op,
                    },
                    res,
                );
                return res;
            }
            Expr::UnaryOp {
                op: UnaryOp::Pre,
                rhs,
            } => {
                // let res = self.ast.push_back_expr(CompiledExpr::Output, info);
                self.compile_pre(ast, node, inputs, outputs, vars, rhs)
            }
            Expr::UnaryOp { op, rhs } => {
                let irhs = self.compile_expr(ast, node, inputs, outputs, vars, rhs);
                self.ast
                    .push_back_expr(CompiledExpr::UnaryOp { op: *op, rhs: irhs }, info)
            }
            Expr::Array(exprs) => {
                let mut iexprs = vec![];
                for e in exprs.iter() {
                    iexprs.push(self.compile_expr(ast, node, inputs, outputs, vars, e));
                }
                self.ast.push_back_expr(CompiledExpr::Array(iexprs), info)
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
                                return self.ast.push_back_expr(
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
                .push_back_expr(CompiledExpr::Lit(value.clone()), info),
        }
    }

    pub fn compile_var(
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

    pub fn compile_node(&mut self, ast: &Ast, node: &Node) -> (Vec<ExprIndex>, Vec<ExprIndex>) {
        let mut inputs_index = vec![];
        for (input, _) in node.inputs.iter() {
            inputs_index.push(
                self.ast
                    .push_back_expr(CompiledExpr::Input, format!("{} : {}", "IN".green(), input)),
            );
        }
        let mut outputs_index = vec![];
        for (output, _) in node.outputs.iter() {
            outputs_index.push(self.ast.push_back_expr(
                CompiledExpr::Output,
                format!("{} : {}", "OUT".blue(), output),
            ));
        }

        let mut vars = HashMap::new();
        for (i, (var_name, _)) in node.outputs.iter().enumerate() {
            let expr_index = self.compile_var(
                ast,
                node,
                &inputs_index,
                &outputs_index,
                &mut vars,
                var_name,
            );

            self.ast
                .replace_expr(CompiledExpr::Variable(expr_index), outputs_index[i]);
        }
        (inputs_index, outputs_index)
    }
    pub fn compile_ast(&mut self, ast: &Ast, node_name: Span) {
        for node in ast.nodes.iter() {
            if node_name == node.name {
                let (inputs, outputs) = self.compile_node(ast, node);
                self.ast.set_inputs(inputs);
                self.ast.set_outputs(outputs);
                return;
            }
        }
    }
}
