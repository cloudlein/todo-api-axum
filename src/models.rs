use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;
use axum_valid::Valid;


#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub completed: bool,
}


#[derive(Debug, Deserialize, Validate)]
pub struct CreateDto {
    #[validate(length(min = 1, max = 255, message = "Title cannot be empty"))]
    pub title: String,
}

#[derive(Deserialize)]
pub struct UpdateTodo {
    pub title: Option<String>,
    pub completed: Option<bool>,
}

#[derive(Serialize)]
pub struct PaginateResponse<T> {
    pub(crate) data: Vec<T>,
    pub(crate) page: u32,
    pub(crate) limit: u32,
    pub total: i64,
}

#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
}