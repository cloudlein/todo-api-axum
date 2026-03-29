use axum::{Router, routing::get};

use crate::{
    AppState,
    handlers::{create_todos, delete_todos, get_todo, get_todos, update_todos},
};

pub fn todo_routes() -> Router<AppState> {
    Router::new()
        .route("/todos", get(get_todos).post(create_todos))
        .route(
            "/todos/{id}",
            get(get_todo).put(update_todos).delete(delete_todos),
        )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::todo_service::TodoService;
    use crate::infrastructure::sqlx_todo_repository::SqlxTodoRepository;
    use axum::{
        body::Body,
        http::{Request, StatusCode, header},
    };
    use sqlx::PgPool;
    use std::sync::{Arc, OnceLock};
    use tower::ServiceExt;

    // Use OnceLock to ensure the database pool is only initialized ONCE for all tests.
    static TEST_POOL: OnceLock<PgPool> = OnceLock::new();

    async fn get_test_pool() -> PgPool {
        if let Some(pool) = TEST_POOL.get() {
            return pool.clone();
        }

        dotenvy::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let pool = PgPool::connect(&database_url).await.unwrap();
        TEST_POOL.set(pool.clone()).ok();
        pool
    }

    async fn setup_test_context() -> (Router, PgPool) {
        let pool = get_test_pool().await;

        // Cleanup before each test
        sqlx::query("TRUNCATE TABLE todos RESTART IDENTITY CASCADE;")
            .execute(&pool)
            .await
            .unwrap();

        let repository = Arc::new(SqlxTodoRepository::new(pool.clone()));
        let service = Arc::new(TodoService::new(repository));
        let state = AppState {
            todo_service: service,
        };

        let app = Router::new()
            .nest("/api/v1", todo_routes())
            .with_state(state);

        (app, pool)
    }

    #[tokio::test]
    async fn test_create_todo_success() {
        let (app, _) = setup_test_context().await;

        let json_payload = serde_json::json!({
            "title": "Learn Clean Architecture"
        });

        let req = Request::builder()
            .method("POST")
            .uri("/api/v1/todos")
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(json_payload.to_string()))
            .unwrap();

        let res = app.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_create_todo_validation_error() {
        let (app, _) = setup_test_context().await;

        let json_payload = serde_json::json!({
            "title": ""
        });

        let req = Request::builder()
            .method("POST")
            .uri("/api/v1/todos")
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(json_payload.to_string()))
            .unwrap();

        let res = app.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_get_all_todos_pagination() {
        let (app, pool) = setup_test_context().await;

        sqlx::query(
            "INSERT INTO todos (title, completed) VALUES ('Todo 1', false), ('Todo 2', true)",
        )
        .execute(&pool)
        .await
        .unwrap();

        let req = Request::builder()
            .method("GET")
            .uri("/api/v1/todos?limit=1&page=1")
            .body(Body::empty())
            .unwrap();

        let res = app.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);

        let body_bytes = axum::body::to_bytes(res.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();

        assert_eq!(json["data"].as_array().unwrap().len(), 1);
        assert_eq!(json["total"], 2);
    }

    #[tokio::test]
    async fn test_get_todo_by_id() {
        let (app, pool) = setup_test_context().await;

        let id: i32 = sqlx::query_scalar(
            "INSERT INTO todos (title, completed) VALUES ('Test Get', false) RETURNING id",
        )
        .fetch_one(&pool)
        .await
        .unwrap();

        let req = Request::builder()
            .method("GET")
            .uri(format!("/api/v1/todos/{}", id))
            .body(Body::empty())
            .unwrap();

        let res = app.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_get_todo_not_found_returns_404() {
        let (app, _) = setup_test_context().await;

        let req = Request::builder()
            .method("GET")
            .uri("/api/v1/todos/999999")
            .body(Body::empty())
            .unwrap();

        let res = app.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_update_todo_success() {
        let (app, pool) = setup_test_context().await;

        let id: i32 = sqlx::query_scalar(
            "INSERT INTO todos (title, completed) VALUES ('Old', false) RETURNING id",
        )
        .fetch_one(&pool)
        .await
        .unwrap();

        let json_payload = serde_json::json!({
            "title": "New Updated Title",
            "completed": true
        });

        let req = Request::builder()
            .method("PUT")
            .uri(format!("/api/v1/todos/{}", id))
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(json_payload.to_string()))
            .unwrap();

        let res = app.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_delete_todo_success() {
        let (app, pool) = setup_test_context().await;

        let id: i32 = sqlx::query_scalar(
            "INSERT INTO todos (title, completed) VALUES ('To Delete', false) RETURNING id",
        )
        .fetch_one(&pool)
        .await
        .unwrap();

        let req = Request::builder()
            .method("DELETE")
            .uri(format!("/api/v1/todos/{}", id))
            .body(Body::empty())
            .unwrap();

        let res = app.clone().oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);

        // Prove it actually deleted
        let req_check = Request::builder()
            .method("GET")
            .uri(format!("/api/v1/todos/{}", id))
            .body(Body::empty())
            .unwrap();

        let res_check = app.oneshot(req_check).await.unwrap();
        assert_eq!(res_check.status(), StatusCode::NOT_FOUND);
    }
}
