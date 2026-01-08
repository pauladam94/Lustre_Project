use crate::ast::ftag::Tag;
use crate::parser::args::args;
use crate::parser::equation::equations;
use crate::parser::literal::identifier;
use crate::parser::parsed_node::ParsedNode;
use crate::parser::span::LSpan;
use crate::parser::span::Span;
use crate::parser::var_type::InnerVarType;
use crate::parser::var_type::VarType;
use crate::parser::white_space::ws;
use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::opt;
use nom::combinator::recognize;
use nom::sequence::delimited;

pub(crate) fn node(input: LSpan) -> IResult<LSpan, ParsedNode> {
    (
        (
            opt(ws(tag("#[test]"))).map(|t| t.map(|s| (Span::new(s), Tag::Test))),
            ws(tag("node").map(|s| Span::new(s))),
            ws(identifier),
            alt((
                recognize((ws(tag("(")), ws(tag(")")))).map(|paren| {
                    vec![(
                        Span::new(paren),
                        VarType {
                            initialized: true,
                            inner: InnerVarType::Unit,
                        },
                    )]
                }),
                delimited(ws(tag("(")), ws(args), ws(tag(")"))),
            )),
            ws(tag("returns").map(|s| Span::new(s))),
            delimited(ws(tag("(")), ws(args), ws(tag(")"))),
            ws(tag(";")).map(|s| Span::new(s)),
        ),
        (
            ws(tag("let").map(|s| Span::new(s))),
            ws(equations),
            ws(tag("tel").map(|s| Span::new(s))),
        ),
    )
        .map(
            |(
                (tag, span_node, name, inputs, span_returns, outputs, span_semicolon),
                (span_let, (let_bindings, span_semicolon_equations), span_tel),
            )| {
                ParsedNode {
                    tag,
                    name,
                    vars: vec![],
                    inputs,
                    outputs,
                    let_bindings,
                    span_semicolon_equations,

                    span_node,
                    span_returns,
                    span_semicolon,
                    span_let,
                    span_tel,
                }
            },
        )
        .parse(input)
}

#[cfg(test)]
mod tests {
    use crate::parser::{
        node::node,
        test::{error_test, ok_test},
    };

    #[test]
    fn empty_node() {
        ok_test(node, "node f() returns (); let tel");
        ok_test(
            node,
            "#[test]node f() returns ();
            let
            tel
        ",
        );
        ok_test(
            node,
            "
            node f    () returns (  )  ;
            let

            tel
            ",
        );

        error_test(
            node,
            "
            noe f    () returns (  )  ;
            let

            tel
            ",
        );
        error_test(
            node,
            "node f returns ();
            let
            tel",
        );
    }
    #[test]
    fn one_equation_node() {
        ok_test(
            node,
            "node f() returns ();
            let
            x = 4;
            tel
            ",
        );
    }
}
