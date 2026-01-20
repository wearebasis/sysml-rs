//! # sysml-text-monticore-sidecar
//!
//! MontiCore parser adapter for SysML v2.
//!
//! This crate provides an adapter that integrates with the MontiCore SysML parser
//! via either JVM execution or HTTP service.
//!
//! ## Integration Points
//!
//! - **JVM Mode**: Calls a MontiCore JAR file via subprocess
//! - **HTTP Mode**: Calls a running MontiCore HTTP service
//!
//! Set `MONTICORE_MODE=http` and `MONTICORE_URL=http://localhost:8081` to use HTTP mode.
//! Set `MONTICORE_MODE=jvm` and `MONTICORE_JAR=/path/to/monticore.jar` to use JVM mode.

use sysml_core::ModelGraph;
use sysml_span::Diagnostic;
use sysml_text::{ParseResult, Parser, SysmlFile};

/// Transport trait for communicating with the MontiCore parser.
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
    /// The base URL of the MontiCore service.
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

/// Stub command transport for JVM execution (not yet implemented).
#[derive(Debug, Clone)]
pub struct CommandTransport {
    /// Path to the MontiCore JAR file.
    pub jar_path: String,
}

impl CommandTransport {
    /// Create a new command transport.
    pub fn new(jar_path: impl Into<String>) -> Self {
        CommandTransport {
            jar_path: jar_path.into(),
        }
    }
}

impl Transport for CommandTransport {
    fn send(&self, _files: &[SysmlFile]) -> Result<TransportResult, TransportError> {
        // Stub: not yet implemented
        Err(TransportError::ParserError(
            "JVM transport not implemented".to_string(),
        ))
    }
}

/// MontiCore parser implementation.
pub struct MontiCoreParser<T: Transport> {
    transport: T,
}

impl<T: Transport> MontiCoreParser<T> {
    /// Create a new MontiCore parser with the given transport.
    pub fn new(transport: T) -> Self {
        MontiCoreParser { transport }
    }
}

impl<T: Transport> Parser for MontiCoreParser<T> {
    fn parse(&self, inputs: &[SysmlFile]) -> ParseResult {
        // Check for environment variable override
        if std::env::var("MONTICORE_ENABLED").is_err() {
            return ParseResult::error(
                "MontiCore parser not enabled. Set MONTICORE_ENABLED=1 to enable.",
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
                    "MontiCore transport error: {}",
                    e
                ))],
            },
        }
    }

    fn name(&self) -> &str {
        "monticore"
    }

    fn version(&self) -> &str {
        "0.1.0"
    }
}

/// Create a MontiCore parser with default configuration from environment variables.
pub fn create_from_env() -> MontiCoreParser<HttpTransport> {
    let url =
        std::env::var("MONTICORE_URL").unwrap_or_else(|_| "http://localhost:8081".to_string());
    MontiCoreParser::new(HttpTransport::new(url))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn http_transport_stub() {
        let transport = HttpTransport::new("http://localhost:8081");
        let result = transport.send(&[]);
        assert!(result.is_err());
    }

    #[test]
    fn command_transport_stub() {
        let transport = CommandTransport::new("/path/to/monticore.jar");
        let result = transport.send(&[]);
        assert!(result.is_err());
    }

    #[test]
    fn monticore_parser_disabled_by_default() {
        // Ensure MONTICORE_ENABLED is not set
        std::env::remove_var("MONTICORE_ENABLED");

        let parser = create_from_env();
        let result = parser.parse(&[]);

        assert!(result.has_errors());
        assert!(result.diagnostics[0]
            .message
            .contains("not enabled"));
    }
}
