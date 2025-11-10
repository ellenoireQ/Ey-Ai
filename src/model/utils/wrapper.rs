use axum::{Json, extract::State};
use serde_json::Value;

use crate::model::{message::message::Message, models::Gemini::{GeminiClient, PromptInput}};

// A wrapper function for EY-Ai integration
// it will handle the key management and request generation
pub async fn eyai_wrapper(State(gemini): State<GeminiClient>, Json(input): Json<PromptInput>) -> Json<Message> {
    // Using the question in the state
    let prompt = input.prompt.clone();
    gemini.generate(prompt).await
}