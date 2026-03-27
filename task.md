# Roadmap: From Beginner to Middle-Level Backend Developer (Rust & Axum)

This document contains a list of tasks designed to help you level up from a beginner to a middle-level developer in the Rust ecosystem (specifically backend development using the Axum framework).

Currently, we have a Todo API that runs in memory (`Arc<Mutex<Vec<Todo>>>`). The code structure works well for initial learning, but there is a lot of room for improvement towards a production-ready "middle" level.

Please check off the boxes below as you progress with your learning!

## Stage 1: Refactoring & Code Organization (Architecture)
As a middle-level developer, you must be able to organize the structure so the code is easy to read, expand, and maintain.
- [x] Create a new file/module for *Models/Entities* (move the `Todo`, `CreateDto`, and `UpdateTodo` structs to `src/models.rs`).
- [x] Create a new file/module for *Handlers/Controllers* (move handler functions like `get_todos`, `create_todos`, etc., to `src/handlers.rs`).
- [x] Create a new file/module for *Routes* (move router configuration to a file like `src/routes.rs`).
- [x] Clean up `main.rs` so it acts purely as an entry point (setting up connections, state, and running the server).

## Stage 2: Error Handling Best Practices
`.unwrap()` is very dangerous in production because it will cause the program to crash (panic). We need proper error handling.
- [ ] Learn the `IntoResponse` trait in Axum to handle *custom errors*.
- [ ] Create an `AppError` struct/enum to catch internal errors (database errors, item not found, etc.).
- [ ] (Optional) Use the `thiserror` crate to simplify *custom error* implementation.
- [ ] Replace all `unwrap()` calls with `?` (the try operator) in handlers, and return relevant HTTP responses like 404 (Not Found) or 500 (Internal Server Error).

## Stage 3: Database & Data Persistence
In-memory state will be lost when the server restarts. It's time to connect the application to a real database!
- [ ] Install a real database on your computer (e.g., PostgreSQL or SQLite).
- [ ] Add the `sqlx` (async database driver) and `dotenvy` crates to `Cargo.toml`.
- [ ] Extract the *connection string* into a `.env` file (e.g., `DATABASE_URL=postgres://user:pass@localhost/todos`).
- [ ] Create a database table migration schema for `todos`.
- [ ] Replace the use of `Arc<Mutex<..>>` with a database connection pool like `sqlx::PgPool` in the *application state*.
- [ ] Refactor all handler logic (GET, POST, PUT, DELETE) to execute *raw SQL queries* against the real database (`sqlx::query!`).

## Stage 4: Observability, Logging, & Configuration
For a "middle" or production level, debugging using `println!` is not effective. We need structured logging.
- [ ] Replace the `println!` macro with the standard log aggregation crates in the Rust community: `tracing` and `tracing-subscriber`.
- [ ] Add logs using `tracing::info!`, `tracing::warn!`, and `tracing::error!` when: a new request comes in, data is successfully added, and when an unexpected error occurs.
- [ ] Create a `Config` struct (application configuration) that loads important variables from the environment (PORT, HOST, DATABASE_URL) instead of hardcoding them (`127.0.0.1:3000`).

## Stage 5: Data Validation
Never assume the data sent by the Client from the *Request Body* is always safe and valid!
- [ ] Add the `validator` crate to validate DTO structs, such as `CreateDto` and `UpdateTodo`.
- [ ] Ensure the `title` input field cannot be *empty* or just contain *spaces*.
- [ ] Ensure `title` is limited in characters (e.g., maximum 255 characters) to minimize potential abuse.
- [ ] Return a 400 (Bad Request) response with detailed invalidity messages if the client input is not valid.

## Stage 6: Pagination & Filtering
As data grows, fetching the *entire* table contents will burden the database and be slow.
- [ ] Implement Pagination: Add *Query Parameters* like `?page=1&limit=10` to the `GET /todos` endpoint using the `axum::extract::Query` extractor.
- [ ] Implement Filtering: Provide filters in `GET /todos` to fetch only completed todos or to handle search *keywords* (e.g., `?status=completed&search=learn`).

## Stage 7: Automated Testing
A middle-level developer does not solely test their application manually. New features can accidentally break existing ones.
- [ ] Write **Unit Tests** to verify the functionality of small logic (validation, core algorithms).
- [ ] Write **Integration Tests** using the built-in axum modules (and calling `router.oneshot(request)`) to simulate full HTTP requests from the front (Request) directly to the back (Response) without having to open Postman.

## Stage 8: CI/CD & Deployment
Middle-level means understanding how to bring your program to the cloud ecosystem.
- [ ] Create a `Dockerfile` with a *multi-stage build* schema to compile the Rust application into a lightweight *binary image*.
- [ ] Build a `docker-compose.yml` configuration file to make it easier to run your Rust API with its local database simultaneously.
- [ ] Create a GitHub Actions workflow (e.g., `.github/workflows/rust.yml`) that includes a pipeline: *Linting* with `cargo clippy`, performance checking (*cargo check*), code formatting with `cargo fmt`, and running `cargo test` every time there is a branch/change pushed to the GitHub repository.

---

Keep up the great work! Just take it one step at a time. The Rust ecosystem and the strict compiler are actually very enjoyable and will guide your learning perfectly. 🦀🚀
