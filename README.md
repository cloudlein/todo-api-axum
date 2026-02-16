# Todo API with Axum

A minimal REST API built with Axum to manage simple in-memory todos. This project is my first exploration into building web services in Rust. While I am new to Rust web development, I am not entirely a beginner in Rust—I have prior experience with terminal UI development using Ratatui.

## Overview
- Framework: Axum (async web framework)
- Runtime: Tokio
- Data model: In-memory `Vec<Todo>` wrapped in `Arc<Mutex<...>>`
- Serialization: Serde / serde_json
- Bind address: `127.0.0.1:3000`

## Endpoints
- `GET /`  
  Returns a plain greeting.
- `GET /todos`  
  Returns all todos as JSON.
- `POST /todos`  
  Creates a new todo from the provided JSON payload and returns it.

### Todo Schema
```json
{
  "id": 1,
  "title": "Learn Axum",
  "completed": false
}
```

## Quick Start
Prerequisites:
- Rust (stable) with Cargo installed

Run the server:
```bash
cargo run
```

You should see:
```
listening on 127.0.0.1:3000
```

## Usage Examples
Get greeting:
```bash
curl http://127.0.0.1:3000/
```

List todos:
```bash
curl http://127.0.0.1:3000/todos
```

Create a todo:
```bash
curl -X POST http://127.0.0.1:3000/todos \
  -H "Content-Type: application/json" \
  -d '{"id":1,"title":"Learn Axum","completed":false}'
```

## Architecture Notes
- Router defines three routes: `/`, `/todos` (GET), `/todos` (POST).
- Shared state uses `Arc<Mutex<Vec<Todo>>>` for simplicity. This is fine for learning and small demos but not recommended for production workloads.
- The server is single binary and has no persistence layer—todos are lost on restart.

## Future Improvements
- Replace `Mutex<Vec<Todo>>` with a proper data layer (e.g., SQLx + Postgres or SQLite).
- Add update/delete endpoints for full CRUD.
- Introduce validation and error handling.
- Write unit/integration tests and CI workflow.

## Background
This repository exists to learn Axum and Rust web development concepts. Although I am new to building web services with Rust, I have experience with Rust via a Ratatui project. The goal is to iteratively improve this API while keeping the code simple and readable.

