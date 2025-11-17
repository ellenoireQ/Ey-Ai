use std::{convert::Infallible, sync::Arc};

use axum::{
    Json,
    extract::State,
    response::{Sse, sse::Event},
};
use futures::Stream;

use crate::models::gemini::PromptInput;
use crate::models::model_client::ModelClient;

/// Handles streaming AI model responses
///
/// This handler receives a prompt from the client and streams the AI model's response
/// in real-time using SSE. Each chunk of the response is sent as a separate event.
///
/// # Arguments
///
/// * `client` - Shared application state containing the model client configuration
///   - `key`: API key for authentication (Mutex-wrapped)
///   - `model`: Model identifier to use (Mutex-wrapped)
///   - `provider`: AI provider instance for generating responses
///
/// * `input` - JSON request body containing the user's prompt
///   ```json
///   {
///     "prompt": "Your question here"
///   }
///   ```
///
/// # Returns
///
/// Returns an SSE stream where each event contains a text chunk from the AI response.
/// The stream never returns errors to the client (uses `Infallible` type).
///
/// # Behavior
///
/// 1. Extracts API key and model name from shared state
/// 2. Initiates streaming generation from the provider
/// 3. Transforms provider chunks into SSE events
/// 4. Converts any chunk errors into error messages sent to the client
///
/// # Example Request
///
/// ```http
/// POST /generate-stream
/// Content-Type: application/json
///
/// {
///   "prompt": "Explain Rust ownership"
/// }
/// ```
///
/// # Example Response (SSE)
///
/// ```json
/// data: Rust ownership is
///
/// data: a system that ensures
///
/// data: memory safety...
/// ```
pub async fn GenerateStreamResponse(
    State(client): State<Arc<ModelClient>>,
    Json(input): Json<PromptInput>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let api_key = client.key.lock().unwrap().clone();
    let model = client.model.lock().unwrap().clone();

    // Initiate streaming generation from the AI provider
    let base_stream = client
        .provider
        .generate_stream(api_key, model, input.prompt)
        .await
        .unwrap();

    // Transform provider stream into SSE events
    // Errors in individual chunks are converted to error messages
    let sse_stream = futures::stream::StreamExt::map(base_stream, |chunk| {
        let text = chunk.unwrap_or_else(|e| format!("error: {e}"));
        Ok::<Event, Infallible>(Event::default().data(text))
    });

    Sse::new(sse_stream)
}
