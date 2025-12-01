use crate::{
    parser::{ast::Ast, node::Node, span::Span, var_type::VarType, visitor::Visitor},
    token_type::TokenType,
};
use lsp_types::SemanticToken;

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
        if let Some((_, t)) = &x.tag {
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
