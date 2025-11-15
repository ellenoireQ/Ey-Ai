use anyhow::Result;
use async_trait::async_trait;
use axum::Json;
use futures::future::BoxFuture;
use serde_json::Value;

use crate::{
    model::message::message::Message,
    model_llm::{GeminiModel, ModelLLM},
};

#[async_trait]
pub trait ModelProvider: Send + Sync {
    fn new() -> Self
    where
        Self: Sized;

    async fn generate_text(&self, api_key: &str, model: &str, prompt: String) -> Result<String>;
    //async fn generate(&self, prompt: String) -> Json<Message>;
    fn generate_without_async(
        &self,
        api_key: String,
        model: String,
        prompt: String,
    ) -> Result<Value>;

    async fn generate_stream(&self, api_key: &str, model: &str, prompt: String) -> Result<()>;
}
