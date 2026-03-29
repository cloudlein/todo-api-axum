use crate::errors::AppError;
use crate::models::{CreateDto, PaginateResponse, PaginationQuery, UpdateTodo};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// --- Domain Entity ---

// The core business object. In a complex app, this might have methods for business logic.
#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub completed: bool,
}

// --- Repository Trait (Interface) ---

// This defines WHAT the database must be able to do, without saying HOW.
// We use `async_trait` because Rust doesn't natively support async traits yet.
#[async_trait]
pub trait TodoRepository: Send + Sync {
    async fn find_all(&self, params: &PaginationQuery) -> Result<PaginateResponse<Todo>, AppError>;
    async fn find_by_id(&self, id: i32) -> Result<Option<Todo>, AppError>;
    async fn create(&self, payload: &CreateDto) -> Result<Todo, AppError>;
    async fn update(&self, id: i32, payload: &UpdateTodo) -> Result<Option<Todo>, AppError>;
    async fn delete(&self, id: i32) -> Result<bool, AppError>;
}
