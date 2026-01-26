//! # sysml-lsp-server
//!
//! LSP server implementation for SysML v2.
//!
//! This crate provides a Language Server Protocol server that uses:
//! - sysml-text-pest for full parsing + resolution diagnostics
//! - sysml-text for library loading and parser traits
//! - sysml-ts for fast CST parsing (outline)
//! - sysml-lsp for protocol types

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

use sysml_lsp::{DiagnosticSeverity as SysmlSeverity, LspDiagnostic, Range as LspRange};
use sysml_text::library::{load_standard_library, LibraryConfig};
use sysml_text::{Parser as SysmlParser, SysmlFile as TextFile};
use sysml_text_pest::PestParser;
use sysml_ts::{extract_outline, FastParser, StubTreeSitterParser, SysmlFile as TsFile};

use sysml_core::ModelGraph;
use sysml_span::Diagnostic as SysmlDiagnostic;

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

#[derive(Debug, Clone)]
enum LibraryState {
    Unloaded,
    Loaded(ModelGraph),
    Failed(String),
}

/// The SysML language server backend.
pub struct SysmlLanguageServer {
    /// The LSP client.
    client: Client,
    /// Open documents.
    documents: Arc<RwLock<HashMap<String, Document>>>,
    /// The CST parser.
    cst_parser: StubTreeSitterParser,
    /// The semantic parser (sysml-text-pest).
    semantic_parser: PestParser,
    /// Standard library cache.
    library_state: Arc<RwLock<LibraryState>>,
}

impl SysmlLanguageServer {
    /// Create a new language server.
    pub fn new(client: Client) -> Self {
        SysmlLanguageServer {
            client,
            documents: Arc::new(RwLock::new(HashMap::new())),
            cst_parser: StubTreeSitterParser::new(),
            semantic_parser: PestParser::new(),
            library_state: Arc::new(RwLock::new(LibraryState::Unloaded)),
        }
    }

    /// Publish diagnostics for a document.
    async fn publish_diagnostics(&self, uri: &str, content: &str) {
        let file = TextFile::new(uri, content);
        let mut result = self.semantic_parser.parse(&[file]);

        let parse_ok = result.error_count() == 0;
        let mut sysml_diags = result.diagnostics.clone();

        if parse_ok {
            let library = self.load_library_if_needed().await;
            let resolution = if let Some(lib) = library {
                result.resolve_with_library(lib)
            } else {
                result.resolve()
            };
            sysml_diags.extend(resolution.diagnostics.into_iter());

            const ENABLE_VALIDATION: bool = true;
            if ENABLE_VALIDATION {
                let base_len = result.diagnostics.len();
                result.validate_structure();
                result.validate_relationships();
                sysml_diags.extend(result.diagnostics.iter().skip(base_len).cloned());
            }
        }

        let diagnostics: Vec<Diagnostic> = sysml_diags
            .iter()
            .map(|diag| to_lsp_diagnostic(diag, content))
            .collect();

        self.client
            .publish_diagnostics(
                Url::parse(uri).unwrap_or_else(|_| Url::parse("file:///unknown").unwrap()),
                diagnostics,
                None,
            )
            .await;
    }

    async fn load_library_if_needed(&self) -> Option<ModelGraph> {
        {
            let state = self.library_state.read().await;
            match &*state {
                LibraryState::Loaded(lib) => return Some(lib.clone()),
                LibraryState::Failed(_) => return None,
                LibraryState::Unloaded => {}
            }
        }

        let config = match LibraryConfig::from_env_optional() {
            Some(config) => config,
            None => {
                let mut state = self.library_state.write().await;
                *state = LibraryState::Failed(
                    "Standard library not configured (SYSML_LIBRARY_PATH not set and default not found)".to_string(),
                );
                return None;
            }
        };

        match load_standard_library(&self.semantic_parser, &config) {
            Ok(library) => {
                let mut state = self.library_state.write().await;
                *state = LibraryState::Loaded(library.clone());
                self.client
                    .log_message(
                        MessageType::INFO,
                        "Loaded SysML standard library for resolution",
                    )
                    .await;
                Some(library)
            }
            Err(err) => {
                let message = format!("Failed to load standard library: {}", err);
                let mut state = self.library_state.write().await;
                *state = LibraryState::Failed(message.clone());
                self.client.log_message(MessageType::ERROR, message).await;
                None
            }
        }
    }
}

fn to_lsp_diagnostic(diag: &SysmlDiagnostic, source: &str) -> Diagnostic {
    let lsp_diag = LspDiagnostic::from_sysml(diag, source);
    let range = to_lsp_range(lsp_diag.range);
    let severity = lsp_diag.severity.map(|s| match s {
        SysmlSeverity::Error => DiagnosticSeverity::ERROR,
        SysmlSeverity::Warning => DiagnosticSeverity::WARNING,
        SysmlSeverity::Information => DiagnosticSeverity::INFORMATION,
        SysmlSeverity::Hint => DiagnosticSeverity::HINT,
    });
    let code = lsp_diag.code.map(NumberOrString::String);

    let related_information: Vec<DiagnosticRelatedInformation> = lsp_diag
        .related_information
        .into_iter()
        .filter_map(|info| {
            let uri = parse_uri(&info.location.uri)?;
            Some(DiagnosticRelatedInformation {
                location: Location {
                    uri,
                    range: to_lsp_range(info.location.range),
                },
                message: info.message,
            })
        })
        .collect();

    Diagnostic {
        range,
        severity,
        code,
        source: lsp_diag.source,
        message: lsp_diag.message,
        related_information: if related_information.is_empty() {
            None
        } else {
            Some(related_information)
        },
        ..Default::default()
    }
}

fn to_lsp_range(range: LspRange) -> Range {
    Range {
        start: Position {
            line: range.start.line,
            character: range.start.character,
        },
        end: Position {
            line: range.end.line,
            character: range.end.character,
        },
    }
}

fn parse_uri(uri: &str) -> Option<Url> {
    Url::parse(uri)
        .ok()
        .or_else(|| Url::from_file_path(uri).ok())
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

        let file = TsFile::new(&uri, &doc.content);
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
