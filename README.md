# 🌌 Ey-AI

`Ey-AI` is a **lightweight**, **modular**, and **high-performance** chatbot REST API framework written in Rust.  
It is designed for simplicity, flexibility, and speed — making it easy to integrate powerful AI model APIs like **Google Gemini**, **OpenAI**, and others.

---

## 🚀 Features

- ⚡ Built on top of [`axum`](https://docs.rs/axum/latest/axum/) and [`reqwest`](https://docs.rs/reqwest/latest/reqwest/)
- 🧩 ~~Modular structure (supports multiple AI backends)~~ (Future plan)
- 🔒 Safe API key management
- 🧠 Simple interface for generating AI responses
- 🔧 Ready to extend into both **async backend** and **blocking CLI** environments

---

## 🧰 Example Usage

```rust
use ey_ai::models::Gemini::GeminiClient;

#[tokio::main]
async fn main() {
    let gemini = GeminiClient::new();
    gemini.initiate("YOUR_API_KEY".to_string());

    let response = gemini.generate("Hi, how are you?".to_string()).await;
    println!("{:?}", response);
}
```

## General Usage

```rust
fn main(){
    // initiate Client before generate
    let gemini = GeminiClient::new();
    gemini.initiate("YOUR_API_KEY".unwrap());

    let response = gemini.generate_without_async("Hai".to_string());
    // will return json
    println!("{:?}", response);
}
```

## ⚠️ This documentation is not yet complete

> Unstable
