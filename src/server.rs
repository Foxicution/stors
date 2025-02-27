use lsp_server::{Connection, Message, Request, Response};
use lsp_types::{
    Hover, HoverContents, HoverParams, HoverProviderCapability, InitializeParams, MarkupContent,
    MarkupKind, ServerCapabilities, TextDocumentSyncCapability, TextDocumentSyncKind,
};
use serde_json::json;

pub fn start_lsp_server() -> Result<(), Box<dyn std::error::Error>> {
    let (connection, io_threads) = Connection::stdio();

    let server_capabilities = serde_json::to_value(ServerCapabilities {
        text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
        hover_provider: Some(HoverProviderCapability::Simple(true)),
        ..Default::default()
    })?;

    let initialization_params = connection.initialize(server_capabilities)?;
    let _params: InitializeParams = serde_json::from_value(initialization_params)?;

    for msg in &connection.receiver {
        match msg {
            Message::Request(req) if connection.handle_shutdown(&req).unwrap_or(false) => {
                break;
            }
            Message::Request(req) => {
                if req.method == "textDocument/hover" {
                    if let Ok(hover_params) = serde_json::from_value::<HoverParams>(req.params) {
                        let hover_response = Hover {
                            contents: HoverContents::Markup(MarkupContent {
                                kind: MarkupKind::Markdown,
                                value: "This is a **Markdown** hover example.".to_string(),
                            }),
                            range: None,
                        };

                        let response = Response::new_ok(req.id, json!(hover_response));
                        connection.sender.send(Message::Response(response))?;
                    }
                }
            }
            _ => {}
        }
    }

    io_threads.join()?;
    Ok(())
}
