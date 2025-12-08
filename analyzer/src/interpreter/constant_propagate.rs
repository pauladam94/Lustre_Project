use crate::{
    checker::types::{FunctionCallType, FunctionType},
    parser::{
        ast::Ast,
        expression::Expr,
        literal::Value,
        node::Node,
        span::{PositionEnd, Span},
    },
};
use lsp_types::{InlayHint, InlayHintLabel, Position};
use std::collections::HashMap;

#[derive(Default)]
pub struct PropagaterConst {
    ast: Ast,
    seen_equations: HashMap<Span, Option<Value>>,
    hints: Vec<InlayHint>,
}

impl PropagaterConst {
    pub fn new() -> Self {
        Self {
            ast: Ast::new(),
            seen_equations: HashMap::new(),
            hints: vec![],
        }
    }
}
impl PropagaterConst {
    fn push_hint(&mut self, position: Position, label: String) {
        self.hints.push(InlayHint {
            position,
            label: InlayHintLabel::String(label),
            kind: None,
            text_edits: None,
            tooltip: None,
            padding_left: None,
            padding_right: None,
            data: None,
        })
    }

    fn reduced_test_hint(&mut self) {
        if self.ast.last_nodes_is_test()
            && let Some((position, label)) = self.ast.hint_last_node_reduced_test()
        {
            self.push_hint(position, label)
        }
    }
}
impl Ast {
    pub fn propagate_const(&self) -> (Self, Vec<InlayHint>) {
        let mut propagater_const = PropagaterConst::new();
        propagater_const.const_ast(self);
        (propagater_const.ast, propagater_const.hints)
    }
}

impl PropagaterConst {
    fn const_expr(&mut self, ast: &Ast, node: &Node, expr: &Expr) -> Expr {
        match expr {
            Expr::BinOp {
                lhs,
                op,
                span_op,
                rhs,
            } => {
                let lhs = self.const_expr(ast, node, lhs);
                let rhs = self.const_expr(ast, node, rhs);

                use crate::parser::binop::BinOp::*;
                match (lhs.get_value(), rhs.get_value()) {
                    (Some(Value::Integer(lv)), Some(Value::Integer(rv))) => match op {
                        Add => Expr::Lit(Value::Integer(lv + rv)),
                        Sub => Expr::Lit(Value::Integer(lv - rv)),
                        Mult => Expr::Lit(Value::Integer(lv * rv)),
                        Div => Expr::Lit(Value::Integer(lv / rv)),
                        Eq => Expr::Lit(Value::Bool(lv == rv)),
                        Neq => Expr::Lit(Value::Bool(lv != rv)),
                        _ => expr.clone(),
                    },
                    (Some(Value::Float(lv)), Some(Value::Float(rv))) => match op {
                        Add => Expr::Lit(Value::Float(lv + rv)),
                        Sub => Expr::Lit(Value::Float(lv - rv)),
                        Mult => Expr::Lit(Value::Float(lv * rv)),
                        Div => Expr::Lit(Value::Float(lv / rv)),
                        Eq => Expr::Lit(Value::Bool(lv == rv)),
                        Neq => Expr::Lit(Value::Bool(lv != rv)),
                        _ => expr.clone(),
                    },
                    (Some(Value::Bool(lv)), Some(Value::Bool(rv))) => match op {
                        Eq => Expr::Lit(Value::Bool(lv == rv)),
                        Neq => Expr::Lit(Value::Bool(lv != rv)),
                        _ => expr.clone(),
                    },
                    (Some(Value::Array(l)), Some(Value::Array(r)))
                    | (Some(Value::Tuple(l)), Some(Value::Tuple(r))) => match op {
                        Eq => {
                            for (lv, rv) in l.iter().zip(r.iter()) {
                                if lv != rv {
                                    return Expr::Lit(Value::Bool(false));
                                }
                            }
                            Expr::Lit(Value::Bool(true))
                        }
                        Neq => todo!(),
                        Or => todo!(),
                        And => todo!(),
                        _ => expr.clone(),
                    },
                    (Some(l), None) => Expr::BinOp {
                        lhs: Box::new(Expr::Lit(l)),
                        op: *op,
                        span_op: span_op.clone(),
                        rhs: Box::new(rhs),
                    },
                    (None, Some(r)) => Expr::BinOp {
                        lhs: Box::new(lhs),
                        op: *op,
                        span_op: span_op.clone(),
                        rhs: Box::new(Expr::Lit(r)),
                    },
                    (Some(l), Some(r)) => Expr::BinOp {
                        lhs: Box::new(Expr::Lit(l)),
                        op: *op,
                        span_op: span_op.clone(),
                        rhs: Box::new(Expr::Lit(r)),
                    },
                    (_, _) => expr.clone(),
                }
            }
            Expr::UnaryOp { op, span_op, rhs } => Expr::UnaryOp {
                op: *op,
                span_op: span_op.clone(),
                rhs: Box::new(self.const_expr(ast, node, rhs)),
            },
            Expr::Tuple(exprs) => Expr::Tuple(
                exprs
                    .iter()
                    .map(|e| self.const_expr(ast, node, e))
                    .collect(),
            ),
            Expr::Array(exprs) => Expr::Array(
                exprs
                    .iter()
                    .map(|e| self.const_expr(ast, node, e))
                    .collect(),
            ),
            Expr::FCall { name, args } => {
                let mut args_are_const = true;
                let mut const_args = vec![];
                let mut inputs = vec![];

                for e in args.iter() {
                    let const_expr = self.const_expr(ast, node, e);
                    match const_expr.get_value() {
                        Some(v) => inputs.push(v),
                        None => {
                            args_are_const = false;
                        }
                    }
                    const_args.push(const_expr);
                }
                eprint!("Consts args of func call of '{name}' are :");
                const_args.iter().for_each(|x| eprint!("{x}, "));
                eprintln!();
                if !args_are_const
                    || (const_args.len() == 1 && const_args[0] == Expr::Lit(Value::Unit))
                {
                    return Expr::FCall {
                        name: name.clone(),
                        args: const_args,
                    };
                }

                let func_type = ast
                    .nodes
                    .iter()
                    .fold(None, |acc, node| match acc {
                        Some(_) => acc,
                        None => {
                            if &node.name == name {
                                let (ftype, _) = FunctionType::get_function_type(node);
                                Some(ftype)
                            } else {
                                None
                            }
                        }
                    })
                    .unwrap(); // Ok because of type checking

                // unwrap ok because of type checking
                // This crash (sometimes) ! (because argument of some functions are empty)
                // dbg!();
                let call_type = func_type.function_call_type(&inputs).unwrap();

                // Compile & Interpret the function because arguments are constant
                let mut compile_ast = ast.compile(name.clone());

                match call_type {
                    FunctionCallType::Simple => {
                        Expr::Lit(Value::tuple_from_vec(compile_ast.step(inputs)))
                    }
                    FunctionCallType::Array => {
                        // OK unwrap because every arguments is an array because of typechecking
                        let array_inputs = Value::unwrap_array(inputs).unwrap();
                        // We know this is ok because no function has 0 arguments (always at least unit)
                        let number_steps = array_inputs[0].len();

                        let mut array_outputs = vec![];

                        for instant in 0..number_steps {
                            let mut input = vec![];
                            for x in array_inputs.iter() {
                                input.push(x[instant].clone())
                            }

                            eprint!("At instant {instant}, input is : [");
                            input.iter().for_each(|x| eprint!("{x}, "));
                            eprintln!("]");

                            for (i, res) in compile_ast.step(input).into_iter().enumerate() {
                                if instant == 0 {
                                    array_outputs.push(vec![]);
                                }
                                array_outputs[i].push(res)
                            }
                        }
                        Expr::Lit(Value::tuple_from_vec(
                            array_outputs.into_iter().map(Value::Array).collect(),
                        ))
                    }
                }
            }
            Expr::Variable(var) => match self.const_var(ast, node, var) {
                Some(val) => Expr::Lit(val),
                None => expr.clone(),
            },
            Expr::Lit(_) => expr.clone(),
            _ => expr.clone(),
        }
    }

    fn const_var(&mut self, ast: &Ast, node: &Node, var: &Span) -> Option<Value> {
        if let Some(val) = self.seen_equations.get(var) {
            return val.clone();
        }
        for (i, (name, expr)) in node.let_bindings.iter().enumerate() {
            if name == var {
                self.seen_equations.insert(var.clone(), None);
                let const_expr = self.const_expr(ast, node, expr);
                let opt_value = match const_expr.get_value() {
                    Some(val) => {
                        let end_semicolon = &node.span_semicolon_equations[i];
                        let decalage = 5;
                        self.push_hint(
                            end_semicolon.position_end(),
                            format!(
                                "{:>width$}>> {}",
                                "",
                                val,
                                width = decalage * (end_semicolon.get_column() / decalage + 1)
                                    - end_semicolon.get_column(),
                            ),
                        );
                        Some(val)
                    }
                    None => {
                        self.ast.push_expr(name.clone(), const_expr);
                        None
                    }
                };
                self.seen_equations.insert(var.clone(), opt_value.clone());
                return opt_value;
            }
        }
        None
    }
    fn const_node(&mut self, ast: &Ast, node: &Node) {
        let shell_node = Node::shell_from_node(node);
        self.ast.nodes.push(shell_node);

        for (out, _) in node.outputs.iter() {
            for (i, (name, expr)) in node.let_bindings.iter().enumerate() {
                if out == name {
                    self.seen_equations.insert(name.clone(), None);
                    let new_expr = self.const_expr(ast, node, expr);

                    let val = new_expr.get_value();
                    if let Some(v) = &val {
                        self.push_hint(
                            node.span_semicolon_equations[i].position_end(),
                            format!(">> {}", v),
                        );
                    }

                    self.seen_equations.insert(name.clone(), val);
                    self.ast.push_expr(name.clone(), new_expr);

                    return;
                }
            }
        }
        eprintln!("Compiled Node: \n{}\n", self.ast.nodes.last().unwrap());
        self.reduced_test_hint();
    }

    pub fn const_ast(&mut self, ast: &Ast) {
        for node in ast.nodes.iter() {
            self.seen_equations = HashMap::new();
            self.const_node(ast, node);
        }
    }
}
