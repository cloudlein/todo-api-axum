use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tower::util::Optional;

#[derive(Serialize, Deserialize, Clone)]
struct Todo {
    id: u32,
    title: String,
    completed: bool,
}
#[derive( Deserialize)]
struct CreateDto {
    title: String
}

#[derive(Deserialize)]
struct UpdateTodo {
    title: Option<String>,
    completed: Option<bool>,
}

type SharedTodos = Arc<Mutex<Vec<Todo>>>;

#[tokio::main]
async fn main() {

    let todos: SharedTodos = Arc::new(Mutex::new(vec![]));

    let app = Router::new()
        .route("/", get(root))
        .route("/todos", get(get_todos).post(create_todos))
        .route("/todos/{id}", get(get_todo).put(update_todos).delete(delete_todos))
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
    Json(payload): Json<CreateDto>
) -> Json<Todo> {
    let mut todos = todos.lock().unwrap();
    let id = (todos.len() as u32) + 1;
    let todo = Todo {
        id,
        title: payload.title,
        completed: false,
    };

    todos.push(todo.clone());
    Json(todo)
}

async fn get_todo(
    Path(id): Path<u32>,
    State(todos): State<SharedTodos>,
) -> Result<Json<Todo>, StatusCode>{
    let todos = todos.lock().unwrap();
    todos.iter()
        .find(|t| t.id == id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

async fn update_todos(
    Path(id): Path<u32>,
    State(todos): State<SharedTodos>,
    Json(payload): Json<UpdateTodo>,
) -> Result<Json<Todo>, StatusCode> {
    let mut todos = todos.lock().unwrap();

    if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
        if let Some(title) = payload.title {
            todo.title = title;
        }
        if let Some(completed) = payload.completed {
            todo.completed = completed;
        }

        return Ok(Json(todo.clone()));
    }

    Err(StatusCode::NOT_FOUND)
}

async fn delete_todos(
    Path(id): Path<u32>,
    State(todos): State<SharedTodos>,
) -> Result<StatusCode, StatusCode> {
    let mut todos = todos.lock().unwrap();
    let len_before = todos.len();
    todos.retain(|t| t.id != id);

    if todos.len() == len_before {
        Err(StatusCode::NOT_FOUND)
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}