use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use crate::models::{CreateDto, Todo, UpdateTodo};
use crate::SharedTodos;


pub async fn root() -> & 'static str {
    "Hello, world!"
}

pub async fn get_todos(
    State(todos): State<SharedTodos>,
) -> Json<Vec<Todo>> {
    let todos = todos.lock().unwrap();
    Json(todos.clone())
}

pub async fn create_todos(
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

pub async fn get_todo(
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

pub async fn update_todos(
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

pub async fn delete_todos(
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


