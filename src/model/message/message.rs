use serde::{Deserialize, Serialize};

/*
Message:
 return example:
    {
    "id":"pass",
    "models":"pass",
    "question":"Explain how AI works in a few words",
    "choice":{
        "role":{
            "role":"pass",
            "content":"AI learns patterns from data to make decisions or predictions."
            }
        },
    "timestamp":"pass",
    "loading":true
    }
    NOTE: For now return json will be passed
*/
#[derive(Serialize, Deserialize, Default)]
pub struct Message {
    pub id: String,
    pub models: String,
    pub question: String,
    pub choice: Choice,
    pub timestamp: String,
    pub loading: bool,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Choice {
    pub role: Role,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Role {
    pub role: String,
    pub content: String,
}

