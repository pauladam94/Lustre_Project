use crate::{
    ast::{
        ast_types::AstTypes, expression::Expr, highlight::DocumentHighlightVisitor, node::Node,
        semantic_token::SemanticTokenVisitor, visitor::Visitor,
    },
    parser::span::Span,
};
use lsp_types::{DocumentHighlight, Position, Range, SemanticToken, TextEdit};

#[derive(Clone, Debug)]
pub struct Ast {
    pub(crate) nodes: Vec<Node>,
    pub types: AstTypes,
}

impl std::fmt::Display for Ast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, node) in self.nodes.iter().enumerate() {
            write!(f, "{node}")?;
            if i != self.nodes.len() - 1 {
                write!(f, "\n\n")?;
            } else {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl Default for Ast {
    fn default() -> Self {
        Self::new()
    }
}

impl Ast {
    pub fn new() -> Self {
        Self {
            nodes: vec![],
            types: AstTypes::new(),
        }
    }
    pub fn hint_last_node_reduced(&self) -> Option<(Position, String)> {
        self.nodes.last().map(|node| node.hint_reduced())
    }
    pub fn last_nodes_is_test(&self) -> bool {
        match self.nodes.last() {
            Some(node) => node.is_test(),
            None => false,
        }
    }
    pub fn push_expr(&mut self, name: Span, expr: Expr) {
        if let Some(node) = self.nodes.last_mut() {
            node.push_expr(name, expr)
        }
    }
    pub fn text_edit(&self) -> Vec<TextEdit> {
        format!("{}", self)
            .lines()
            .enumerate()
            .map(|(nb_line, line)| TextEdit {
                range: Range {
                    start: Position {
                        line: nb_line as u32,
                        character: 0,
                    },
                    end: Position {
                        line: nb_line as u32,
                        character: 0,
                    },
                },
                new_text: line.to_string(),
            })
            .collect()
    }

    pub fn document_hightlight(&self, pos: Position) -> Vec<DocumentHighlight> {
        let mut visitor = DocumentHighlightVisitor::new(self, pos);
        visitor.walk(self);
        visitor.hightlights()
    }
    pub fn semantic_tokens_full(&self) -> Vec<SemanticToken> {
        let mut visitor = SemanticTokenVisitor::new();
        visitor.walk(self);
        visitor.get_tokens()
    }
}
