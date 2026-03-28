use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use crate::into_response::AppError;
use crate::models::{CreateDto, Todo, UpdateTodo};
use crate::SharedTodos;


pub async fn root() -> &'static str {
    "Hello, world!"
}

pub async fn get_todos(
    State(todos): State<SharedTodos>,
) -> Result<impl IntoResponse, AppError> {
    let todos = todos
        .lock().map_err(|_| AppError::InternalServerError)?;
    Ok(Json(todos.clone()))
}

pub async fn create_todos(
    State(todos): State<SharedTodos>,
    Json(payload): Json<CreateDto>
) -> Result<Json<Todo>, AppError>  {
    let mut todos = todos
        .lock()
        .map_err(|_| AppError::InternalServerError)?;

    let id = (todos.len() as u32) + 1;
    let todo = Todo {
        id,
        title: payload.title,
        completed: false,
    };

    todos.push(todo.clone());
   Ok(Json(todo))
}

pub async fn get_todo(
    Path(id): Path<u32>,
    State(todos): State<SharedTodos>,
) -> Result<Json<Todo>, AppError>{
    let todos = todos
        .lock().map_err(|_| AppError::InternalServerError)?;
    todos.iter()
        .find(|t| t.id == id)
        .cloned()
        .map(Json)
        .ok_or(AppError::NotFound)
}

pub async fn update_todos(
    Path(id): Path<u32>,
    State(todos): State<SharedTodos>,
    Json(payload): Json<UpdateTodo>,
) -> Result<Json<Todo>, AppError> {
    let mut  todos = todos
        .lock().map_err(|_| AppError::InternalServerError)?;

    if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
        if let Some(title) = payload.title {
            todo.title = title;
        }
        if let Some(completed) = payload.completed {
            todo.completed = completed;
        }
        return Ok(Json(todo.clone()));
    }

    Err(AppError::NotFound)
}

pub async fn delete_todos(
    Path(id): Path<u32>,
    State(todos): State<SharedTodos>,
) -> Result<impl IntoResponse, AppError> {
    let mut todos = todos
        .lock().map_err(|_| AppError::InternalServerError)?;
    let len_before = todos.len();
    todos.retain(|t| t.id != id);

    if todos.len() == len_before {
        Err(AppError::NotFound)
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}


