use crate::parser::{
    ast::Ast, binop::BinOp, expression::Expr, ftag::Tag, literal::Value, node::Node, span::Span,
    unary_op::UnaryOp, var_type::VarType,
};

pub(crate) trait Visitor {
    fn visit_bin_op(&mut self, _: &BinOp) {}
    fn visit_unary_op(&mut self, _: &UnaryOp) {}

    fn visit_span(&mut self, _: &Span) {}

    fn visit_literal(&mut self, _: &Value) {}
    fn visit_tag(&mut self, _: &Tag) {}
    fn visit_var_type(&mut self, _: &VarType) {}

    fn visit_expr(&mut self, x: &Expr) {
        match x {
            Expr::BinOp {
                lhs,
                op,
                span_op: _,
                rhs,
            } => {
                self.visit_expr(lhs);
                self.visit_bin_op(op);
                self.visit_expr(rhs);
            }
            Expr::Lit(literal) => self.visit_literal(literal),
            Expr::UnaryOp {
                op,
                span_op: _,
                rhs,
            } => {
                self.visit_unary_op(op);
                self.visit_expr(rhs);
            }
            Expr::Array(arr) | Expr::Tuple(arr) => arr.iter().for_each(|x| self.visit_expr(x)),
            Expr::FCall { name, args } => {
                self.visit_span(name);
                args.iter().for_each(|e| self.visit_expr(e));
            }
            Expr::Variable(s) => {
                self.visit_span(s);
            }
            Expr::If { cond, yes, no } => {
                self.visit_expr(cond);
                self.visit_expr(yes);
                self.visit_expr(no);
            }
        }
    }

    fn visit_node(&mut self, x: &Node) {
        if let Some((_, t)) = &x.tag {
            self.visit_tag(t)
        }
        self.visit_span(&x.name);
        for (name, t) in x.inputs.iter() {
            self.visit_span(name);
            self.visit_var_type(t)
        }

        for (name, t) in x.vars.iter() {
            self.visit_span(name);
            self.visit_var_type(t)
        }

        for (name, t) in x.outputs.iter() {
            self.visit_span(name);
            self.visit_var_type(t)
        }
        for (name, t) in x.let_bindings.iter() {
            self.visit_span(name);
            self.visit_expr(t)
        }
    }
    fn visit_ast(&mut self, x: &Ast) {
        for node in x.nodes.iter() {
            self.visit_node(node);
        }
    }
    fn walk(&mut self, ast: &Ast) {
        self.visit_ast(ast)
    }
}
