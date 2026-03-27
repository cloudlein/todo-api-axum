use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{delete, get, post, put};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::task::id;

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
        .route("/todos/:id", put(update_todos))
        .route("/todos/:id", delete(delete_todos))
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

async fn update_todos(
    Path(id): Path<u32>,
    State(todos): State<SharedTodos>,
    Json(payload): Json<Todo>
) -> Result<Json<Todo>, StatusCode> {
    let mut todos = todos.lock().unwrap();

    for todo in todos.iter_mut()  {
        if todo.id == id {
            todo.title = payload.title.clone();
            todo.completed = payload.completed;

            return Ok(Json(todo.clone()));
        }

    }

    Err(StatusCode::NOT_FOUND)
}

async fn delete_todos(
    Path(id): Path<u32>,
    State(todos): State<SharedTodos>,
) -> StatusCode {
    let mut todos = todos.lock().unwrap();

    let previous_len = todos.len();

    todos.retain(|todo| todo.id != id);

    if todos.len() < previous_len {
        StatusCode::NO_CONTENT
    }else {
        StatusCode::NOT_FOUND
    }
}