
# Ey-AI

`Ey-AI` is a **lightweight**, **modular**, and **high-performance** chatbot REST API framework written in Rust.  
It is designed for simplicity, flexibility, and speed making it easy to integrate powerful AI model APIs like **Google Gemini**.

---

## 👀 Deprecated DOCS
Updated the docs soon after crate released

## 🚀 Features

- ⚡ Built on top of [`axum`](https://docs.rs/axum/latest/axum/) and [`reqwest`](https://docs.rs/reqwest/latest/reqwest/)
- 🧩 ~~Modular structure (supports multiple AI backends)~~ (Future plan)
- 🔒 Safe API key management
- 🧠 Simple interface for generating AI responses
- 🔧 Ready to extend into both **async backend** and **blocking CLI** environments

---

## General Usage

```rust
 fn main(){
    // Environment variable
    dotenv().ok();

    // Start client
    let gemini = GeminiClient::new();
    gemini.initiate(env::var("GEMINI_API_KEY").unwrap());

    // Generate response
    let result = gemini.generate_without_async("Hi, how are you?".to_string());
    println!("{}", result.unwrap());
}
```

## REST Api Usage

```rust
#[tokio::main]
async fn main() {
    // Environment variable
    dotenv().ok();

    // Start client
    let gemini = GeminiClient::new();
    gemini.initiate(env::var("GEMINI_API_KEY").unwrap());

    let app = Router::new()
        .route("/generate", post(eyai_wrapper))
        .with_state(gemini.clone());

    let listener = tokio::net::TcpListener::bind("localhost:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

