use crate::into_response::AppError;
use crate::models::{CreateDto, Todo, UpdateTodo};
use crate::AppState;
use axum::extract::{Path, State};
use axum::Json;

pub async fn root() -> &'static str {
    "Hello, world!"
}

pub async fn get_todos(
    State(state): State<AppState>,
) -> Result<Json<Vec<Todo>>, AppError> {

    let todos = sqlx::query_as::<_, Todo>(
        "SELECT id, title, completed FROM todos"
    )
        .fetch_all(&state.db)
        .await?;

    Ok(Json(todos))
}
pub async fn create_todos(
    State(state): State<AppState>,
    Json(payload): Json<CreateDto>,
) -> Result<Json<Todo>, AppError> {
    tracing::info!("Creating a new todo: {}", payload.title);

    let todo = sqlx::query_as::<_, Todo>(
        "INSERT INTO todos (title, completed)
         VALUES ($1, false)
         RETURNING id, title, completed"
    )
        .bind(payload.title)
        .fetch_one(&state.db)
        .await?;

    Ok(Json(todo))
}
pub async fn get_todo(
    Path(id): Path<u32>,
    State(state): State<AppState>,
) -> Result<Json<Todo>, AppError>{
    let todos = sqlx::query_as::<_, Todo>(
        "SELECT * FROM todos WHERE id = $1"
    )
        .fetch_one(&state.db)
        .await?;

    Ok(Json(todos))
}

pub async fn update_todos(
    Path(id): Path<i32>,
    State(state): State<AppState>,
    Json(payload): Json<UpdateTodo>,
) -> Result<Json<Todo>, AppError> {
    tracing::info!("Updating todo with id: {}", id);

    let todo = sqlx::query_as::<_, Todo>(
        r#"
        UPDATE todos
        SET
            title = COALESCE($1, title),
            completed = COALESCE($2, completed)
        WHERE id = $3
        RETURNING id, title, completed
        "#
    )
        .bind(payload.title)
        .bind(payload.completed)
        .bind(id)
        .fetch_optional(&state.db)
        .await?;

    match todo {
        Some(todo) => Ok(Json(todo)),
        None => Err(AppError::NotFound),
    }
}

pub async fn delete_todos(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<(), AppError> {
    tracing::info!("Deleting todo with id: {}", id);

    sqlx::query("DELETE FROM todos WHERE id = $1")
        .bind(id)
        .execute(&state.db)
        .await?;

    Ok(())
}