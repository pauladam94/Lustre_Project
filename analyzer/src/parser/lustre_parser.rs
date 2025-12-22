use crate::ast::to_range::ToRange;
use crate::parser::ast::{Ast, ast};
use crate::parser::span::LSpan;
use ls_types::{Diagnostic, DiagnosticSeverity, Position, Range};

pub fn lustre_parse(input: &str) -> Result<Ast, Vec<Diagnostic>> {
    match ast(LSpan::new(input)) {
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
                        message: "Parsing Error : Unknown".to_string(),
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
                        message: format!("Parsing Error : Needed Size {}", non_zero),
                        ..Default::default()
                    }]
                }
            },
            nom::Err::Error(nom::error::Error { input, code })
            | nom::Err::Failure(nom::error::Error { input, code }) => {
                vec![Diagnostic {
                    range: input.to_range(),
                    severity: Some(DiagnosticSeverity::ERROR),
                    message: format!(
                        "Parsing Error : at {}:{} of kind {:?}",
                        input.get_column(),
                        input.location_line(),
                        code
                    ),
                    ..Default::default()
                }]
            }
        }),
    }
}
