use axum::{
    routing::{get, post, put, delete},
    Router,
};
use std::sync::{Arc, Mutex};

use crate::models::{Todo, CreateDto, UpdateTodo};
use crate::{
    get_todos, create_todos, update_todos, delete_todos, get_todo
};

type SharedTodos = Arc<Mutex<Vec<Todo>>>;

pub fn todo_routes() -> Router<SharedTodos> {
    Router::new()
        .route("/todos", get(get_todos).post(create_todos))
        .route("/todos/:id", get(get_todo).put(update_todos).delete(delete_todos))
}