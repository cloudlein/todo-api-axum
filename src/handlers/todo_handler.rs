use crate::AppState;
use crate::domain::todo::Todo;
use crate::errors::AppError;
use crate::models::{CreateDto, PaginateResponse, PaginationQuery, UpdateTodo};
use axum::Json;
use axum::extract::{Path, Query, State};
use axum_valid::Valid;

// GET /todos
pub async fn get_todos(
    Query(params): Query<PaginationQuery>,
    State(state): State<AppState>,
) -> Result<Json<PaginateResponse<Todo>>, AppError> {
    let result = state.todo_service.get_todos(&params).await?;
    Ok(Json(result))
}

// POST /todos
pub async fn create_todos(
    State(state): State<AppState>,
    Valid(Json(payload)): Valid<Json<CreateDto>>,
) -> Result<Json<Todo>, AppError> {
    let result = state.todo_service.create_todo(&payload).await?;
    Ok(Json(result))
}

// GET /todos/{id}
pub async fn get_todo(
    Path(id): Path<i32>,
    State(state): State<AppState>,
) -> Result<Json<Todo>, AppError> {
    let result = state.todo_service.get_todo(id).await?;
    Ok(Json(result))
}

// PUT /todos/{id}
pub async fn update_todos(
    Path(id): Path<i32>,
    State(state): State<AppState>,
    Json(payload): Json<UpdateTodo>,
) -> Result<Json<Todo>, AppError> {
    let result = state.todo_service.update_todo(id, &payload).await?;
    Ok(Json(result))
}

// DELETE /todos/{id}
pub async fn delete_todos(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<(), AppError> {
    state.todo_service.delete_todo(id).await?;
    Ok(())
}
