use crate::parser::expression::Expr;
use crate::parser::literal::identifier;
use crate::parser::parser::Ident;
use crate::parser::parser::Tag;
use crate::parser::parser::args;
use crate::parser::parser::let_binding;
use crate::parser::span::LSpan;
use crate::parser::span::Span;
use crate::parser::span_eq::SpanEq;
use crate::parser::var_type::VarType;
use crate::parser::white_space::ws;
use nom::IResult;
use nom::Parser;
use nom::bytes::complete::tag;
use nom::sequence::delimited;
use nom::sequence::terminated;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Node {
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
        terminated(
            (
                delimited(
                    ws(tag("node")),
                    (
                        ws(identifier),
                        delimited(ws(tag("(")), ws(args), ws(tag(")"))),
                    ),
                    ws(tag("returns")),
                ),
                delimited(ws(tag("(")), ws(args), ws(tag(")"))),
            ),
            ws(tag(";")),
        ),
        ws(let_binding),
    )
        .map(|(((name, inputs), outputs), let_bindings)| Node {
            tag: None,
            name: name,
            vars: vec![],
            inputs,
            outputs,
            let_bindings,
        })
        .parse(input)
}
