use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub completed: bool,
}


#[derive(Deserialize)]
pub struct CreateDto {
    pub title: String,
}

#[derive(Deserialize)]
pub struct UpdateTodo {
    pub title: Option<String>,
    pub completed: Option<bool>,
}