use anyhow::Result;
use async_trait::async_trait;
use axum::Json;
use futures::future::BoxFuture;
use serde_json::Value;

use crate::{
    model::message::message::Message,
    model_llm::{GeminiModel, ModelLLM},
};

/// A trait that defines the contract for a Large Language Model (LLM) provider.
///
/// This trait abstracts the common functionalities required to interact with
/// various LLM APIs, such as generating text, handling streaming responses,
/// and managing model configurations. Implementors of this trait can be used
/// interchangeably through the `ModelClient`.
///
/// The `Send` and `Sync` bounds are required to allow the provider to be
/// shared safely across threads, which is essential for asynchronous runtimes
/// like Axum.
#[async_trait]
pub trait ModelProvider: Send + Sync {
    /// Creates a new instance of the provider.
    ///
    /// This function should be implemented to return a new, initialized
    /// instance of the struct that implements this trait.
    fn new() -> Self
    where
        Self: Sized;

    /// Asynchronously generates a text response from the LLM Providers.
    ///
    /// This is the primary method for non-streaming text generation and is designed
    /// for use in asynchronous applications (e.g., web servers).
    ///
    /// # Arguments
    /// * `api_key` - The API key for authenticating with the provider's service.
    /// * `model` - The specific model to use for generation (e.g., "gemini-1.5-flash").
    /// * `prompt` - The input text to send to the model.
    ///
    /// # Returns
    /// A `Result` containing the generated `String` on success, or an `anyhow::Error` on failure.
    async fn generate_text(&self, api_key: &str, model: &str, prompt: String) -> Result<String>;
    //async fn generate(&self, prompt: String) -> Json<Message>;

    /// Synchronously (blocking) generates a text response from the LLM Providers.
    ///
    /// This method is suitable for use cases where an async runtime is not available
    /// or necessary, such as in command-line tools or simple scripts.
    ///
    /// # Arguments
    /// * `api_key` - The API key for authenticating with the provider's service.
    /// * `model` - The specific model to use for generation.
    /// * `prompt` - The input text to send to the model.
    ///
    /// # Returns
    /// A `Result` containing a `serde_json::Value` on success, or an `anyhow::Error` on failure.
    fn generate_without_async(
        &self,
        api_key: String,
        model: String,
        prompt: String,
    ) -> Result<Value>;
    /// Asynchronously generates a streaming response from the LLM Providers.
    ///
    /// This function is intended for real-time applications where the response
    /// should be processed in chunks as it's being generated (e.g., for a chatbot UI).
    ///
    /// * Placeholder
    async fn generate_stream(&self, api_key: &str, model: &str, prompt: String) -> Result<()>;
}
