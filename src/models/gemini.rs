use std::sync::{Arc, Mutex};

use anyhow::Ok;
use axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use anyhow::{anyhow, Result};
use crate::model::message::message::{Choice, Message, Role};

#[derive(Clone)]
pub struct GeminiClient {
    key: Arc<Mutex<Option<String>>>,
}

#[derive(Serialize, Deserialize)]
pub struct PromptInput {
    pub prompt: String,
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

    /// Generate text
    ///
    /// # State:
    /// - WIP: This function is currently experimental.
    /// - May soon become the default handler for REST API requests.
    ///
    /// # Description:
    /// Sends a prompt to the Gemini model and returns the generated text.
    /// This version uses a simple JSON body and is intended for lightweight REST usage.
    ///
    /// # Returns
    /// A `Result<String>` will return 'content', see /websocket/websocket.rs: handle_socket()...match client.generate_text
    pub async fn generate_text(&self, prompt: String) -> Result<String> {
        let api_key = self.get_key();

        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash:generateContent?key={}",
            api_key
        );

        let body = json!({
            "contents": [{
                "parts": [{
                    "text": prompt
                }]
            }]
        });
        
        let req = reqwest::Client::new();
        let res = req
            .post(&url)
            .json(&body)
            .header("Content-Type", "application/json")
            .send()
            .await
            .map_err(|e| anyhow!("Failed to send request: {}", e))?;

        let json: Value = res
            .json()
            .await
            .map_err(|e| anyhow!("Failed to parse response: {}", e))?;

        let reply = json["candidates"]
            .get(0)
            .and_then(|c| c["content"]["parts"].get(0))
            .and_then(|p| p["text"].as_str())
            .ok_or_else(|| anyhow!("No response from Gemini"))?
            .to_string();

        Ok(reply)
    }

    /// Generate v1.
    ///
    /// # State
    /// Deprecated: This version will be removed in future releases.
    /// Use [`generate_text`], see the function in this file.
    pub async fn generate(&self, prompt: String) -> Json<Message> {
        let req = reqwest::Client::new();
        let api_key = self.get_key();

         let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash:generateContent?key={}",
            api_key
        );

        let body = json!({
            "contents": [{
                "parts": [{
                    "text": prompt
                }]
            }]
        });
        
        let res = req.post(&url).json(&body).header("Content-Type", "application/json").send().await.expect("");
        let json: Value = res.json().await.expect("");
        let reply = json["candidates"]
                    .get(0)
                    .and_then(|c| c["content"]["parts"].get(0))
                    .and_then(|p| p["text"].as_str())
                    .unwrap_or("Not responded")
                    .to_string();
                
        let message = Message{
            id: "pass".to_string(),
            models: "pass".to_string(),
            question: prompt,
            choice: Choice{
                role: Role{
                    role: "pass".to_string(),
                    content: reply
                }
            },
            timestamp: "pass".to_string(),
            loading: true,
        };
        Json(message)
    }

    /// Generate text using the Gemini API (synchronous version).
    ///
    /// # Overview
    /// This method provides a blocking (synchronous) alternative to [`generate`].
    /// It can be useful for contexts where asynchronous execution is not available,
    /// such as command-line tools or background worker threads.
    ///
    /// # Returns
    /// A [`Result<Value>`] containing the model's response message on success,
    /// or an [`anyhow::Error`] if the request or parsing fails.
    ///
    /// # Example
    /// ```ignore
    /// let gemini = GeminiClient::new();
    /// gemini.initiate("API_KEY".into());
    /// let result = gemini.generate_without_async("Hello Gemini!".into())?;
    /// println!("{}", result);
    /// ```
    ///
    /// # Notes
    /// - Despite the name, this method is blocking.
    /// - For non-blocking behavior, use [`generate_text`] instead.
    pub fn generate_without_async(&self, prompt: String) -> anyhow::Result<Value>{
        let req = reqwest::blocking::Client::new();
        let api_key = self.get_key();

        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash:generateContent?key={}",
            api_key
        );

        let body = json!({
            "contents": [{
                "parts": [{
                    "text": prompt
                }]
            }]
        });

        let res = req.post(&url).json(&body).header("Content-Type", "application/json").send()?.json::<Value>()?;
        let reply = res["candidates"]
                    .get(0)
                    .and_then(|c| c["content"]["parts"].get(0))
                    .and_then(|p| p["text"].as_str())
                    .unwrap_or("Not responded")
                    .to_string();

        let message = Message{
            id: "pass".into(),
            models: "pass".into(),
            question: prompt,
            choice: Choice{
                role: Role{
                    role: "pass".into(),
                    content: reply
                }
            },
            timestamp: "pass".into(),
            loading: true,
        };
        Ok(json!(message))
    }
}
