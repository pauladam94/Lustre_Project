use crate::parser::node::{Node, node};
use crate::parser::span::LSpan;
use crate::parser::visitor::{DocumentHighlightVisitor, SemanticTokenVisitor, Visitor};
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
    pub fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
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
        visitor
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
