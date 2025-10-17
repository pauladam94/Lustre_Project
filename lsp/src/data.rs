use lsp_types::Diagnostic;
use lsp_types::DiagnosticSeverity;
use lsp_types::DocumentDiagnosticReport;
use lsp_types::DocumentDiagnosticReportResult;
use lsp_types::DocumentHighlight;
use lsp_types::FullDocumentDiagnosticReport;
use lsp_types::Position;
use lsp_types::Range;
use lsp_types::RelatedFullDocumentDiagnosticReport;
use lsp_types::TextEdit;
use lustre_analyzer::parser::ast::Ast;
use lustre_analyzer::parser::parser::lustre_parse;
use tower_lsp_server::jsonrpc::Result;

#[derive(Debug, Clone)]
pub struct Data {
    text: String,
    parse: std::result::Result<Ast, Vec<Diagnostic>>,
    type_check: Option<Vec<Diagnostic>>,
    test_check: Option<Vec<Diagnostic>>,
}
impl Data {
    pub fn update_text(&mut self, s: String) {
        self.text = s;
        self.parse = lustre_parse(&self.text);
        if self.parse.is_err() {
            return;
        }

        // self.type_check = self.ast.type_check();
    }
    pub fn formatting(&self) -> Result<Option<Vec<TextEdit>>> {
        let mut text_edits: Vec<TextEdit> = self
            .text
            .lines()
            .enumerate()
            .map(|(i, line)| TextEdit {
                range: Range {
                    start: Position {
                        line: i as u32,
                        character: 0,
                    },
                    end: Position {
                        line: i as u32,
                        character: line.len() as u32,
                    },
                },
                new_text: "".to_string(),
            })
            .collect();

        match &self.parse {
            Ok(ast) => {
                text_edits.extend(ast.text_edit());
                Ok(Some(text_edits))
            }
            _ => Ok(None),
        }
    }
    pub fn document_hightlight(
        &self,
    ) -> Result<Option<Vec<DocumentHighlight>>> {
        match &self.parse {
            Err(_) => Ok(None),
            Ok(ast) => Ok(Some(ast.document_hightlight())),
        }
    }
    pub fn diagnostic(&self) -> Result<DocumentDiagnosticReportResult> {
        let diags = match &self.parse {
            Ok(ast) => match &self.type_check {
                Some(d) => d.clone(),
                None => vec![],
            },
            Err(d) => d.clone(),
        };

        Ok(DocumentDiagnosticReportResult::Report(
            DocumentDiagnosticReport::Full(
                RelatedFullDocumentDiagnosticReport {
                    related_documents: None,
                    full_document_diagnostic_report:
                        FullDocumentDiagnosticReport {
                            result_id: None,
                            items: diags,
                        },
                },
            ),
        ))
    }
}

impl std::default::Default for Data {
    fn default() -> Self {
        Self {
            text: Default::default(),
            parse: Err(vec![]),
            type_check: None,
            test_check: None,
        }
    }
}
