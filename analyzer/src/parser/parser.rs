use crate::parser::ast::{Ast, ast};
use crate::parser::expression::{Expr, expression};
use crate::parser::literal::{Literal, identifier};
use crate::parser::span::{LSpan, Span};
use crate::parser::var_type::{VarType, var_type};
use crate::parser::white_space::ws;
use lsp_types::{Diagnostic, DiagnosticSeverity, Position, Range};
use nom::bytes::complete::tag;
use nom::combinator::opt;
use nom::multi::many0;
use nom::sequence::delimited;
use nom::{
    IResult, Parser,
    multi::fold,
    sequence::{separated_pair, terminated},
};

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Tag {
    Test,
}

impl std::fmt::Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Tag::Test => write!(f, "test"),
        }
    }
}

pub(crate) type Ident = Span;

pub(crate) fn blit(s: Span) -> Box<Expr> {
    Box::new(Expr::Lit(Literal::Ident(s)))
}
pub(crate) fn lit(s: Span) -> Expr {
    Expr::Lit(Literal::Ident(s))
}

pub(crate) fn equation(input: LSpan) -> IResult<LSpan, (Ident, Expr)> {
    separated_pair(ws(identifier), ws(tag("=")), ws(expression)).parse(input)
}

pub(crate) fn let_binding(input: LSpan) -> IResult<LSpan, Vec<(Ident, Expr)>> {
    delimited(
        ws(tag("let")),
        fold(
            0..,
            terminated(ws(equation), ws(tag(";"))),
            // preallocates a vector of the max size
            || Vec::new(),
            |mut acc: Vec<_>, item| {
                acc.push(item);
                acc
            },
        ),
        ws(tag("tel")),
    )
    .parse(input)
}

fn arg(input: LSpan) -> IResult<LSpan, (Ident, VarType)> {
    separated_pair(ws(identifier), ws(tag(":")), ws(var_type)).parse(input)
}

pub(crate) fn array(input: LSpan) -> IResult<LSpan, Vec<Expr>> {
    (
        delimited(
            ws(tag("[")),
            many0(terminated(expression, ws(tag(",")))),
            ws(tag("]")),
        ),
        expression,
    )
        .map(|(mut x, l)| {
            x.push(l);
            x
        })
        .parse(input)
}

pub(crate) fn args(input: LSpan) -> IResult<LSpan, Vec<(Ident, VarType)>> {
    (many0(terminated(ws(arg), ws(tag(",")))), opt(ws(arg)))
        .map(|(mut l, v)| {
            if let Some(v) = v {
                l.push(v);
            }
            l
        })
        .parse(input)
}

pub fn lustre_parse(input: &str) -> Result<Ast, Vec<Diagnostic>> {
    match ast.parse(LSpan::new(input)) {
        Ok((_, ast)) => Ok(ast),
        Err(err) => Err(match err {
            nom::Err::Incomplete(needed) => match needed {
                nom::Needed::Unknown => {
                    vec![Diagnostic {
                        range: Range {
                            start: Position {
                                line: 0,
                                character: 0,
                            },
                            end: Position {
                                line: 0,
                                character: 0,
                            },
                        },
                        severity: Some(DiagnosticSeverity::ERROR),
                        message: format!("Parsing Error : Unknown"),
                        ..Default::default()
                    }]
                }
                nom::Needed::Size(non_zero) => {
                    vec![Diagnostic {
                        range: Range {
                            start: Position {
                                line: 0,
                                character: 0,
                            },
                            end: Position {
                                line: 0,
                                character: 0,
                            },
                        },
                        severity: Some(DiagnosticSeverity::ERROR),
                        message: format!(
                            "Parsing Error : Needed Size {}",
                            non_zero
                        ),
                        ..Default::default()
                    }]
                }
            },
            nom::Err::Error(nom::error::Error { input, code })
            | nom::Err::Failure(nom::error::Error { input, code }) => {
                vec![Diagnostic {
                    range: Range {
                        start: Position {
                            line: 0,
                            character: 0,
                        },
                        end: Position {
                            line: 0,
                            character: 0,
                        },
                    },
                    severity: Some(DiagnosticSeverity::ERROR),
                    message: format!(
                        "Parsing Error : at {}{} of kind {:?}",
                        input.location_offset(),
                        input.location_line(),
                        code
                    ),
                    ..Default::default()
                }]
            }
        }),
    }
}
