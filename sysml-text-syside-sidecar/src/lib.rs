//! # sysml-text-syside-sidecar
//!
//! SySide parser adapter for SysML v2.
//!
//! This crate provides an adapter that integrates with the SySide SysML parser
//! via Node.js service.
//!
//! ## Integration Points
//!
//! - **HTTP Mode**: Calls a running SySide Node.js HTTP service
//!
//! Set `SYSIDE_ENABLED=1` and `SYSIDE_URL=http://localhost:8082` to enable.

use sysml_core::ModelGraph;
use sysml_span::Diagnostic;
use sysml_text::{ParseResult, Parser, SysmlFile};

/// Transport trait for communicating with the SySide parser.
pub trait Transport {
    /// Send files to the parser and receive the result.
    fn send(&self, files: &[SysmlFile]) -> Result<TransportResult, TransportError>;
}

/// Result from the transport layer.
pub struct TransportResult {
    /// Raw JSON output from the parser.
    pub json: String,
}

/// Error from the transport layer.
#[derive(Debug)]
pub enum TransportError {
    /// Connection failed.
    ConnectionFailed(String),
    /// Parser returned an error.
    ParserError(String),
    /// Timeout.
    Timeout,
}

impl std::fmt::Display for TransportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransportError::ConnectionFailed(s) => write!(f, "connection failed: {}", s),
            TransportError::ParserError(s) => write!(f, "parser error: {}", s),
            TransportError::Timeout => write!(f, "timeout"),
        }
    }
}

impl std::error::Error for TransportError {}

/// Stub HTTP transport (not yet implemented).
#[derive(Debug, Clone)]
pub struct HttpTransport {
    /// The base URL of the SySide service.
    pub url: String,
}

impl HttpTransport {
    /// Create a new HTTP transport.
    pub fn new(url: impl Into<String>) -> Self {
        HttpTransport { url: url.into() }
    }
}

impl Transport for HttpTransport {
    fn send(&self, _files: &[SysmlFile]) -> Result<TransportResult, TransportError> {
        // Stub: not yet implemented
        Err(TransportError::ConnectionFailed(
            "HTTP transport not implemented".to_string(),
        ))
    }
}

/// SySide parser implementation.
pub struct SySideParser<T: Transport> {
    transport: T,
}

impl<T: Transport> SySideParser<T> {
    /// Create a new SySide parser with the given transport.
    pub fn new(transport: T) -> Self {
        SySideParser { transport }
    }
}

impl<T: Transport> Parser for SySideParser<T> {
    fn parse(&self, inputs: &[SysmlFile]) -> ParseResult {
        // Check for environment variable override
        if std::env::var("SYSIDE_ENABLED").is_err() {
            return ParseResult::error(
                "SySide parser not enabled. Set SYSIDE_ENABLED=1 to enable.",
            );
        }

        match self.transport.send(inputs) {
            Ok(_result) => {
                // TODO: Parse the JSON result and convert to ModelGraph
                ParseResult::success(ModelGraph::new())
            }
            Err(e) => ParseResult {
                graph: ModelGraph::new(),
                diagnostics: vec![Diagnostic::error(format!(
                    "SySide transport error: {}",
                    e
                ))],
            },
        }
    }

    fn name(&self) -> &str {
        "syside"
    }

    fn version(&self) -> &str {
        "0.1.0"
    }
}

/// Create a SySide parser with default configuration from environment variables.
pub fn create_from_env() -> SySideParser<HttpTransport> {
    let url = std::env::var("SYSIDE_URL").unwrap_or_else(|_| "http://localhost:8082".to_string());
    SySideParser::new(HttpTransport::new(url))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn http_transport_stub() {
        let transport = HttpTransport::new("http://localhost:8082");
        let result = transport.send(&[]);
        assert!(result.is_err());
    }

    #[test]
    fn syside_parser_disabled_by_default() {
        // Ensure SYSIDE_ENABLED is not set
        std::env::remove_var("SYSIDE_ENABLED");

        let parser = create_from_env();
        let result = parser.parse(&[]);

        assert!(result.has_errors());
        assert!(result.diagnostics[0]
            .message
            .contains("not enabled"));
    }
}
