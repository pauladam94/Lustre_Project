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

#[derive(Debug, Clone)]
pub struct ServerState {
    text: String,
    parse: std::result::Result<Ast, Vec<Diagnostic>>,
    type_diag: Vec<Diagnostic>,
    type_hint: Vec<InlayHint>,
    test_hint: Vec<InlayHint>,
    test_diag: Vec<Diagnostic>,
}
impl ServerState {
    pub fn text(&self) -> String {
        self.text.clone()
    }
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
            let (diags, type_hint) = ast.check();
            if diags.is_empty() {
                let (_, test_hint) = ast.propagate_const();
                self.test_hint = test_hint;
            } else {
                self.test_hint.clear();
            }
            self.type_diag = diags;
            self.type_hint = type_hint;
        } else {
            self.type_diag.clear();
            self.test_hint.clear();
            self.type_hint.clear();
        }
    }
    pub fn formatting(&self) -> Option<Vec<TextEdit>> {
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
                Some(text_edits)
            }
            _ => None,
        }
    }
    pub fn document_hightlight(&self, pos: Position) -> Option<Vec<DocumentHighlight>> {
        match &self.parse {
            Err(_) => None,
            Ok(ast) => Some(ast.document_hightlight(pos)),
        }
    }
    pub fn diagnostic(&self) -> DocumentDiagnosticReportResult {
        let diags = match &self.parse {
            Ok(_) => {
                if !self.test_diag.is_empty() {
                    self.test_diag.clone()
                } else {
                    self.type_diag.clone()
                }
            }
            Err(d) => d.clone(),
        };

        DocumentDiagnosticReportResult::Report(DocumentDiagnosticReport::Full(
            RelatedFullDocumentDiagnosticReport {
                related_documents: None,
                full_document_diagnostic_report: FullDocumentDiagnosticReport {
                    result_id: None,
                    items: diags,
                },
            },
        ))
    }
    pub fn semantic_tokens_full(&self) -> Option<SemanticTokensResult> {
        match &self.parse {
            Ok(ast) => Some(SemanticTokensResult::Tokens(SemanticTokens {
                result_id: None,
                data: ast.semantic_tokens_full(),
            })),
            Err(_) => Some(SemanticTokensResult::Partial(
                lsp_types::SemanticTokensPartialResult { data: vec![] },
            )),
        }
    }
    pub fn inlay_hint(&self) -> Option<Vec<InlayHint>> {
        let mut hints = self.type_hint.clone();
        hints.extend(self.test_hint.iter().cloned());
        Some(hints)
    }
}

impl std::default::Default for ServerState {
    fn default() -> Self {
        Self {
            text: Default::default(),
            parse: Err(vec![]),
            type_diag: vec![],
            type_hint: vec![],
            test_hint: vec![],
            test_diag: vec![],
        }
    }
}
