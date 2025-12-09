use lsp_types::Diagnostic;
use lsp_types::DocumentDiagnosticReport;
use lsp_types::DocumentDiagnosticReportResult;
use lsp_types::DocumentHighlight;
use lsp_types::FullDocumentDiagnosticReport;
use lsp_types::InlayHint;
use lsp_types::Position;
use lsp_types::Range;
use lsp_types::RelatedFullDocumentDiagnosticReport;
use lsp_types::SemanticTokens;
use lsp_types::SemanticTokensResult;
use lsp_types::TextEdit;
use lustre_analyzer::parser::ast::Ast;
use lustre_analyzer::parser::lustre_parser::lustre_parse;
use tower_lsp_server::jsonrpc::Result;

#[derive(Debug, Clone)]
pub struct Data {
    text: String,
    parse: std::result::Result<Ast, Vec<Diagnostic>>,
    check: Vec<Diagnostic>,
    type_hint: Vec<InlayHint>,
    test_hint: Vec<InlayHint>,
    test_diag: Vec<Diagnostic>,
}
impl Data {
    /// Core function that update the data concerning
    /// a given text that is Lustre code
    ///
    /// All steps are :
    /// 1. parsing
    /// 2. type checking
    /// 3. propagate constant of ast
    pub fn update_text(&mut self, s: String) {
        self.text = s;
        self.parse = lustre_parse(&self.text);
        if let Ok(ast) = &self.parse {
            let (check, type_hint) = ast.check();
            if check.is_empty() {
                let (_, test_hint) = ast.propagate_const();
                self.test_hint = test_hint;
            }
            self.check = check;
            self.type_hint = type_hint;
        }
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
    pub fn document_hightlight(&self) -> Result<Option<Vec<DocumentHighlight>>> {
        match &self.parse {
            Err(_) => Ok(None),
            Ok(ast) => Ok(Some(ast.document_hightlight())),
        }
    }
    pub fn diagnostic(&self) -> Result<DocumentDiagnosticReportResult> {
        let diags = match &self.parse {
            Ok(_) => {
                if !self.test_diag.is_empty() {
                    self.test_diag.clone()
                } else {
                    self.check.clone()
                }
            }
            Err(d) => d.clone(),
        };

        Ok(DocumentDiagnosticReportResult::Report(
            DocumentDiagnosticReport::Full(RelatedFullDocumentDiagnosticReport {
                related_documents: None,
                full_document_diagnostic_report: FullDocumentDiagnosticReport {
                    result_id: None,
                    items: diags,
                },
            }),
        ))
    }
    pub fn semantic_tokens_full(&self) -> Result<Option<SemanticTokensResult>> {
        match &self.parse {
            Ok(ast) => Ok(Some(SemanticTokensResult::Tokens(SemanticTokens {
                result_id: None,
                data: ast.semantic_tokens_full(),
            }))),
            Err(_) => Ok(Some(SemanticTokensResult::Partial(
                lsp_types::SemanticTokensPartialResult { data: vec![] },
            ))),
        }
    }
    pub fn inlay_hint(&self) -> Result<Option<Vec<InlayHint>>> {
        let mut hints = self.type_hint.clone();
        hints.extend(self.test_hint.iter().cloned());
        Ok(Some(hints))
    }
}

impl std::default::Default for Data {
    fn default() -> Self {
        Self {
            text: Default::default(),
            parse: Err(vec![]),
            check: vec![],
            type_hint: vec![],
            test_hint: vec![],
            test_diag: vec![],
        }
    }
}
