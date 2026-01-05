use crate::ast::ast::Ast;
use crate::parser::node::node;
use crate::parser::span::LSpan;
use crate::parser::white_space::ws;
use nom::IResult;
use nom::Parser;
use nom::combinator::all_consuming;
use nom::multi::many0;

pub fn ast(input: LSpan) -> IResult<LSpan, Ast> {
    all_consuming(many0(ws(node)))
        .map(|n| Ast { nodes: n })
        .parse(input)
}
