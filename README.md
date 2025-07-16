# 🦀 Rust Greet Server (Axum + In-Memory Store)

A simple and beginner-friendly HTTP server written in **Rust** using the [Axum](https://docs.rs/axum) framework. It accepts a JSON `POST` request with a user's name and age, stores the info in memory, and returns a greeting response.

---

## 🚀 Features

- 📥 Accepts POST request with `name` and `age`
- 💾 Stores data in an in-memory `HashMap`
- 🧠 Built using `async` Rust (`tokio` runtime)
- ⚡ Uses `Arc<Mutex<...>>` for safe concurrent state
- 🔌 Easy to extend with GET routes, file storage, or a database

---

## 🛠 Tech Stack

- [Rust](https://www.rust-lang.org/)
- [Axum](https://docs.rs/axum)
- [Tokio](https://tokio.rs/)
- [Serde](https://serde.rs/) for JSON (de)serialization

---

## 📬 API Endpoints

### `POST /greet`

Accepts a JSON payload with `name` and `age`.

#### ✅ Request Body
```json
{
  "name": "Sanju",
  "age": 21
}
