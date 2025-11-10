use axum::{Json, Router, extract::State, routing::get};
use serde_json::Value;

use crate::model::models::Gemini::GeminiClient;
mod model;

async fn handler(State(api): State<GeminiClient>) -> Json<Value> {
    api.generate("halo dunia".to_string()).await
}

#[tokio::main]
async fn main() {
    let gemini = GeminiClient::new();
    gemini.initiate("H".to_string());
    let key = gemini.generate("Hello, Gemini!".to_string());
    println!("Gemini key initiated: {:?}", key.await);

    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(handler))
        .with_state(gemini.clone());


    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("localhost:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
