use crate::model::utils::models::ModelLLVM;

// selector:
// this function is for matching enum model & return into string
pub fn selector(model: ModelLLVM) -> String{
    match model{
        ModelLLVM::Gemini25Flash => "gemini-2.5-flash".to_string(),
        ModelLLVM::Gemini25Pro => "gemini-2.5-pro".to_string(),
        ModelLLVM::Gemini25FlashLite => "gemini-2.5-flash-lite".to_string(),
    }
}