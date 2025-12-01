use crate::parser::{
    ast::Ast, binop::BinOp, expression::Expr, ftag::Tag, literal::Value, node::Node, span::Span,
    var_type::VarType,
};
use colored::Colorize;

pub(crate) trait DoubleTogetherVisitor {
    fn visit_bin_op(&mut self, a: &BinOp, b: &BinOp);
    fn visit_span(&mut self, a: &Span, b: &Span);
    fn visit_literal(&mut self, a: &Value, b: &Value);
    fn visit_tag(&mut self, a: &Tag, b: &Tag);
    fn visit_var_type(&mut self, a: &VarType, b: &VarType);
    fn visit_expr(&mut self, a: &Expr, b: &Expr);
    fn visit_node(&mut self, a: &Node, b: &Node);
    fn visit_ast(&mut self, x1: &Ast, x2: &Ast);
    fn walk(&mut self, ast1: &Ast, ast2: &Ast) {
        self.visit_ast(ast1, ast2);
    }
}

pub(crate) struct ShallowEq {
    pub(crate) is_equal: bool,
}

impl std::default::Default for ShallowEq {
    fn default() -> Self {
        Self { is_equal: true }
    }
}

macro_rules! im_here {
    ($f : expr, $x1 : expr, $x2 : expr) => {
        if !$f.is_eq() {
            println!(">> Here {} \"{}\" != \"{}\"", "ERROR".red(), $x1, $x2);
        }
    };
}

impl ShallowEq {
    fn and(&mut self, b: bool) {
        self.is_equal = self.is_equal && b;
        if !self.is_equal {}
    }
    fn set(&mut self, b: bool) {
        self.is_equal = b;
    }
    pub(crate) fn is_eq(&self) -> bool {
        self.is_equal
    }
}
impl DoubleTogetherVisitor for ShallowEq {
    fn visit_tag(&mut self, x1: &Tag, x2: &Tag) {
        self.and(x1 == x2);
        im_here!(self, x1, x2);
    }

    fn visit_var_type(&mut self, x1: &VarType, x2: &VarType) {
        self.and(x1 == x2);
        im_here!(self, x1, x2);
    }
    fn visit_bin_op(&mut self, x1: &BinOp, x2: &BinOp) {
        self.and(x1 == x2);
        im_here!(self, x1, x2);
    }

    fn visit_span(&mut self, x1: &Span, x2: &Span) {
        self.and(x1.fragment() == x2.fragment());
        im_here!(self, x1, x2);
    }

    fn visit_literal(&mut self, x1: &Value, x2: &Value) {
        match (x1, x2) {
            (Value::Integer(i1), Value::Integer(i2)) => {
                self.and(i1 == i2);
            }
            (Value::Float(f1), Value::Float(f2)) => {
                self.and(f1 == f2);
            }
            (Value::Bool(b1), Value::Bool(b2)) => {
                self.and(b1 == b2);
            }
            (_, _) => {
                self.set(false);
            }
        }
        im_here!(self, x1, x2);
    }

    fn visit_expr(&mut self, a: &Expr, b: &Expr) {
        match (a, b) {
            (
                Expr::BinOp {
                    lhs: alhs,
                    op: aop,
                    span_op: _,
                    rhs: arhs,
                },
                Expr::BinOp {
                    lhs: blhs,
                    op: bop,
                    span_op: _,
                    rhs: brhs,
                },
            ) => {
                self.visit_expr(alhs, blhs);
                self.visit_bin_op(aop, bop);
                self.visit_expr(arhs, brhs);
            }
            (Expr::Lit(aliteral), Expr::Lit(bliteral)) => self.visit_literal(aliteral, bliteral),
            (_, _) => {}
        }
        im_here!(self, a, b);
    }
    fn visit_node(&mut self, x1: &Node, x2: &Node) {
        match (&x1.tag, &x2.tag) {
            (None, None) => {}
            (None, Some(_)) | (Some(_), None) => {
                self.set(false);
                return;
            }
            (Some((_, t1)), Some((_, t2))) => self.visit_tag(t1, t2),
        }

        self.visit_span(&x1.name, &x2.name);

        if x1.inputs.len() != x2.inputs.len() {
            self.set(false);
            return;
        }
        for i in 0..x1.inputs.len() {
            self.visit_span(&x1.inputs[i].0, &x2.inputs[i].0);
            self.visit_var_type(&x1.inputs[i].1, &x2.inputs[i].1);
        }

        if x1.outputs.len() != x2.outputs.len() {
            self.set(false);
            return;
        }
        for i in 0..x1.outputs.len() {
            self.visit_span(&x1.outputs[i].0, &x2.outputs[i].0);
            self.visit_var_type(&x1.outputs[i].1, &x2.outputs[i].1);
        }

        if x1.vars.len() != x2.vars.len() {
            self.set(false);
            return;
        }
        for i in 0..x1.vars.len() {
            self.visit_span(&x1.vars[i].0, &x2.vars[i].0);
            self.visit_var_type(&x1.vars[i].1, &x2.vars[i].1);
        }

        if x1.let_bindings.len() != x2.let_bindings.len() {
            self.set(false);
            return;
        }
        for i in 0..x1.let_bindings.len() {
            self.visit_span(&x1.let_bindings[i].0, &x2.let_bindings[i].0);
            self.visit_expr(&x1.let_bindings[i].1, &x2.let_bindings[i].1);
        }
        im_here!(self, x1, x2);
    }

    fn visit_ast(&mut self, x1: &Ast, x2: &Ast) {
        if x1.nodes.len() != x2.nodes.len() {
            self.set(false);
            return;
        }
        for i in 0..x1.nodes.len() {
            self.visit_node(&x1.nodes[i], &x2.nodes[i]);
        }
        im_here!(self, x1, x2);
    }
}
