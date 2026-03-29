use crate::domain::todo::{Todo, TodoRepository};
use crate::errors::AppError;
use crate::models::{CreateDto, PaginateResponse, PaginationQuery, UpdateTodo};
use std::sync::Arc;

pub struct TodoService {
    // Dynamic dispatch of the repository or static dispatch can be used here.
    // For simplicity and standard DI, we use Arc<dyn TodoRepository>.
    repository: Arc<dyn TodoRepository>,
}

impl TodoService {
    pub fn new(repository: Arc<dyn TodoRepository>) -> Self {
        Self { repository }
    }

    pub async fn get_todos(
        &self,
        params: &PaginationQuery,
    ) -> Result<PaginateResponse<Todo>, AppError> {
        self.repository.find_all(params).await
    }

    pub async fn get_todo(&self, id: i32) -> Result<Todo, AppError> {
        let todo = self.repository.find_by_id(id).await?;
        todo.ok_or(AppError::NotFound)
    }

    pub async fn create_todo(&self, payload: &CreateDto) -> Result<Todo, AppError> {
        self.repository.create(payload).await
    }

    pub async fn update_todo(&self, id: i32, payload: &UpdateTodo) -> Result<Todo, AppError> {
        let todo = self.repository.update(id, payload).await?;
        todo.ok_or(AppError::NotFound)
    }

    pub async fn delete_todo(&self, id: i32) -> Result<(), AppError> {
        let success = self.repository.delete(id).await?;
        if !success {
            return Err(AppError::NotFound);
        }
        Ok(())
    }
}
