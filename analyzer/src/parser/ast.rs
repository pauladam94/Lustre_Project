use crate::parser::node::node;
use crate::parser::parsed_ast::ParsedAst;
use crate::parser::span::LSpan;
use crate::parser::white_space::ws;
use nom::IResult;
use nom::Parser;
use nom::combinator::all_consuming;
use nom::multi::many0;

pub fn ast(input: LSpan) -> IResult<LSpan, ParsedAst> {
    all_consuming(many0(ws(node)))
        .map(|nodes| ParsedAst { nodes })
        .parse(input)
}
