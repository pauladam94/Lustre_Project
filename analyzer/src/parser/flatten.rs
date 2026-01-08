use crate::{ast::{ast::Ast, ast_types::AstTypes, expression::Expr, literal::Value, node::Node}, parser::{parsed_ast::ParsedAst, parsed_node::ParsedNode}};

impl ParsedAst {
    pub fn flatten(self) -> Ast {
        Ast {
            nodes: self.nodes.into_iter().map(|node| node.flatten()).collect(),
            types: AstTypes::default()
        }
    }
}

impl ParsedNode {
    pub fn flatten(self) -> Node {
        let Self {
            span_node,
            span_returns,
            span_let,
            span_tel,
            span_semicolon,
            tag,
            name,
            inputs,
            vars,
            outputs,
            let_bindings,
            span_semicolon_equations,
        } = self;
        let mut new_let_bindings = vec![];
        for (names, expr) in let_bindings.into_iter() {
            if names.len() == 1 {
                new_let_bindings.push((names[0].clone(), expr));
            } else {
                for (index, name) in names.into_iter().enumerate() {
                    new_let_bindings.push((
                        name,
                        Expr::Index {
                            index: Box::new(Expr::Lit(Value::Int(index as i64))),
                            expr: Box::new(expr.clone()),
                        },
                    ))
                }
            }
        }
        Node {
            span_node,
            span_returns,
            span_let,
            span_tel,
            span_semicolon,
            tag,
            name,
            inputs,
            vars,
            outputs,
            let_bindings: new_let_bindings,
            span_semicolon_equations,
        }
    }
}
