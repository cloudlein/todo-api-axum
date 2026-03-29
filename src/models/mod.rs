use serde::{Deserialize, Serialize};
use sqlx::{Postgres, QueryBuilder};
use validator::Validate;

// --- Models (Data Transfer Objects) ---

// Data Transfer Object (DTO) for creating a new Todo.
#[derive(Debug, Deserialize, Validate)]
pub struct CreateDto {
    #[validate(length(min = 1, max = 255, message = "Title cannot be empty"))]
    pub title: String,
}

// DTO for updating an existing Todo.
#[derive(Deserialize)]
pub struct UpdateTodo {
    pub title: Option<String>,
    pub completed: Option<bool>,
}

// A standard structure for paginated responses.
#[derive(Serialize)]
pub struct PaginateResponse<T> {
    pub data: Vec<T>,
    pub page: u32,
    pub limit: u32,
    pub total: i64,
}

// Represents the query parameters for the GET /todos endpoint.
#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub completed: Option<bool>,
    pub search: Option<String>,
}

impl PaginationQuery {
    pub fn apply_filters<'args>(&'args self, builder: &mut QueryBuilder<'args, Postgres>) {
        let mut has_condition = false;

        if self.completed.is_some() || self.search.is_some() {
            builder.push(" WHERE ");
        }

        if let Some(completed) = &self.completed {
            builder.push("completed = ");
            builder.push_bind(*completed);
            has_condition = true;
        }

        if let Some(search) = &self.search {
            if has_condition {
                builder.push(" AND ");
            }
            builder.push("title ILIKE ");
            let search_term = format!("%{}%", search);
            builder.push_bind(search_term);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::{Postgres, QueryBuilder};
    use validator::Validate;

    #[test]
    fn test_create_dto_valid() {
        let dto = CreateDto {
            title: "Some title".to_string(),
        };
        let is_valid = dto.validate().is_ok();
        assert!(is_valid, "DTO should be completely valid!");
    }

    #[test]
    fn test_create_dto_invalid_empty() {
        let dto = CreateDto {
            title: "".to_string(),
        };
        let validation_result = dto.validate();
        assert!(
            validation_result.is_err(),
            "DTO should fail because the title is completely empty!"
        );

        let errors = validation_result.unwrap_err();
        let field_errors = errors.field_errors();
        assert!(
            field_errors.contains_key("title"),
            "The error must be triggered by 'title' field"
        );
    }

    #[test]
    fn test_create_dto_max_length_exceeded() {
        let long_title = "a".repeat(300);
        let dto = CreateDto { title: long_title };
        assert!(
            dto.validate().is_err(),
            "DTO should fail if title is > 255 bytes"
        );
    }

    #[test]
    fn test_pagination_query_no_filters() {
        let query = PaginationQuery {
            page: None,
            limit: None,
            completed: None,
            search: None,
        };
        let mut builder = QueryBuilder::<Postgres>::new("SELECT * FROM todos");
        query.apply_filters(&mut builder);
        assert_eq!(builder.sql(), "SELECT * FROM todos");
    }

    #[test]
    fn test_pagination_query_only_completed() {
        let query = PaginationQuery {
            page: None,
            limit: None,
            completed: Some(true),
            search: None,
        };
        let mut builder = QueryBuilder::<Postgres>::new("SELECT * FROM todos");
        query.apply_filters(&mut builder);
        assert_eq!(builder.sql(), "SELECT * FROM todos WHERE completed = $1");
    }

    #[test]
    fn test_pagination_query_only_search() {
        let query = PaginationQuery {
            page: None,
            limit: None,
            completed: None,
            search: Some("rust".to_string()),
        };
        let mut builder = QueryBuilder::<Postgres>::new("SELECT * FROM todos");
        query.apply_filters(&mut builder);
        assert_eq!(builder.sql(), "SELECT * FROM todos WHERE title ILIKE $1");
    }

    #[test]
    fn test_pagination_query_both_filters() {
        let query = PaginationQuery {
            page: None,
            limit: None,
            completed: Some(false),
            search: Some("axum".to_string()),
        };
        let mut builder = QueryBuilder::<Postgres>::new("SELECT * FROM todos");
        query.apply_filters(&mut builder);
        assert_eq!(
            builder.sql(),
            "SELECT * FROM todos WHERE completed = $1 AND title ILIKE $2"
        );
    }
}
