use std::sync::{Arc, Mutex};

use axum::Json;
use serde_json::{Value, json};

#[derive(Clone)]
pub struct GeminiClient {
    key: Arc<Mutex<Option<String>>>,
}

impl GeminiClient {
    pub fn new() -> Self {
        Self {
            key: Arc::new(Mutex::new(None)),
        }
    }

    pub fn initiate(&self, env: String) {
        let mut key = self.key.lock().unwrap();
        *key = Some(env);
    }

    pub fn get_key(&self) -> String {
        let key = self.key.lock().unwrap();
        key.as_ref().unwrap().clone()
    }

    pub async fn generate(&self, prompt: String) -> Json<Value> {
        let req = reqwest::Client::new();

        let res = req.get("https://jsonplaceholder.typicode.com/todos/1").send().await.expect("");
        let json: Value = res.json().await.expect("Gagal parse JSON");
        Json(json)
    }
}
