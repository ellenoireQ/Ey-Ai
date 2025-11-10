use std::env;

use axum::{ Router, routing::{post}};
use dotenvy::dotenv;
use crate::model::{models::Gemini::GeminiClient, utils::wrapper::eyai_wrapper,};
mod model;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let gemini = GeminiClient::new();
    gemini.initiate(env::var("GEMINI_API_KEY").unwrap());

    let app = Router::new()
        .route("/generate", post(eyai_wrapper))
        .with_state(gemini.clone());

    let listener = tokio::net::TcpListener::bind("localhost:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
