use std::sync::Arc;

use crate::{
    model_llm::ModelLLM,
    models::{gemini::GeminiProvider, model_client::ModelClient},
    traits::ModelProvider,
};

// selector:
// this function is for matching enum model & return into string
pub fn selector(model: ModelLLM) -> ModelClient {
    match model {
        ModelLLM::Gemini => ModelClient::new(Arc::new(GeminiProvider::new())),
    }
}
