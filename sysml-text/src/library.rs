//! Standard library loading for name resolution.
//!
//! This module provides functionality to parse and load the SysML v2 standard
//! library files (KerML kernel libraries and SysML systems libraries) into a
//! ModelGraph for use in name resolution.
//!
//! # Usage
//!
//! ```ignore
//! use sysml_text::library::{load_standard_library, LibraryConfig};
//! use sysml_text_pest::PestParser;
//!
//! let config = LibraryConfig::from_env().unwrap();
//! let parser = PestParser::new();
//! let library = load_standard_library(&parser, &config)?;
//!
//! // Use with resolution
//! let result = parser.parse(&files).into_resolved_with_library(library);
//! ```

use std::path::{Path, PathBuf};

use sysml_core::{ElementKind, ModelGraph};
use thiserror::Error;
use walkdir::WalkDir;

use crate::{Parser, SysmlFile};

/// Errors that can occur during library loading.
#[derive(Debug, Error)]
pub enum LibraryLoadError {
    /// The library path does not exist.
    #[error("Library path does not exist: {0}")]
    PathNotFound(PathBuf),

    /// Failed to read a library file.
    #[error("Failed to read library file {path}: {source}")]
    ReadError {
        path: PathBuf,
        source: std::io::Error,
    },

    /// Parse errors in library files.
    #[error("Parse errors in library files: {0} errors")]
    ParseErrors(usize),

    /// Environment variable not set.
    #[error("Environment variable {0} not set")]
    EnvVarNotSet(String),
}

/// Configuration for library loading.
#[derive(Debug, Clone)]
pub struct LibraryConfig {
    /// Base path to the library directory.
    /// Should contain `library.kernel/` and `library.systems/` subdirectories.
    pub library_path: PathBuf,

    /// Whether to load KerML kernel libraries.
    pub load_kerml: bool,

    /// Whether to load SysML systems libraries.
    pub load_sysml: bool,

    /// Whether to load domain libraries (ISQ, Geometry, etc.).
    pub load_domain: bool,

    /// Whether to fail on parse errors in library files.
    /// If false, files with errors are skipped.
    pub strict: bool,
}

impl LibraryConfig {
    /// Create a new library config with the given path.
    pub fn new(library_path: impl Into<PathBuf>) -> Self {
        Self {
            library_path: library_path.into(),
            load_kerml: true,
            load_sysml: true,
            load_domain: true,
            strict: false,
        }
    }

    /// Create config from SYSML_LIBRARY_PATH environment variable.
    pub fn from_env() -> Result<Self, LibraryLoadError> {
        let path = std::env::var("SYSML_LIBRARY_PATH")
            .map_err(|_| LibraryLoadError::EnvVarNotSet("SYSML_LIBRARY_PATH".to_string()))?;

        Ok(Self::new(path))
    }

    /// Try to create config from environment, returning None if not set.
    pub fn from_env_optional() -> Option<Self> {
        Self::from_env().ok()
    }

    /// Create config from corpus path (for testing).
    /// The corpus path should be the sysmlv2-references directory.
    pub fn from_corpus_path(corpus_path: impl Into<PathBuf>) -> Self {
        let corpus = corpus_path.into();
        let library_path = corpus.join("SysML-v2-Pilot-Implementation/org.omg.sysml.xpect.tests");
        Self::new(library_path)
    }
}

impl Default for LibraryConfig {
    fn default() -> Self {
        Self {
            library_path: PathBuf::new(),
            load_kerml: true,
            load_sysml: true,
            load_domain: true,
            strict: false,
        }
    }
}

/// Load the standard library using the given parser.
///
/// This parses all library files and returns a combined ModelGraph with
/// all root packages registered as library packages.
///
/// # Arguments
///
/// * `parser` - The parser to use for parsing library files
/// * `config` - Library loading configuration
///
/// # Returns
///
/// A `ModelGraph` containing all library elements with root packages
/// registered as library packages.
pub fn load_standard_library<P: Parser>(
    parser: &P,
    config: &LibraryConfig,
) -> Result<ModelGraph, LibraryLoadError> {
    if !config.library_path.exists() {
        return Err(LibraryLoadError::PathNotFound(config.library_path.clone()));
    }

    let mut combined = ModelGraph::new();
    let mut total_errors = 0;

    // Load KerML kernel libraries first (they're foundational)
    if config.load_kerml {
        let kerml_dir = config.library_path.join("library.kernel");
        if kerml_dir.exists() {
            let (graph, errors) = load_files_from_dir(parser, &kerml_dir, "kerml")?;
            combined.merge(graph, false);
            total_errors += errors;
        }
    }

    // Then load SysML systems libraries
    if config.load_sysml {
        let sysml_dir = config.library_path.join("library.systems");
        if sysml_dir.exists() {
            let (graph, errors) = load_files_from_dir(parser, &sysml_dir, "sysml")?;
            combined.merge(graph, false);
            total_errors += errors;
        }
    }

    // Load domain libraries (ISQ, Geometry, etc.)
    if config.load_domain {
        let domain_dir = config.library_path.join("library.domain");
        if domain_dir.exists() {
            // Each subdirectory is a domain library (may have spaces in name)
            if let Ok(entries) = std::fs::read_dir(&domain_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        let (graph, errors) = load_files_from_dir(parser, &path, "sysml")?;
                        combined.merge(graph, false);
                        total_errors += errors;
                    }
                }
            }
        }
    }

    // Rebuild indexes after merging
    combined.rebuild_indexes();

    // Register all root packages as library packages
    register_library_packages(&mut combined);

    if config.strict && total_errors > 0 {
        return Err(LibraryLoadError::ParseErrors(total_errors));
    }

    Ok(combined)
}

/// Load all files from a directory with the given extension.
fn load_files_from_dir<P: Parser>(
    parser: &P,
    dir: &Path,
    extension: &str,
) -> Result<(ModelGraph, usize), LibraryLoadError> {
    let mut files = Vec::new();

    for entry in WalkDir::new(dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == extension) {
            let content = std::fs::read_to_string(path).map_err(|e| LibraryLoadError::ReadError {
                path: path.to_path_buf(),
                source: e,
            })?;

            let relative = path
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| path.to_string_lossy().to_string());

            files.push(SysmlFile::new(relative, content));
        }
    }

    // Parse all files
    let mut combined = ModelGraph::new();
    let mut error_count = 0;

    for file in &files {
        let result = parser.parse(&[file.clone()]);

        if result.has_errors() {
            error_count += result.error_count();
            // Still merge partial results
        }

        // Merge into combined graph
        for (id, element) in result.graph.elements {
            combined.elements.insert(id, element);
        }
        for (id, rel) in result.graph.relationships {
            combined.relationships.insert(id, rel);
        }
    }

    Ok((combined, error_count))
}

/// Register all root packages as library packages.
fn register_library_packages(graph: &mut ModelGraph) {
    // Collect root package IDs first to avoid borrow issues
    let root_package_ids: Vec<_> = graph
        .elements
        .values()
        .filter(|e| {
            e.owner.is_none()
                && (e.kind == ElementKind::Package
                    || e.kind == ElementKind::LibraryPackage
                    || e.kind.is_subtype_of(ElementKind::Package))
        })
        .map(|e| e.id.clone())
        .collect();

    // Register each root package
    for id in root_package_ids {
        graph.register_library_package(id);
    }
}

/// Get statistics about a loaded library.
#[derive(Debug, Clone, Default)]
pub struct LibraryStats {
    /// Number of elements in the library.
    pub element_count: usize,
    /// Number of registered library packages.
    pub package_count: usize,
    /// Names of registered library packages.
    pub package_names: Vec<String>,
}

impl LibraryStats {
    /// Compute statistics for a library graph.
    pub fn from_graph(graph: &ModelGraph) -> Self {
        let package_names: Vec<String> = graph
            .library_package_elements()
            .filter_map(|e| e.name.clone())
            .collect();

        Self {
            element_count: graph.element_count(),
            package_count: graph.library_packages().len(),
            package_names,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::StubParser;

    #[test]
    fn library_config_default() {
        let config = LibraryConfig::default();
        assert!(config.load_kerml);
        assert!(config.load_sysml);
        assert!(config.load_domain);
        assert!(!config.strict);
    }

    #[test]
    fn library_config_from_corpus() {
        let config = LibraryConfig::from_corpus_path("/some/corpus/path");
        // Path should NOT contain "library.kernel" (that's a subdir)
        assert!(!config.library_path.to_string_lossy().contains("library.kernel"));
        // Path should contain the test directory
        assert!(config
            .library_path
            .to_string_lossy()
            .contains("org.omg.sysml.xpect.tests"));
    }

    #[test]
    fn load_nonexistent_path() {
        let parser = StubParser::new();
        let config = LibraryConfig::new("/nonexistent/path");
        let result = load_standard_library(&parser, &config);
        assert!(matches!(result, Err(LibraryLoadError::PathNotFound(_))));
    }

    #[test]
    fn register_library_packages_empty() {
        let mut graph = ModelGraph::new();
        register_library_packages(&mut graph);
        assert!(graph.library_packages().is_empty());
    }
}
