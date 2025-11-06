use crate::{
    diagnostic::ToRange,
    parser::{
        ast::Ast, binop::BinOp, expression::Expr, ftag::Tag, literal::Value, node::Node,
        span::Span, unary_op::UnaryOp, var_type::VarType,
    },
    token_type::TokenType,
};
use lsp_types::{DocumentHighlight, DocumentHighlightKind, SemanticToken};

pub(crate) trait Visitor {
    fn visit_bin_op(&mut self, x: &BinOp) {}
    fn visit_unary_op(&mut self, x: &UnaryOp) {}

    fn visit_span(&mut self, x: &Span) {}

    fn visit_literal(&mut self, x: &Value) {
        match x {
            Value::Unit => {}
            Value::Integer(_) => {}
            Value::Float(_) => {}
            Value::Bool(_) => {}
            Value::Array(values) => {}
        }
    }
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
            Expr::UnaryOp { op, rhs } => {
                self.visit_unary_op(op);
                self.visit_expr(rhs);
            }
            Expr::Array(arr) => arr.iter().for_each(|x| self.visit_expr(x)),
            Expr::FCall { name, args } => {
                self.visit_span(name);
                args.iter().for_each(|e| self.visit_expr(e));
            }
            Expr::Variable(s) => {
                self.visit_span(s);
            }
        }
    }

    fn visit_node(&mut self, x: &Node) {
        if let Some(t) = &x.tag {
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

pub(crate) type DocumentHighlightVisitor = Vec<DocumentHighlight>;

impl Visitor for DocumentHighlightVisitor {
    fn visit_bin_op(&mut self, _: &BinOp) {}
    fn visit_unary_op(&mut self, _: &UnaryOp) {}

    fn visit_span(&mut self, x: &Span) {
        self.push(DocumentHighlight {
            range: x.to_range(),
            kind: Some(DocumentHighlightKind::TEXT),
        })
    }
}

pub(crate) struct SemanticTokenVisitor {
    tokens: Vec<SemanticToken>,
    _current_type: Option<u32>,
}

impl SemanticTokenVisitor {
    fn push(&mut self, token: SemanticToken) {
        self.tokens.push(token);
    }
    fn compile_tokens(&mut self) {
        for i in (0..self.tokens.len()).rev() {
            let current = self.tokens[i];
            if let Some(last) = self.tokens.get(i - 1) {
                self.tokens[i] = SemanticToken {
                    delta_line: current.delta_line - last.delta_line,
                    delta_start: if current.delta_line == last.delta_line {
                        current.delta_start - last.delta_start
                    } else {
                        current.delta_start
                    },
                    length: current.length,
                    token_type: current.token_type,
                    token_modifiers_bitset: current.token_modifiers_bitset,
                }
            }
        }
    }
    pub(crate) fn get_tokens(self) -> Vec<SemanticToken> {
        self.tokens
    }
}

impl SemanticTokenVisitor {
    pub(crate) fn new() -> Self {
        Self {
            tokens: vec![],
            _current_type: None,
        }
    }
}

impl Visitor for SemanticTokenVisitor {
    fn visit_var_type(&mut self, t: &VarType) {}
    fn visit_node(&mut self, x: &Node) {
        self.push(x.span_node.to_semantic_token(TokenType::Keyword));
        if let Some(t) = &x.tag {
            self.visit_tag(t)
        }
        self.visit_span(&x.name);
        for (name, t) in x.inputs.iter() {
            self.visit_span(name);
            self.visit_var_type(t)
        }

        self.push(x.span_returns.to_semantic_token(TokenType::Keyword));
        for (name, t) in x.outputs.iter() {
            self.visit_span(name);
            self.visit_var_type(t)
        }
        for (name, t) in x.vars.iter() {
            self.visit_span(name);
            self.visit_var_type(t)
        }

        self.push(x.span_let.to_semantic_token(TokenType::Keyword));
        for (name, t) in x.let_bindings.iter() {
            self.visit_span(name);
            self.visit_expr(t)
        }
        self.push(x.span_tel.to_semantic_token(TokenType::Keyword));
    }
    fn visit_span(&mut self, x: &Span) {
        self.push(x.to_semantic_token(TokenType::Variable));
    }
    fn walk(&mut self, ast: &Ast) {
        self.visit_ast(ast);
        self.compile_tokens();
    }
}
