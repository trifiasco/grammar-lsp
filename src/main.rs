mod grammar_client;
mod ollama;

use dashmap::DashMap;
use grammar_client::GrammarCheckProvider;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

#[derive(Debug)]
struct Backend {
    client: Client,
    documents: DashMap<String, String>,
    grammar_provider: GrammarCheckProvider,
}

impl Backend {
    fn new(client: Client) -> Self {
        Self {
            client,
            documents: DashMap::new(),
            grammar_provider: GrammarCheckProvider::new(),
        }
    }

    async fn check_grammar(&self, _uri: &Url, text: &str) -> Vec<Diagnostic> {
        let issues = self.grammar_provider.check_grammar(text).await;

        issues
            .into_iter()
            .map(|issue| {
                let line = if issue.line > 0 { issue.line - 1 } else { 0 };

                Diagnostic {
                    range: Range {
                        start: Position {
                            line,
                            character: issue.column,
                        },
                        end: Position {
                            line,
                            character: issue.column + 1,
                        },
                    },
                    severity: Some(DiagnosticSeverity::WARNING),
                    source: Some("grammar-checker".to_string()),
                    message: issue.message,
                    ..Default::default()
                }
            })
            .collect()
    }

}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Options(
                    TextDocumentSyncOptions {
                        open_close: Some(true),
                        change: Some(TextDocumentSyncKind::FULL),
                        will_save: None,
                        will_save_wait_until: None,
                        save: Some(TextDocumentSyncSaveOptions::SaveOptions(SaveOptions {
                            include_text: Some(false),
                        })),
                    },
                )),
                ..Default::default()
            },
            server_info: Some(ServerInfo {
                name: "grammar-lsp".to_string(),
                version: Some("0.1.0".to_string()),
            }),
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "Grammar LSP initialized")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri.to_string();
        self.documents.insert(uri, params.text_document.text);
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        if let Some(change) = params.content_changes.first() {
            let uri = params.text_document.uri.to_string();
            self.documents.insert(uri, change.text.clone());
        }
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        let uri = params.text_document.uri.to_string();

        if let Some(text) = self.documents.get(&uri) {
            let diagnostics = self.check_grammar(&params.text_document.uri, &text).await;

            self.client
                .publish_diagnostics(params.text_document.uri, diagnostics, None)
                .await;
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let uri = params.text_document.uri.to_string();
        self.documents.remove(&uri);

        self.client
            .publish_diagnostics(params.text_document.uri, vec![], None)
            .await;
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend::new(client));

    Server::new(stdin, stdout, socket).serve(service).await;
}
