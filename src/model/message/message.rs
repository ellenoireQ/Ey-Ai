use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct Message{
    pub id: String,
    pub models: String,
    pub question: String,
    pub choice: Choice,
    pub timestamp: String,
    pub loading: bool
}

#[derive(Serialize, Deserialize, Default)]
pub struct Choice{
    pub role: Role,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Role{
    pub role: String,
    pub content: String,
}