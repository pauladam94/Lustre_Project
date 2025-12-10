use crate::parser::expression::Expr;
use crate::parser::hightlight::DocumentHighlightVisitor;
use crate::parser::node::{Node, node};
use crate::parser::semantic_token::SemanticTokenVisitor;
use crate::parser::span::{LSpan, Span};
use crate::parser::visitor::Visitor;
use crate::parser::white_space::ws;
use lsp_types::{DocumentHighlight, Position, Range, SemanticToken, TextEdit};
use nom::IResult;
use nom::Parser;
use nom::combinator::all_consuming;
use nom::multi::many0;

#[derive(Clone, Debug, PartialEq)]
pub struct Ast {
    pub(crate) nodes: Vec<Node>,
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
        Self { nodes: vec![] }
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

    pub fn document_hightlight(&self) -> Vec<DocumentHighlight> {
        let mut visitor = DocumentHighlightVisitor::new();
        visitor.walk(self);
        visitor.hightlights()
    }
    pub fn semantic_tokens_full(&self) -> Vec<SemanticToken> {
        let mut visitor = SemanticTokenVisitor::new();
        visitor.walk(self);
        visitor.get_tokens()
    }
}

pub fn ast(input: LSpan) -> IResult<LSpan, Ast> {
    all_consuming(many0(ws(node)))
        .map(|n| Ast { nodes: n })
        .parse(input)
}
