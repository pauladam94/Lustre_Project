use crate::parser::{
    ast::Ast,
    expression::{BinOp, Expr, UnaryOp},
    literal::Literal,
    node::Node,
    parser::Tag,
    span::{LSpan, Span},
    var_type::VarType,
};
use lsp_types::{DocumentHighlight, Position, Range};

pub(crate) trait Visitor {
    fn visit_bin_op(&mut self, x: &BinOp);
    fn visit_unary_op(&mut self, x: &UnaryOp);

    fn visit_span(&mut self, x: &Span);

    fn visit_literal(&mut self, x: &Literal) {
        match x {
            Literal::Ident(span) => {
                self.visit_span(span);
            }
            Literal::Integer(_) => {}
            Literal::Float(_) => {}
        }
    }
    fn visit_tag(&mut self, x: &Tag) {}

    fn visit_var_type(&mut self, x: &VarType) {}

    fn visit_expr(&mut self, x: &Expr) {
        match x {
            Expr::BinOp { lhs, op, rhs } => {
                self.visit_expr(lhs);
                self.visit_bin_op(op);
                self.visit_expr(rhs);
            }
            Expr::Lit(literal) => self.visit_literal(literal),
            Expr::UnaryOp { op, rhs } => {
                self.visit_unary_op(op);
                self.visit_expr(rhs);
            }
            Expr::Array { arr } => arr.iter().for_each(|x| self.visit_expr(x)),
        }
    }

    fn visit_node(&mut self, x: &Node) {
        if let Some(t) = &x.tag {
            self.visit_tag(&t)
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

pub(crate) type DocumentHighlightVisitor = Vec<DocumentHighlight>;

impl Visitor for DocumentHighlightVisitor {
    fn visit_bin_op(&mut self, x: &BinOp) {}
    fn visit_unary_op(&mut self, x: &UnaryOp) {}

    fn visit_span(&mut self, x: &Span) {
        self.push(DocumentHighlight {
            range: Range {
                start: Position {
                    line: x.location_line(),
                    character: 0,
                },
                end: Position {
                    line: x.location_line(),
                    character: 0 + x.fragment().len() as u32,
                },
            },
            kind: None,
        })
    }
}
