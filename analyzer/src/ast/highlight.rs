use crate::{
    ast::{to_range::ToRange, visitor::Visitor},
    parser::{ast::Ast, binop::BinOp, node::Node, span::Span, unary_op::UnaryOp},
};
use ls_types::{DocumentHighlight, DocumentHighlightKind, Position};

#[derive(Debug)]
pub(crate) struct DocumentHighlightVisitor {
    searched_span: Option<Span>,
    searched_node: Option<Span>,

    current_node: Option<Span>,

    hightlights: Vec<DocumentHighlight>,
}

impl DocumentHighlightVisitor {
    pub fn new(ast: &Ast, pos: Position) -> Self {
        let mut node_index: i32 = 0;
        for (i, node) in ast.nodes.iter().enumerate() {
            if node.name >= pos {
                node_index = i as i32 - 1;
                break;
            }
        }
        let searched_node = if node_index >= 0 {
            Some(ast.nodes[node_index as usize].name.clone())
        } else {
            None
        };
        Self {
            searched_span: None,
            searched_node,

            current_node: None,

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
    fn visit_node(&mut self, node: &Node) {
        self.current_node = Some(node.name.clone());
        for (name, t) in node.inputs.iter() {
            self.visit_span(name);
            self.visit_var_type(t)
        }

        for (name, t) in node.vars.iter() {
            self.visit_span(name);
            self.visit_var_type(t)
        }

        for (name, t) in node.outputs.iter() {
            self.visit_span(name);
            self.visit_var_type(t)
        }
        for (name, t) in node.let_bindings.iter() {
            self.visit_span(name);
            self.visit_expr(t)
        }
    }
    fn visit_span(&mut self, x: &Span) {
        self.hightlights.push(DocumentHighlight {
            range: x.to_range(),
            kind: Some(DocumentHighlightKind::TEXT),
        })
    }
    fn walk(&mut self, ast: &Ast) {
        if self.searched_span == None || self.searched_node == None {
            return;
        }
        self.visit_ast(ast)
    }
}
