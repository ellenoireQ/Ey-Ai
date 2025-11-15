use std::sync::{Arc, Mutex, MutexGuard};

use crate::{
    model::message::message::{Choice, Message, Role},
    model_llm::{GeminiModel, ModelLLM},
    traits::ModelProvider,
    utils::select_model::selector,
};
use anyhow::Ok;
use anyhow::{Result, anyhow};
use async_trait::async_trait;
use axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

/// Input structure for receiving prompts from API requests.
///
/// # Fields
/// * `prompt` - String containing the text prompt to be sent to the Gemini model
///
/// # Example
/// ```json
/// {
///   "prompt": "Explain Rust programming"
/// }
/// ```
#[derive(Serialize, Deserialize)]
pub struct PromptInput {
    pub prompt: String,
}

/// Provider implementation for Google Gemini API.
///
/// `GeminiProvider` implements the `ModelProvider` trait to provide concrete
/// implementation for communicating with Google Gemini language models. This struct
/// is designed to be used through the `ModelClient` wrapper which handles API key
/// and model name management.
///
/// # Usage
/// This provider should not be used directly. Instead, use it through `ModelClient`:
///
/// ```rust,no_run
///
/// let provider = Arc::new(GeminiProvider::new());
/// let client = ModelClient::new(provider);
/// client.init("YOUR_API_KEY".to_string(), "gemini-pro".to_string());
/// ```
///
/// # See Also
/// * [`ModelClient`] - The recommended wrapper for using this provider
/// * [`ModelProvider`] - The trait this struct implements
pub struct GeminiProvider;

impl GeminiProvider {
    /// Creates a new instance of `GeminiProvider`.
    ///
    /// # Returns
    /// A new instance of `GeminiProvider`
    ///
    /// No need to using this vanilla function
    /// consider to using selector(), see: select_model.rs
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl ModelProvider for GeminiProvider {
    fn new() -> Self {
        Self
    }

    /// Generates text using the Gemini API (asynchronous implementation).
    ///
    /// # Direct Usage Not Recommended
    /// This is a low-level implementation method. Use [`ModelClient::generate_text`] instead,
    /// which provides a cleaner interface and handles API key/model management automatically.
    ///
    /// # Arguments
    /// * `api_key` - The API key for authenticating with Google Gemini API
    /// * `model` - The name of the Gemini model to use (e.g., "gemini-2.5-flash")
    /// * `prompt` - The text prompt to send to the model
    ///
    /// # Returns
    /// * `Ok(String)` - The generated text response from the model
    ///
    /// # Errors
    /// This function will return an error if:
    /// * The HTTP request fails to send
    /// * The response cannot be parsed as JSON
    /// * The response doesn't contain the expected structure
    /// * The API key is invalid or expired
    ///
    /// # API Details
    /// **Endpoint:**
    /// ```text
    /// POST https://generativelanguage.googleapis.com/v1beta/models/{model}:generateContent?key={api_key}
    /// ```
    ///
    /// **Request Body:**
    /// ```json
    /// {
    ///   "contents": [{
    ///     "parts": [{
    ///       "text": "your prompt here"
    ///     }]
    ///   }]
    /// }
    /// ```
    ///
    /// **Response Parsing:**
    /// Extracts text from: `response.candidates[0].content.parts[0].text`
    ///
    /// # Recommended Usage Pattern
    /// ```rust,no_run
    /// # async fn example() {
    /// dotenv.ok();
    /// // Recommended: Use ModelClient wrapper
    /// let gemini = selector(ModelLLM::Gemini);
    ///  gemini.init(
    ///    env::var("GEMINI_API_KEY").unwrap(),
    ///    "gemini-2.5-flash".into(),
    ///  );
    ///
    //  let _ = println!("{:?}", gemini.generate_sync("Hello!".to_string()));
    /// # }
    /// ```
    async fn generate_text(&self, api_key: &str, model: &str, prompt: String) -> Result<String> {
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
            model, api_key
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

    /*
    /// Generate v1 (Deprecated).
    ///
    /// # Status
    /// **Deprecated**: This version will be removed in future releases.
    /// Use [`generate_text`] through [`ModelClient`] instead.
    async fn generate(&self, prompt: String) -> Json<Message> {
        let req = reqwest::Client::new();
        let (api_key, model) = self.get_property();

        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
            model, api_key
        );

        let body = json!({
            "contents": [{
                "parts": [{
                    "text": prompt
                }]
            }]
        });

        let res = req
            .post(&url)
            .json(&body)
            .header("Content-Type", "application/json")
            .send()
            .await
            .expect("");
        let json: Value = res.json().await.expect("");
        let reply = json["candidates"]
            .get(0)
            .and_then(|c| c["content"]["parts"].get(0))
            .and_then(|p| p["text"].as_str())
            .unwrap_or("Not responded")
            .to_string();

        let message = Message {
            id: "pass".to_string(),
            models: "pass".to_string(),
            question: prompt,
            choice: Choice {
                role: Role {
                    role: "pass".to_string(),
                    content: reply,
                },
            },
            timestamp: "pass".to_string(),
            loading: true,
        };
        Json(message)
    }*/

    /// Generates text using the Gemini API (synchronous/blocking implementation).
    ///
    /// # Direct Usage Not Recommended
    /// This is a low-level implementation method. Use [`ModelClient::generate_sync`] instead,
    /// which provides a cleaner interface and handles API key/model management automatically.
    ///
    /// # Arguments
    /// * `api_key` - The API key for authenticating with Google Gemini API
    /// * `model` - The name of the Gemini model to use (e.g., "gemini-2.5-flash")
    /// * `prompt` - The text prompt to send to the model
    ///
    /// # Returns
    /// * `Ok(Value)` - A JSON value containing the [`Message`] object with the model's response
    /// * `Err(anyhow::Error)` - If the request or parsing fails
    ///
    /// # Response Structure
    /// The returned JSON contains a `Message` object with the following structure:
    /// ```rust,ignore
    /// Message {
    ///     id: "pass",
    ///     models: "pass",
    ///     question: prompt,
    ///     choice: Choice {
    ///         role: Role {
    ///             role: "pass",
    ///             content: reply_from_gemini
    ///         }
    ///     },
    ///     timestamp: "pass",
    ///     loading: true
    /// }
    /// ```
    ///
    /// # Warning: Blocking Operation
    /// This method is **blocking** and will halt the current thread until a response
    /// is received. It uses `reqwest::blocking::Client` internally.
    ///
    /// # Use Cases
    /// Suitable for:
    /// - Command-line interface (CLI) tools
    /// - Background worker threads without async runtime
    /// - Batch processing scripts
    /// - Testing and debugging
    ///
    /// **Not suitable for:**
    /// - Web servers (use async version instead)
    /// - Applications requiring high concurrency
    ///
    ///
    /// # Performance Considerations
    /// This method blocks the current thread. For web servers or applications that need
    /// to handle concurrent requests efficiently, use [`generate_text`] through
    /// [`ModelClient::generate_text`] instead.
    fn generate_without_async(
        &self,
        api_key: String,
        model: String,
        prompt: String,
    ) -> anyhow::Result<Value> {
        let req = reqwest::blocking::Client::new();

        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
            model, api_key
        );

        let body = json!({
            "contents": [{
                "parts": [{
                    "text": prompt
                }]
            }]
        });

        let res = req
            .post(&url)
            .json(&body)
            .header("Content-Type", "application/json")
            .send()?
            .json::<Value>()?;
        let reply = res["candidates"]
            .get(0)
            .and_then(|c| c["content"]["parts"].get(0))
            .and_then(|p| p["text"].as_str())
            .unwrap_or("Not responded")
            .to_string();

        let message = Message {
            id: "pass".into(),
            models: "pass".into(),
            question: prompt,
            choice: Choice {
                role: Role {
                    role: "pass".into(),
                    content: reply,
                },
            },
            timestamp: "pass".into(),
            loading: true,
        };
        Ok(json!(message))
    }
}
