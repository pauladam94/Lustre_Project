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
use lustre_analyzer::ast::ast::Ast;
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
    /// 4. Type check Again with more information on Ast
    /// 5. Propagate constant again on Ast
    pub fn update_text(&mut self, s: String) {
        self.text = s;
        // 1.
        self.parse = lustre_parse(&self.text);
        if let Ok(ast) = &mut self.parse {
            // 2.
            let (diags_1, type_hint_1) = ast.check();

            if diags_1.is_empty() {
                eprintln!(">> Second Round");
                // 3.
                let (mut ast, mut test_hint_1) = ast.propagate_const();
                eprintln!(">> 1st Propagate Const:\n{}", ast);
                // 4.
                // maybe not ignoring this
                let (diags_2, mut type_hint_2) = ast.check();
                // 5.
                let (ast, test_hint_2) = ast.propagate_const();
                eprintln!(">> 2nd Propagate Const:\n{}", ast);
                for hint2 in test_hint_2 {
                    if !test_hint_1
                        .iter()
                        .any(|hint1| hint1.position == hint2.position)
                    {
                        test_hint_1.push(hint2)
                    }
                }
                eprintln!("TypeHint1: \n{:#?}", type_hint_1);
                eprintln!("TypeHint1: \n{:#?}", type_hint_2);
                for hint1 in type_hint_1.clone() {
                    if !type_hint_2
                        .iter()
                        .any(|hint2| hint1.position == hint2.position)
                    {
                        type_hint_2.push(hint1)
                    }
                }
                self.type_diag = diags_2;
                self.type_hint = type_hint_2;
                self.test_hint = test_hint_1;
                // self.type_hint = type_hint_1;
                // self.test_hint = test_hint_1;
            } else {
                eprintln!("\t>> Got {} diags", diags_1.len());
                self.test_hint.clear();
                self.type_diag = diags_1;
                self.type_hint = type_hint_1;
            }
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
