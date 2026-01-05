use std::io::{Read, Write, stdin};

use lsp_server::{Message, Response};
use lsp_types::{
    DiagnosticOptions, DiagnosticServerCapabilities, DidChangeTextDocumentParams,
    DidOpenTextDocumentParams, DocumentFormattingParams, DocumentHighlightParams, InitializeParams,
    InitializeResult, InlayHintParams, OneOf, SemanticTokenModifier, SemanticTokensFullOptions,
    SemanticTokensLegend, SemanticTokensOptions, SemanticTokensServerCapabilities,
    ServerCapabilities, TextDocumentSyncCapability, TextDocumentSyncKind, WorkDoneProgressOptions,
};
use lustre_analyzer::ast::token_type::TokenType;
use lustrels::data::ServerState;
use serde_json::{from_value, to_value};

fn read_message() -> Message {
    let mut string_buf = String::new();
    let content_length = "Content-Length: ";
    let _ = stdin().read_line(&mut string_buf);
    let length = string_buf[content_length.len()..]
        .trim()
        .parse::<u64>()
        .unwrap();
    let mut buf = vec![0; length as usize];

    let _ = stdin().read_line(&mut string_buf);
    let mut count: u64 = 0;
    while count < length {
        if let Ok(n) = stdin().read(&mut buf) {
            if n == 0 {
                eprintln!("Nothing has been read (read() == 0)");
                continue;
            }
            count += n as u64;
        }
    }
    return serde_json::from_slice(&buf[..(length as usize)]).unwrap();
}

fn send_message(msg: Message) {
    let len = serde_json::to_vec(&msg).unwrap().len();
    print!("Content-Length: {}\r\n\r\n", len);
    let _ = std::io::stdout().flush();
    serde_json::to_writer(std::io::stdout(), &to_value(&msg).unwrap()).unwrap();
    let _ = std::io::stdout().flush();
}
fn main() {
    let mut data = ServerState::default();
    let mut message: Message;

    loop {
        message = read_message();
        match message {
            Message::Request(request) => {
                eprintln!(">> GOT Request {}", request.method);
                let method = request.method;
                let params = request.params;

                if method == "initialize" {
                    let _params: InitializeParams = from_value(params).unwrap();
                    send_message(Message::Response(Response {
                        id: request.id,
                        result: Some(to_value(initialize_result()).unwrap()),
                        error: None,
                    }));
                } else if method == "textDocument/inlayHint" {
                    let _params: InlayHintParams = from_value(params).unwrap();

                    send_message(Message::Response(Response {
                        id: request.id,
                        result: Some(to_value(data.inlay_hint()).unwrap()),
                        error: None,
                    }));
                } else if method == "textDocument/diagnostic" {
                    send_message(Message::Response(Response {
                        id: request.id,
                        result: Some(to_value(data.diagnostic()).unwrap()),
                        error: None,
                    }));
                } else if method == "textDocument/semanticTokens/full" {
                    send_message(Message::Response(Response {
                        id: request.id,
                        result: Some(to_value(data.semantic_tokens_full()).unwrap()),
                        error: None,
                    }));
                } else if method == "textDocument/documentHightlight" {
                    let params: DocumentHighlightParams = from_value(params).unwrap();
                    send_message(Message::Response(Response {
                        id: request.id,
                        result: Some(
                            to_value(data.document_hightlight(
                                params.text_document_position_params.position,
                            ))
                            .unwrap(),
                        ),
                        error: None,
                    }));
                } else if method == "textDocument/formatting" {
                    let _params: DocumentFormattingParams = from_value(params).unwrap();
                    send_message(Message::Response(Response {
                        id: request.id,
                        result: Some(to_value(data.formatting()).unwrap()),
                        error: None,
                    }));
                } else {
                    eprintln!(">> ERROR : method {method} not supported");
                }
            }
            Message::Response(_response) => {
                eprintln!("Response from client not expected");
            }
            Message::Notification(notification) => {
                eprintln!(">> GOT Notification {}", notification.method);
                let method = notification.method;
                let params = notification.params;
                if method == "textDocument/didOpen" {
                    let params: DidOpenTextDocumentParams = from_value(params).unwrap();
                    data.update_text(params.text_document.text);
                } else if method == "textDocument/didChange" {
                    let params: DidChangeTextDocumentParams = from_value(params).unwrap();
                    data.update_text(params.content_changes[0].text.clone());
                }
            }
        }
    }
}

fn initialize_result() -> InitializeResult {
    InitializeResult {
        capabilities: ServerCapabilities {
            document_formatting_provider: Some(OneOf::Left(true)),
            text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
            // document_highlight_provider: Some(OneOf::Left(true)),
            diagnostic_provider: Some(DiagnosticServerCapabilities::Options(DiagnosticOptions {
                identifier: None,
                inter_file_dependencies: false,
                workspace_diagnostics: false,
                work_done_progress_options: WorkDoneProgressOptions {
                    work_done_progress: None,
                },
            })),
            semantic_tokens_provider: Some(
                SemanticTokensServerCapabilities::SemanticTokensOptions(SemanticTokensOptions {
                    work_done_progress_options: WorkDoneProgressOptions {
                        work_done_progress: None,
                    },
                    legend: SemanticTokensLegend {
                        token_types: TokenType::to_vec(),
                        token_modifiers: vec![SemanticTokenModifier::DECLARATION],
                    },
                    range: Some(false),
                    full: Some(SemanticTokensFullOptions::Bool(true)),
                }),
            ),
            inlay_hint_provider: Some(OneOf::Left(true)),
            ..Default::default()
        },
        ..Default::default()
    }
}
