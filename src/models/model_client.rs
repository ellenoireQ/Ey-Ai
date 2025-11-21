use crate::{model_llm::Models, traits::ModelProvider};
use anyhow::Result;
use serde_json::Value;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct ModelClient {
    pub key: Arc<Mutex<String>>,
    pub model: Arc<Mutex<String>>,
    pub provider: Arc<dyn ModelProvider>,
}

impl ModelClient {
    pub fn new(provider: Arc<dyn ModelProvider>) -> Self {
        Self {
            key: Arc::new(Mutex::new(String::new())),
            model: Arc::new(Mutex::new(String::new())),
            provider,
        }
    }

    pub fn init(&self, api_key: String, model_name: Models) -> Self {
        *self.key.lock().unwrap() = api_key;

        let match_model = match model_name {
            Models::Gemini25Flash => "gemini-2.5-flash".to_string(),
            Models::Gemini25Pro => "gemini-2.5-pro".to_string(),
            Models::Gemini25FlashLite => "gemini-2.5-flash-lite".to_string(),
            Models::Gemini3ProPreview => "gemini-3-pro-preview".to_string(),
        };

        *self.model.lock().unwrap() = match_model;
        self.clone()
    }

    pub async fn GenerateContent(&self, prompt: String) -> Result<String> {
        let key = self.key.lock().unwrap().clone();
        let model = self.model.lock().unwrap().clone();
        self.provider.generate_text(&key, &model, prompt).await
    }

    pub fn GenerateSyncContent(&self, prompt: String) -> Result<Value> {
        let key = self.key.lock().unwrap().clone().to_string();
        let model = self.model.lock().unwrap().clone().to_string();
        self.provider.generate_without_async(key, model, prompt)
    }
}
