use crate::parser::args::args;
use crate::parser::equation::equations;
use crate::parser::expression::Expr;
use crate::parser::ftag::Tag;
use crate::parser::literal::Value;
use crate::parser::literal::identifier;
use crate::parser::span::Ident;
use crate::parser::span::LSpan;
use crate::parser::span::PositionEnd;
use crate::parser::span::Span;
// use crate::parser::span_eq::_SpanEq;
use crate::parser::var_type::InnerVarType;
use crate::parser::var_type::VarType;
use crate::parser::white_space::ws;
use lsp_types::Position;
use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::opt;
use nom::combinator::recognize;
use nom::sequence::delimited;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Node {
    // pub(crate) span_tag: Span,
    pub(crate) span_node: Span,
    pub(crate) span_returns: Span,
    pub(crate) span_let: Span,
    pub(crate) span_tel: Span,
    pub(crate) span_semicolon: Span,

    pub(crate) tag: Option<(Span, Tag)>,
    pub(crate) name: Span,
    pub(crate) inputs: Vec<(Ident, VarType)>,
    pub(crate) vars: Vec<(Ident, VarType)>,
    pub(crate) outputs: Vec<(Ident, VarType)>,
    pub(crate) let_bindings: Vec<(Ident, Expr)>,
    pub(crate) span_semicolon_equations: Vec<Span>,
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some((_, t)) = &self.tag {
            writeln!(f, "#[{t}]")?;
        }
        write!(f, "node {}(", self.name)?;
        if self.inputs.len() != 1 || self.inputs[0].1.inner != InnerVarType::Unit {
            for (i, (s, t)) in self.inputs.iter().enumerate() {
                write!(f, "{s} : {t}")?;
                if i != self.inputs.len() - 1 {
                    write!(f, ", ")?;
                }
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

        writeln!(f, "let")?;
        for (s, e) in self.let_bindings.iter() {
            writeln!(f, "\t{s} = {e};")?;
        }
        write!(f, "tel")
    }
}

impl Node {
    pub fn hint_reduced(&self) -> (Position, String) {
        (
            self.tag.as_ref().unwrap().0.position_end(),
            if self.is_only_true_equations() {
                " ✅".to_string()
            } else {
                " ❌".to_string()
            },
        )
    }
    pub fn is_test(&self) -> bool {
        self.tag.is_some() && self.outputs.len() == 1
    }
    pub fn is_only_true_equations(&self) -> bool {
        self.let_bindings.len() == 1 // it has one equation
        && self.let_bindings[0].0.fragment() == self.outputs[0].0.fragment() // the only equation is the one of the output 
        && self.let_bindings[0].1 == Expr::Lit(Value::Bool(true)) // The only equation is "= true;"
    }
    pub fn push_expr(&mut self, name: Span, expr: Expr) {
        self.let_bindings.push((name, expr));
    }
    pub fn shell_from_node(&self) -> Self {
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
            let_bindings: _,
            span_semicolon_equations,
        } = self;

        Self {
            // todo use better patter for this
            span_node: span_node.clone(),
            span_returns: span_returns.clone(),
            span_let: span_let.clone(),
            span_tel: span_tel.clone(),
            span_semicolon: span_semicolon.clone(),
            tag: tag.clone(),
            name: name.clone(),
            inputs: inputs.clone(),
            vars: vars.clone(),
            outputs: outputs.clone(),
            let_bindings: vec![],
            span_semicolon_equations: span_semicolon_equations.clone(),
        }
    }
}

pub(crate) fn node(input: LSpan) -> IResult<LSpan, Node> {
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
                Node {
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
