use crate::parser::ast::{Ast, ast};
use crate::parser::expression::{Expr, expression};
use crate::parser::literal::{Literal, identifier};
use crate::parser::span::{Ident, LSpan, Span};
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





fn arg(input: LSpan) -> IResult<LSpan, (Ident, VarType)> {
    separated_pair(ws(identifier), ws(tag(":")), ws(var_type)).parse(input)
}

pub(crate) fn array(input: LSpan) -> IResult<LSpan, Vec<Expr>> {
    delimited(
        ws(tag("[")),
        terminated(
            (many0(terminated(expression, ws(tag(",")))), ws(expression)),
            opt(ws(tag(","))),
        ),
        ws(tag("]")),
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
