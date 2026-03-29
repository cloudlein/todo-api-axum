pub mod todo_handler;

pub use todo_handler::*;

// Root handler for the basic health check
pub async fn root() -> &'static str {
    "Hello, world!"
}
