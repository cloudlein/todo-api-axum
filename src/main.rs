mod models;
mod handlers;
mod routes;
mod into_response;
mod config;

use crate::handlers::{create_todos, delete_todos, get_todo, get_todos, root, update_todos};
use crate::routes::todo::todo_routes;
use axum::routing::get;
use axum::Router;
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;
use std::net::SocketAddr;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

#[tokio::main]
async fn main() {

    dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let config = crate::config::Config::init();

    let db_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL is not set in .env file");

    let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect(&config.database_url)
    .await
    .expect("Unable to connect to postgresql");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Unable to migrate the database");

    let state = AppState { db: pool };

    let app = Router::new()
        .route("/", get(root))
        .nest("/api/v1/",todo_routes())
        .with_state(state);


    let addr = format!("{}:{}", config.host, config.port);
    tracing::info!("Server running on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

