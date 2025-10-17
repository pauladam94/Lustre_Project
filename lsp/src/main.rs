use async_lock::RwLock;
use lsp_types::{
    Diagnostic, DiagnosticOptions, DiagnosticServerCapabilities,
    DiagnosticSeverity, DocumentDiagnosticParams,
    DocumentDiagnosticReportResult, DocumentFormattingParams,
    DocumentHighlight, DocumentHighlightOptions, DocumentHighlightParams,
    InitializeParams, InitializeResult, InitializedParams, MessageType, OneOf,
    ServerCapabilities, TextDocumentSyncCapability, TextDocumentSyncKind,
    TextEdit, WorkDoneProgressOptions,
};
use lustrels::data::Data;
use tower_lsp_server::jsonrpc::Result;
use tower_lsp_server::{Client, LanguageServer, LspService, Server};

#[derive(Debug)]
struct Backend {
    client: Client,
    data: RwLock<Data>,
}

impl Backend {
    async fn update_text(&self, s: String) {
        self.data.write().await.update_text(s)
    }
}

impl LanguageServer for Backend {
    async fn initialize(
        &self,
        _: InitializeParams,
    ) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                document_formatting_provider: Some(OneOf::Left(true)),
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                document_highlight_provider: Some(OneOf::Right(
                    DocumentHighlightOptions {
                        work_done_progress_options: WorkDoneProgressOptions {
                            work_done_progress: Some(true),
                        },
                    },
                )),
                diagnostic_provider: Some(
                    DiagnosticServerCapabilities::Options(DiagnosticOptions {
                        identifier: None,
                        inter_file_dependencies: false,
                        workspace_diagnostics: false,
                        work_done_progress_options: WorkDoneProgressOptions {
                            work_done_progress: None,
                        },
                    }),
                ),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "Lustre Server Initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
    async fn did_open(&self, params: lsp_types::DidOpenTextDocumentParams) {
        self.update_text(params.text_document.text).await
    }

    async fn did_change(&self, params: lsp_types::DidChangeTextDocumentParams) {
        self.update_text(params.content_changes[0].text.clone())
            .await
    }
    async fn formatting(
        &self,
        params: DocumentFormattingParams,
    ) -> Result<Option<Vec<TextEdit>>> {
        self.data.read().await.formatting()
    }

    async fn document_highlight(
        &self,
        params: DocumentHighlightParams,
    ) -> Result<Option<Vec<DocumentHighlight>>> {
        self.data.read().await.document_hightlight()
    }

    async fn diagnostic(
        &self,
        params: DocumentDiagnosticParams,
    ) -> Result<DocumentDiagnosticReportResult> {
        self.data.read().await.diagnostic()
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend {
        client,
        data: RwLock::new(Data::default()),
    });
    Server::new(stdin, stdout, socket).serve(service).await;
}
