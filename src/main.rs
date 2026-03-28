mod models;
mod handlers;
mod routes;
mod into_response;

use crate::handlers::{create_todos, delete_todos, get_todo, get_todos, root, update_todos};
use crate::models::Todo;
use axum::routing::{delete, get, post, put};
use axum::Router;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use crate::routes::todo::todo_routes;

type SharedTodos = Arc<Mutex<Vec<Todo>>>;

#[tokio::main]
async fn main() {

    let todos: SharedTodos = Arc::new(Mutex::new(vec![]));

    let app = Router::new()
        .route("/", get(root))
        .merge(todo_routes())
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

