# 🦀 Rust Axum Greet Server

A lightweight HTTP server written in **Rust** using the [Axum](https://docs.rs/axum) web framework.  
It accepts a simple JSON POST request containing a `name` and responds with a greeting message.

---

## 🚀 Features

- Accepts a JSON `POST` request to `/greet`
- Returns a greeting message in JSON format
- Runs on `tokio` async runtime
- Clean, minimal, and beginner-friendly Rust project

---
---

## 🧠 Planned Features / TODO

- [ ] Store name and age in memory using `HashMap`
- [ ] Add a `GET /people` route to list stored users
- [ ] Save to JSON file for persistence
- [ ] Add input validation (e.g., name not empty, age > 0)
- [ ] Serve static HTML pages or React frontend
- [ ] Deploy to cloud (Render, Railway, etc.)

## 📈 Learning Goals

This project is part of my journey to learn:

- Backend development in Rust
- Async programming with Tokio
- Building and documenting real-world APIs

### Test with 
```json
{
curl -X POST http://localhost:8080/greet \
  -H "Content-Type: application/json" \
  -d '{"name": "Sanju"}'
}



#### ✅ Request Body

```json
{
  "name": "xyz"
}
