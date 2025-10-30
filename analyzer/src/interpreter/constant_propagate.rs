use crate::parser::{ast::Ast, expression::Expr, literal::Value, node::Node};

pub trait PropagateConst {
    fn propagate_const(&mut self);
}

impl PropagateConst for Ast {
    fn propagate_const(&mut self) {
        for node in self.nodes.iter_mut() {
            node.propagate_const();
        }
    }
}

impl PropagateConst for Node {
    fn propagate_const(&mut self) {
        for (name, expr) in self.let_bindings.iter_mut() {
            expr.propagate_const();
            match expr.get_value() {
                None => {},
                Some(val) => {
                    // self.replace_variable(val);
                }
            }
            // if expr.is_const() {
            //     // replace occurence of name
            // }
        }
    }
}

impl PropagateConst for Expr {
    fn propagate_const(&mut self) {
        match self {
            Expr::BinOp {
                lhs,
                op,
                span_op: _,
                rhs,
            } => {
                lhs.propagate_const();
                rhs.propagate_const();
                use crate::parser::expression::BinOp::*;
                match (lhs.get_value(), rhs.get_value()) {
                    (Some(Value::Integer(lv)), Some(Value::Integer(rv))) => match op {
                        Add => *self = Expr::Lit(Value::Integer(lv + rv)),
                        Sub => *self = Expr::Lit(Value::Integer(lv - rv)),
                        Mult => *self = Expr::Lit(Value::Integer(lv * rv)),
                        Div => *self = Expr::Lit(Value::Integer(lv / rv)),
                        Fby => {}
                        Arrow => {}
                        Eq => *self = Expr::Lit(Value::Bool(lv == rv)),
                        Neq => *self = Expr::Lit(Value::Bool(lv != rv)),
                    },
                    (Some(Value::Float(lv)), Some(Value::Float(rv))) => match op {
                        Add => *self = Expr::Lit(Value::Float(lv + rv)),
                        Sub => *self = Expr::Lit(Value::Float(lv - rv)),
                        Mult => *self = Expr::Lit(Value::Float(lv * rv)),
                        Div => *self = Expr::Lit(Value::Float(lv / rv)),
                        Fby => {}
                        Arrow => {}
                        Eq => *self = Expr::Lit(Value::Bool(lv == rv)),
                        Neq => *self = Expr::Lit(Value::Bool(lv != rv)),
                    },
                    (Some(Value::Bool(lv)), Some(Value::Bool(rv))) => match op {
                        Eq => *self = Expr::Lit(Value::Bool(lv == rv)),
                        Neq => *self = Expr::Lit(Value::Bool(lv != rv)),
                        Fby => {}
                        Arrow => {}
                        Add => {}
                        Sub => {}
                        Mult => {}
                        Div => {}
                    },
                    (_, _) => {}
                }
            }
            Expr::UnaryOp { op, rhs } => {
                rhs.propagate_const();
            }
            Expr::Array(exprs) => todo!(),
            Expr::FCall { name, args } => todo!(),
            Expr::Variable(span) => todo!(),
            Expr::Lit(literal) => todo!(),
        }
    }
}
