use crate::domain::todo::{Todo, TodoRepository};
use crate::errors::AppError;
use crate::models::{CreateDto, PaginateResponse, PaginationQuery, UpdateTodo};
use async_trait::async_trait;
use sqlx::{PgPool, Postgres, QueryBuilder};

pub struct SqlxTodoRepository {
    pool: PgPool,
}

impl SqlxTodoRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TodoRepository for SqlxTodoRepository {
    async fn find_all(&self, params: &PaginationQuery) -> Result<PaginateResponse<Todo>, AppError> {
        let page = params.page.unwrap_or(1);
        let limit = params.limit.unwrap_or(10);
        let offset = (page - 1) * limit;

        // 1. Fetch data
        let mut query_builder =
            QueryBuilder::<Postgres>::new("SELECT id, title, completed FROM todos");
        params.apply_filters(&mut query_builder);
        query_builder.push(" ORDER BY id ASC LIMIT ");
        query_builder.push_bind(limit as i64);
        query_builder.push(" OFFSET ");
        query_builder.push_bind(offset as i64);

        let todos = query_builder
            .build_query_as::<Todo>()
            .fetch_all(&self.pool)
            .await?;

        // 2. Fetch total count
        let mut count_builder = QueryBuilder::<Postgres>::new("SELECT COUNT(*) FROM todos");
        params.apply_filters(&mut count_builder);
        let (total,): (i64,) = count_builder.build_query_as().fetch_one(&self.pool).await?;

        Ok(PaginateResponse {
            data: todos,
            page,
            limit,
            total,
        })
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<Todo>, AppError> {
        let todo =
            sqlx::query_as::<_, Todo>("SELECT id, title, completed FROM todos WHERE id = $1")
                .bind(id)
                .fetch_optional(&self.pool)
                .await?;
        Ok(todo)
    }

    async fn create(&self, payload: &CreateDto) -> Result<Todo, AppError> {
        let todo = sqlx::query_as::<_, Todo>(
            "INSERT INTO todos (title, completed) VALUES ($1, false) RETURNING id, title, completed"
        )
            .bind(&payload.title)
            .fetch_one(&self.pool)
            .await?;
        Ok(todo)
    }

    async fn update(&self, id: i32, payload: &UpdateTodo) -> Result<Option<Todo>, AppError> {
        let todo = sqlx::query_as::<_, Todo>(
            r#"
            UPDATE todos
            SET
                title = COALESCE($1, title),
                completed = COALESCE($2, completed)
            WHERE id = $3
            RETURNING id, title, completed
            "#,
        )
        .bind(&payload.title)
        .bind(payload.completed)
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(todo)
    }

    async fn delete(&self, id: i32) -> Result<bool, AppError> {
        let result = sqlx::query("DELETE FROM todos WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}
