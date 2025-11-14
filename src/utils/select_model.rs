use crate::utils::models::ModelLLM;

// selector:
// this function is for matching enum model & return into string
pub fn selector(model: ModelLLM) -> String{
    match model{
        ModelLLM::Gemini25Flash => "gemini-2.5-flash".to_string(),
        ModelLLM::Gemini25Pro => "gemini-2.5-pro".to_string(),
        ModelLLM::Gemini25FlashLite => "gemini-2.5-flash-lite".to_string(),
    }
}