use crate::{
    model::message::message::{Choice, Message, Role},
    models::gemini::GeminiClient,
};
use axum::{
    extract::{
        State, WebSocketUpgrade,
        ws::{Message as WsMessage, WebSocket},
    },
    response::IntoResponse,
};
use serde_json::{json, to_string};

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

    if let Err(e) = socket
        .send(WsMessage::Text(welcome.to_string().into()))
        .await
    {
        eprintln!("Failed to send welcome message: {}", e);
        return;
    }

    while let Some(msg) = socket.recv().await {
        match msg {
            Ok(WsMessage::Text(text)) => {
                println!("Received message: {}", text);

                let loading_msg = Message {
                    id: "loading".into(),
                    models: "".into(),
                    question: text.to_string(),
                    choice: Choice {
                        role: Role {
                            role: "system".into(),
                            content: "".into(),
                        },
                    },
                    timestamp: "".into(),
                    loading: true,
                };

                let _ = socket
                    .send(WsMessage::Text(to_string(&loading_msg).unwrap().into()))
                    .await;

                match client.generate_text(text.to_string()).await {
                    Ok(reply) => {
                        let response_msg = Message {
                            id: "response".into(),
                            models: "".into(),
                            question: text.to_string(),
                            choice: Choice {
                                role: Role {
                                    role: "assistant".into(),
                                    content: reply,
                                },
                            },
                            timestamp: "".into(),
                            loading: false,
                        };

                        let _ = socket
                            .send(WsMessage::Text(to_string(&response_msg).unwrap().into()))
                            .await;
                    }
                    Err(e) => {
                        let error_msg = Message {
                            id: "error".into(),
                            models: "".into(),
                            question: "".into(),
                            choice: Choice {
                                role: Role {
                                    role: "system".into(),
                                    content: e.to_string(),
                                },
                            },
                            timestamp: "".into(),
                            loading: false,
                        };

                        let _ = socket
                            .send(WsMessage::Text(to_string(&error_msg).unwrap().into()))
                            .await;
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
            Ok(_) => {}
            Err(e) => {
                eprintln!("WebSocket error: {}", e);
                break;
            }
        }
    }

    println!("WebSocket connection closed");
}
