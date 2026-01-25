use std::path::Path;

use zed_extension_api as zed;

struct SysmlExtension;

impl zed::Extension for SysmlExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        if language_server_id.as_ref() != "sysml-lsp" {
            return Err(format!("Unknown language server: {}", language_server_id).into());
        }

        if let Ok(settings) = zed::settings::LspSettings::for_worktree("sysml-lsp", worktree) {
            if let Some(binary) = settings.binary {
                if let Some(path) = binary.path {
                    let command = if Path::new(&path).is_absolute() {
                        path
                    } else {
                        format!("{}/{}", worktree.root_path(), path)
                    };
                    let args = binary.arguments.unwrap_or_default();
                    let env = binary
                        .env
                        .unwrap_or_default()
                        .into_iter()
                        .collect::<Vec<(String, String)>>();
                    return Ok(zed::Command { command, args, env });
                }
            }
        }

        if let Some(path) = worktree.which("sysml-lsp-server") {
            return Ok(zed::Command {
                command: path,
                args: vec![],
                env: vec![],
            });
        }

        let root = worktree.root_path();
        let candidates = [
            format!("{root}/sysml-rs/target/debug/sysml-lsp-server"),
            format!("{root}/target/debug/sysml-lsp-server"),
        ];

        for command in candidates {
            return Ok(zed::Command {
                command,
                args: vec![],
                env: vec![],
            });
        }

        Err("sysml-lsp-server not found. Build it with `cargo build -p sysml-lsp-server` and either add `target/debug` to PATH or set `lsp.sysml-lsp.binary.path` in Zed settings.".into())
    }
}

zed::register_extension!(SysmlExtension);
