mod application;
mod config;
mod domain;
mod errors;
mod handlers;
mod infrastructure;
mod models;
mod routes;

use crate::application::todo_service::TodoService;
use crate::handlers::root;
use crate::infrastructure::sqlx_todo_repository::SqlxTodoRepository;
use crate::routes::todo_routes;
use axum::Router;
use axum::routing::get;
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub todo_service: Arc<TodoService>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    //Load Application Configuration
    let config = crate::config::Config::init();

    // Initialize Database Connection Pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .expect("Unable to connect to postgresql");

    // 5. Run Database Migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Unable to migrate the database");

    // Initialize Repository and Service (Dependency Injection)
    let repository = Arc::new(SqlxTodoRepository::new(pool));
    let service = Arc::new(TodoService::new(repository));

    // Setup Application State
    let state = AppState {
        todo_service: service,
    };

    // Build Axum Router
    let app = Router::new()
        .route("/", get(root))
        .nest("/api/v1/", todo_routes())
        .with_state(state);

    // 9. Start Server
    let addr = format!("{}:{}", config.host, config.port);
    tracing::info!("Server running on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
