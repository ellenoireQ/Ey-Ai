use crate::traits::ModelProvider;
use anyhow::Result;
use serde_json::Value;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct ModelClient {
    key: Arc<Mutex<String>>,
    model: Arc<Mutex<String>>,
    provider: Arc<dyn ModelProvider>,
}

impl ModelClient {
    pub fn new(provider: Arc<dyn ModelProvider>) -> Self {
        Self {
            key: Arc::new(Mutex::new(String::new())),
            model: Arc::new(Mutex::new(String::new())),
            provider,
        }
    }

    pub fn init(&self, api_key: String, model_name: String) {
        *self.key.lock().unwrap() = api_key;
        *self.model.lock().unwrap() = model_name;
    }

    pub async fn generate_text(&self, prompt: String) -> Result<String> {
        let key = self.key.lock().unwrap().clone();
        let model = self.model.lock().unwrap().clone();
        self.provider.generate_text(&key, &model, prompt).await
    }

    pub fn generate_sync(&self, prompt: String) -> Result<Value> {
        let key = self.key.lock().unwrap().clone().to_string();
        let model = self.model.lock().unwrap().clone().to_string();
        self.provider.generate_without_async(key, model, prompt)
    }
}
