use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use axum::extract::State;
use axum::{Json, Router};
use axum::routing::{get, post};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
struct Todo {
    id: u32,
    title: String,
    completed: bool,
}

type SharedTodos = Arc<Mutex<Vec<Todo>>>;

#[tokio::main]
async fn main() {

    let todos: SharedTodos = Arc::new(Mutex::new(vec![]));

    let app = Router::new()
        .route("/", get(root))
        .route("/todos", get(get_todos))
        .route("/todos", post(create_todos))
        .with_state(todos);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);

    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app,
    )
        .await
        .unwrap();
}

async fn root() -> & 'static str {
    "Hello, world!"
}

async fn get_todos(
    State(todos): State<SharedTodos>,
) -> Json<Vec<Todo>> {
    let todos = todos.lock().unwrap();
    Json(todos.clone())
}

async fn create_todos(
    State(todos): State<SharedTodos>,
    Json(payload): Json<Todo>
) -> Json<Todo> {
    let mut todos = todos.lock().unwrap();
    todos.push(payload.clone());
    Json(payload)
}