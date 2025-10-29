use crate::parser::equation::equations;
use crate::parser::expression::Expr;
use crate::parser::ftag::Tag;
use crate::parser::literal::identifier;
use crate::parser::parser::args;
use crate::parser::span::Ident;
use crate::parser::span::LSpan;
use crate::parser::span::Span;
use crate::parser::span_eq::SpanEq;
use crate::parser::var_type::VarType;
use crate::parser::white_space::ws;
use nom::IResult;
use nom::Parser;
use nom::bytes::complete::tag;
use nom::combinator::opt;
use nom::sequence::delimited;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Node {
    pub(crate) span_node: Span,
    pub(crate) span_returns: Span,
    pub(crate) span_let: Span,
    pub(crate) span_tel: Span,
    pub(crate) span_semicolon: Span,

    pub(crate) tag: Option<Tag>,
    pub(crate) name: Span,
    pub(crate) inputs: Vec<(Ident, VarType)>,
    pub(crate) vars: Vec<(Ident, VarType)>,
    pub(crate) outputs: Vec<(Ident, VarType)>,
    pub(crate) let_bindings: Vec<(Ident, Expr)>,
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(t) = &self.tag {
            write!(f, "#[{t}]")?;
        }
        write!(f, "node {}(", self.name)?;
        for (i, (s, t)) in self.inputs.iter().enumerate() {
            write!(f, "{s} : {t}")?;
            if i != self.inputs.len() - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, ") returns (")?;
        for (i, (s, t)) in self.outputs.iter().enumerate() {
            write!(f, "{s} : {t}")?;
            if i != self.outputs.len() - 1 {
                write!(f, ", ")?;
            }
        }

        writeln!(f, ");")?;

        write!(f, "let\n")?;
        for (s, e) in self.let_bindings.iter() {
            writeln!(f, "\t{s} = {e};")?;
        }
        write!(f, "tel")
    }
}

impl SpanEq for Node {
    fn span_eq(&self, other: Self) -> bool {
        if self.tag != other.tag {
            return false;
        }
        if self.name != other.name {
            return false;
        }
        if self.inputs.len() != other.inputs.len() {
            return false;
        }
        if self.outputs.len() != other.outputs.len() {
            return false;
        }
        if self.let_bindings.len() != other.let_bindings.len() {
            return false;
        }
        if self.vars.len() != other.vars.len() {
            return false;
        }
        for i in 0..self.inputs.len() {}

        true
    }
}

pub(crate) fn node(input: LSpan) -> IResult<LSpan, Node> {
    (
        (
            opt(ws(tag("#[test]"))).map(|t| t.map(|_| Tag::Test)),
            ws(tag("node").map(|s| Span::new(s))),
            ws(identifier),
            delimited(ws(tag("(")), ws(args), ws(tag(")"))),
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
                (
                    tag,
                    span_node,
                    name,
                    inputs,
                    span_returns,
                    outputs,
                    span_semicolon,
                ),
                (span_let, let_bindings, span_tel),
            )| {
                Node {
                    tag,
                    name,
                    vars: vec![],
                    inputs,
                    outputs,
                    let_bindings,

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
