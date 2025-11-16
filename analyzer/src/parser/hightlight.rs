use crate::{
    diagnostic::ToRange,
    parser::{binop::BinOp, span::Span, unary_op::UnaryOp, visitor::Visitor},
};
use lsp_types::{DocumentHighlight, DocumentHighlightKind};

#[derive(Debug)]
pub(crate) struct DocumentHighlightVisitor {
    searched_span: Option<Span>,
    hightlights: Vec<DocumentHighlight>,
}

impl DocumentHighlightVisitor {
    pub fn new() -> Self {
        Self {
            searched_span: None,
            hightlights: vec![],
        }
    }
    pub fn hightlights(self) -> Vec<DocumentHighlight> {
        self.hightlights
    }
}

impl Visitor for DocumentHighlightVisitor {
    fn visit_bin_op(&mut self, _: &BinOp) {}
    fn visit_unary_op(&mut self, _: &UnaryOp) {}

    fn visit_span(&mut self, x: &Span) {
        self.hightlights.push(DocumentHighlight {
            range: x.to_range(),
            kind: Some(DocumentHighlightKind::TEXT),
        })
    }
}
