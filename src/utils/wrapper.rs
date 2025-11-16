/*
use crate::{model::message::message::Message, models::gemini::PromptInput, traits::ModelProvider};
use axum::{Json, extract::State};


/// A wrapper function for EY-Ai integration.
///
/// This function will handle both the **API key management**
/// and request generation for model calls.
///
/// Future Plan
/// In future versions, this function is planned to become a universal interface
/// not limited to Gemini, allowing integration with multiple AI models
/// under a single standardized abstraction layer.
pub async fn eyai_wrapper(
    State(gemini): State<ModelClient>,
    Json(input): Json<PromptInput>,
) -> Json<Message> {
    let prompt = input.prompt.clone();
    gemini.generate_text(prompt).await
}
*/
