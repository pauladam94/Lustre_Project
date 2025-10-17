use lsp_types::{DocumentHighlight, Position, Range, TextEdit};
use nom::IResult;
use nom::combinator::all_consuming;
use nom::multi::many0;
use nom::sequence::terminated;
use nom::{Parser, bytes::complete::tag};

use crate::parser::node::{Node, node};
use crate::parser::span::LSpan;
use crate::parser::visitor::{DocumentHighlightVisitor, Visitor};
use crate::parser::white_space::ws;

#[derive(Clone, Debug, PartialEq)]
pub struct Ast {
    pub(crate) nodes: Vec<Node>,
}

impl std::fmt::Display for Ast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, node) in self.nodes.iter().enumerate() {
            write!(f, "{node};")?;
            if i != self.nodes.len() - 1 {
                write!(f, "\n\n")?;
            }
        }
        Ok(())
    }
}

impl Ast {
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
}

pub fn ast(input: LSpan) -> IResult<LSpan, Ast> {
    all_consuming(many0(terminated(ws(node), ws(tag(";")))))
        .map(|n| Ast { nodes: n })
        .parse(input)
}
