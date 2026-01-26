# Zed SysML Extension (local)

Minimal Zed extension that wires up the SysML LSP and the in-repo Tree-sitter grammar from
`sysml-ts/tree-sitter` for syntax highlighting.

## How it works
- Registers the `SysML` language for `.sysml` / `.kerml`
- Attaches the `sysml-lsp-server` language server
- Uses the `sysml-ts` Tree-sitter grammar for highlighting

## LSP binary
The extension will try, in order:
1) `lsp.sysml-lsp.binary.path` from Zed settings
2) `sysml-lsp-server` on `PATH`
3) `${WORKTREE}/sysml-rs/target/debug/sysml-lsp-server`
4) `${WORKTREE}/target/debug/sysml-lsp-server`

To explicitly point Zed at your local build, add this to Zed settings:

```
{
  "lsp": {
    "sysml-lsp": {
      "binary": {
        "path": "/home/ricky/personal_repos/sysml-rs/sysml-rs/target/debug/sysml-lsp-server",
        "arguments": []
      }
    }
  }
}
```

Build the server with:

```
cargo build -p sysml-lsp-server
```

## Tree-sitter
Zed requires a Tree-sitter grammar to register a language. This extension points at
the in-repo grammar in `sysml-ts/tree-sitter`. If your repo lives elsewhere,
update the `repository` path in `extension.toml`.

Ensure the generated parser exists:

```
cd sysml-ts/tree-sitter
tree-sitter generate --abi=14
```

If you previously installed the extension with a different grammar source, remove the
cached grammar directory:
```
rm -rf sysml-lsp-zed-extension/grammars/sysml
```
