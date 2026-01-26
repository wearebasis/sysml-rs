use sysml_lsp_server::run_stdio;

#[tokio::main]
async fn main() {
    run_stdio().await;
}
