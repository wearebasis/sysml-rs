//! # sysml-lsp-server
//!
//! LSP server implementation for SysML v2.
//!
//! This crate provides a Language Server Protocol server that uses:
//! - sysml-ts for fast CST parsing (outline, syntax errors)
//! - sysml-text for full semantic parsing (optional, when parser available)
//! - sysml-lsp for protocol types

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

use sysml_lsp::{LspDiagnostic, Range as LspRange};
use sysml_ts::{FastParser, StubTreeSitterParser, SysmlFile, extract_outline};

/// Document state.
#[derive(Debug, Clone)]
struct Document {
    /// The document URI.
    uri: String,
    /// The document content.
    content: String,
    /// The document version.
    version: i32,
}

/// The SysML language server backend.
pub struct SysmlLanguageServer {
    /// The LSP client.
    client: Client,
    /// Open documents.
    documents: Arc<RwLock<HashMap<String, Document>>>,
    /// The CST parser.
    cst_parser: StubTreeSitterParser,
}

impl SysmlLanguageServer {
    /// Create a new language server.
    pub fn new(client: Client) -> Self {
        SysmlLanguageServer {
            client,
            documents: Arc::new(RwLock::new(HashMap::new())),
            cst_parser: StubTreeSitterParser::new(),
        }
    }

    /// Publish diagnostics for a document.
    async fn publish_diagnostics(&self, uri: &str, content: &str) {
        let file = SysmlFile::new(uri, content);
        let cst = self.cst_parser.parse_cst(&file);

        let diagnostics: Vec<Diagnostic> = cst
            .errors()
            .iter()
            .map(|err| {
                let range = LspRange::from_span(&err.span, content);
                Diagnostic {
                    range: tower_lsp::lsp_types::Range {
                        start: tower_lsp::lsp_types::Position {
                            line: range.start.line,
                            character: range.start.character,
                        },
                        end: tower_lsp::lsp_types::Position {
                            line: range.end.line,
                            character: range.end.character,
                        },
                    },
                    severity: Some(DiagnosticSeverity::ERROR),
                    source: Some("sysml".to_string()),
                    message: "Syntax error".to_string(),
                    ..Default::default()
                }
            })
            .collect();

        self.client
            .publish_diagnostics(
                Url::parse(uri).unwrap_or_else(|_| Url::parse("file:///unknown").unwrap()),
                diagnostics,
                None,
            )
            .await;
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for SysmlLanguageServer {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                document_symbol_provider: Some(OneOf::Left(true)),
                ..Default::default()
            },
            server_info: Some(ServerInfo {
                name: "sysml-lsp-server".to_string(),
                version: Some("0.1.0".to_string()),
            }),
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "SysML language server initialized")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri.to_string();
        let content = params.text_document.text.clone();
        let version = params.text_document.version;

        {
            let mut docs = self.documents.write().await;
            docs.insert(
                uri.clone(),
                Document {
                    uri: uri.clone(),
                    content: content.clone(),
                    version,
                },
            );
        }

        self.publish_diagnostics(&uri, &content).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri.to_string();
        let version = params.text_document.version;

        // Full sync - take the last change
        if let Some(change) = params.content_changes.into_iter().last() {
            let content = change.text;

            {
                let mut docs = self.documents.write().await;
                docs.insert(
                    uri.clone(),
                    Document {
                        uri: uri.clone(),
                        content: content.clone(),
                        version,
                    },
                );
            }

            self.publish_diagnostics(&uri, &content).await;
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let uri = params.text_document.uri.to_string();
        let mut docs = self.documents.write().await;
        docs.remove(&uri);
    }

    async fn document_symbol(
        &self,
        params: DocumentSymbolParams,
    ) -> Result<Option<DocumentSymbolResponse>> {
        let uri = params.text_document.uri.to_string();

        let docs = self.documents.read().await;
        let doc = match docs.get(&uri) {
            Some(d) => d,
            None => return Ok(None),
        };

        let file = SysmlFile::new(&uri, &doc.content);
        let cst = self.cst_parser.parse_cst(&file);
        let outline = extract_outline(&cst, &doc.content);

        let symbols: Vec<DocumentSymbol> = outline
            .into_iter()
            .map(|item| {
                let range = LspRange::from_span(&item.span, &doc.content);
                DocumentSymbol {
                    name: item.name,
                    detail: None,
                    kind: SymbolKind::PACKAGE,
                    tags: None,
                    deprecated: None,
                    range: tower_lsp::lsp_types::Range {
                        start: tower_lsp::lsp_types::Position {
                            line: range.start.line,
                            character: range.start.character,
                        },
                        end: tower_lsp::lsp_types::Position {
                            line: range.end.line,
                            character: range.end.character,
                        },
                    },
                    selection_range: tower_lsp::lsp_types::Range {
                        start: tower_lsp::lsp_types::Position {
                            line: range.start.line,
                            character: range.start.character,
                        },
                        end: tower_lsp::lsp_types::Position {
                            line: range.end.line,
                            character: range.end.character,
                        },
                    },
                    children: None,
                }
            })
            .collect();

        Ok(Some(DocumentSymbolResponse::Nested(symbols)))
    }
}

/// Create an LSP service.
pub fn create_service() -> (LspService<SysmlLanguageServer>, tower_lsp::ClientSocket) {
    LspService::new(|client| SysmlLanguageServer::new(client))
}

/// Run the LSP server on stdin/stdout.
pub async fn run_stdio() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = create_service();
    Server::new(stdin, stdout, socket).serve(service).await;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn service_creation() {
        let (_service, _socket) = create_service();
        // Just verify it compiles and creates without panic
    }
}
