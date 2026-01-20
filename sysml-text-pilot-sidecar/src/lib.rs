//! # sysml-text-pilot-sidecar
//!
//! Pilot parser adapter for SysML v2.
//!
//! This crate provides an adapter that integrates with the Pilot SysML parser
//! via either JVM execution or HTTP service.
//!
//! ## Integration Points
//!
//! - **JVM Mode**: Calls a Pilot JAR file via subprocess
//! - **HTTP Mode**: Calls a running Pilot HTTP service
//!
//! Set `PILOT_MODE=http` and `PILOT_URL=http://localhost:8080` to use HTTP mode.
//! Set `PILOT_MODE=jvm` and `PILOT_JAR=/path/to/pilot.jar` to use JVM mode.

use sysml_core::ModelGraph;
use sysml_span::Diagnostic;
use sysml_text::{ParseResult, Parser, SysmlFile};

/// Transport trait for communicating with the Pilot parser.
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
    /// The base URL of the Pilot service.
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
    /// Path to the Pilot JAR file.
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

/// Pilot parser implementation.
pub struct PilotParser<T: Transport> {
    transport: T,
}

impl<T: Transport> PilotParser<T> {
    /// Create a new Pilot parser with the given transport.
    pub fn new(transport: T) -> Self {
        PilotParser { transport }
    }
}

impl<T: Transport> Parser for PilotParser<T> {
    fn parse(&self, inputs: &[SysmlFile]) -> ParseResult {
        // Check for environment variable override
        if std::env::var("PILOT_ENABLED").is_err() {
            return ParseResult::error(
                "Pilot parser not enabled. Set PILOT_ENABLED=1 to enable.",
            );
        }

        match self.transport.send(inputs) {
            Ok(_result) => {
                // TODO: Parse the JSON result and convert to ModelGraph
                ParseResult::success(ModelGraph::new())
            }
            Err(e) => ParseResult {
                graph: ModelGraph::new(),
                diagnostics: vec![Diagnostic::error(format!("Pilot transport error: {}", e))],
            },
        }
    }

    fn name(&self) -> &str {
        "pilot"
    }

    fn version(&self) -> &str {
        "0.1.0"
    }
}

/// Create a Pilot parser with default configuration from environment variables.
pub fn create_from_env() -> PilotParser<HttpTransport> {
    let url = std::env::var("PILOT_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());
    PilotParser::new(HttpTransport::new(url))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn http_transport_stub() {
        let transport = HttpTransport::new("http://localhost:8080");
        let result = transport.send(&[]);
        assert!(result.is_err());
    }

    #[test]
    fn command_transport_stub() {
        let transport = CommandTransport::new("/path/to/pilot.jar");
        let result = transport.send(&[]);
        assert!(result.is_err());
    }

    #[test]
    fn pilot_parser_disabled_by_default() {
        // Ensure PILOT_ENABLED is not set
        std::env::remove_var("PILOT_ENABLED");

        let parser = create_from_env();
        let result = parser.parse(&[]);

        assert!(result.has_errors());
        assert!(result.diagnostics[0]
            .message
            .contains("not enabled"));
    }
}
