// Model-LLVM enumeration
// in FUTURE plan would be adding other llvm
pub enum ModelLLVM{
    // Google Gemini
    // ref: https://ai.google.dev/gemini-api/docs/models
    Gemini25Flash,
    Gemini25Pro,
    Gemini25FlashLite
}

// selector:
// this function is for matching enum model & return into string
pub fn selector(model: ModelLLVM) -> String{
    match model{
        ModelLLVM::Gemini25Flash => "gemini-2.5-flash".to_string(),
        ModelLLVM::Gemini25Pro => "gemini-2.5-pro".to_string(),
        ModelLLVM::Gemini25FlashLite => "gemini-2.5-flash-lite".to_string(),
    }
}