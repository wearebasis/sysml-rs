use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

/// Errors returned by visualization helpers.
#[derive(Debug)]
pub enum VisError {
    Io(std::io::Error),
    MissingExecutable { name: String },
    CommandFailed { command: String, status: Option<i32>, stderr: String },
}

impl std::fmt::Display for VisError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VisError::Io(err) => write!(f, "io error: {}", err),
            VisError::MissingExecutable { name } => write!(f, "missing executable: {}", name),
            VisError::CommandFailed {
                command,
                status,
                stderr,
            } => write!(
                f,
                "command failed: {} (status: {:?}) {}",
                command, status, stderr
            ),
        }
    }
}

impl std::error::Error for VisError {}

impl From<std::io::Error> for VisError {
    fn from(err: std::io::Error) -> Self {
        VisError::Io(err)
    }
}

/// Graphviz output formats.
#[derive(Debug, Clone, Copy)]
pub enum GraphvizFormat {
    Svg,
    Png,
    Pdf,
}

impl GraphvizFormat {
    fn as_str(self) -> &'static str {
        match self {
            GraphvizFormat::Svg => "svg",
            GraphvizFormat::Png => "png",
            GraphvizFormat::Pdf => "pdf",
        }
    }
}

/// Graphviz layout engines.
#[derive(Debug, Clone, Copy)]
pub enum GraphvizEngine {
    Dot,
    Neato,
    Fdp,
    Sfdp,
}

impl GraphvizEngine {
    fn as_str(self) -> &'static str {
        match self {
            GraphvizEngine::Dot => "dot",
            GraphvizEngine::Neato => "neato",
            GraphvizEngine::Fdp => "fdp",
            GraphvizEngine::Sfdp => "sfdp",
        }
    }
}

/// Graphviz rendering options.
#[derive(Debug, Clone, Copy)]
pub struct GraphvizOptions {
    pub engine: GraphvizEngine,
}

impl Default for GraphvizOptions {
    fn default() -> Self {
        GraphvizOptions {
            engine: GraphvizEngine::Dot,
        }
    }
}

/// Render DOT using the Graphviz `dot` binary.
pub fn render_dot(
    dot: &str,
    format: GraphvizFormat,
    options: GraphvizOptions,
) -> Result<Vec<u8>, VisError> {
    let mut cmd = Command::new("dot");
    cmd.arg(format!("-T{}", format.as_str()))
        .arg(format!("-K{}", options.engine.as_str()))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child = match cmd.spawn() {
        Ok(child) => child,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            return Err(VisError::MissingExecutable {
                name: "dot".to_string(),
            });
        }
        Err(err) => return Err(VisError::Io(err)),
    };

    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(dot.as_bytes())?;
    }

    let output = child.wait_with_output()?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        return Err(VisError::CommandFailed {
            command: "dot".to_string(),
            status: output.status.code(),
            stderr,
        });
    }

    Ok(output.stdout)
}

/// Render DOT to an SVG string using Graphviz.
pub fn render_dot_to_svg(dot: &str) -> Result<String, VisError> {
    let bytes = render_dot(dot, GraphvizFormat::Svg, GraphvizOptions::default())?;
    Ok(String::from_utf8_lossy(&bytes).to_string())
}

/// Render DOT to a PNG file using Graphviz.
pub fn render_dot_to_png(dot: &str, path: impl AsRef<Path>) -> Result<(), VisError> {
    let bytes = render_dot(dot, GraphvizFormat::Png, GraphvizOptions::default())?;
    std::fs::write(path, bytes)?;
    Ok(())
}

/// Render DOT to a PDF file using Graphviz.
pub fn render_dot_to_pdf(dot: &str, path: impl AsRef<Path>) -> Result<(), VisError> {
    let bytes = render_dot(dot, GraphvizFormat::Pdf, GraphvizOptions::default())?;
    std::fs::write(path, bytes)?;
    Ok(())
}
