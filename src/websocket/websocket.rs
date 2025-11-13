use axum::{
    extract::{
        ws::{Message as WsMessage, WebSocket},
        State, WebSocketUpgrade,
    },
    response::IntoResponse,
};
use serde_json::json;
use crate::models::gemini::GeminiClient;


// websocket_handler:
// Main function websocket to handling realtime connection
// WIP model
pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(client): State<GeminiClient>,
) -> impl IntoResponse {
    ws.on_failed_upgrade(|error| {
        eprintln!("WebSocket upgrade failed: {}", error);
    })
    .on_upgrade(move |socket| handle_socket(socket, client))
}

pub async fn handle_socket(mut socket: WebSocket, client: GeminiClient) {
    println!("New WebSocket connection established");

    let welcome = json!({
        "type": "connection",
        "status": "connected",
        "message": "Connected to EY-AI WebSocket"
    });
    
    if let Err(e) = socket.send(WsMessage::Text(welcome.to_string().into())).await {
        eprintln!("Failed to send welcome message: {}", e);
        return;
    }

    while let Some(msg) = socket.recv().await {
        match msg {
            Ok(WsMessage::Text(text)) => {
                println!("Received message: {}", text);
                
                let loading_msg = json!({
                    "type": "status",
                    "loading": true,
                    "content": ""
                });
                
                if let Err(e) = socket.send(WsMessage::Text(loading_msg.to_string().into())).await {
                    eprintln!("Failed to send loading message: {}", e);
                    break;
                }

                match client.generate_text(text.to_string()).await {
                    Ok(reply) => {
                        let response_msg = json!({
                            "type": "response",
                            "loading": false,
                            "content": reply,
                        });
                        
                        if let Err(e) = socket.send(WsMessage::Text(response_msg.to_string().into())).await {
                            eprintln!("Failed to send response: {}", e);
                            break;
                        }
                    }
                    Err(e) => {
                        eprintln!("Generation error: {}", e);
                        let error_msg = json!({
                            "type": "error",
                            "loading": false,
                            "content": "",
                            "error": e.to_string()
                        });
                        
                        if let Err(e) = socket.send(WsMessage::Text(error_msg.to_string().into())).await {
                            eprintln!("Failed to send error message: {}", e);
                            break;
                        }
                    }
                }
            }
            Ok(WsMessage::Close(_)) => {
                println!("Client closed connection");
                break;
            }
            Ok(WsMessage::Ping(data)) => {
                if let Err(e) = socket.send(WsMessage::Pong(data)).await {
                    eprintln!("Failed to send pong: {}", e);
                    break;
                }
            }
            Ok(_) => {
               
            }
            Err(e) => {
                eprintln!("WebSocket error: {}", e);
                break;
            }
        }
    }

    println!("WebSocket connection closed");
}