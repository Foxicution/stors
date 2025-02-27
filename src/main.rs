use server::start_lsp_server;

mod server;

fn main() {
    start_lsp_server().expect("Starting the lsp server failed!");
}
