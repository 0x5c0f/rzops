use thiserror::Error;

/// Base application error type.
/// Handlers should return `Result<T, AppError>` for consistent error responses.
/// The `IntoResponse` impl is in the `api` crate to keep common dependency-free.
#[derive(Debug, Error)]
pub enum AppError {
    #[error("resource not found: {0}")]
    NotFound(String),

    #[error("validation error: {0}")]
    Validation(String),

    #[error("unauthorized")]
    Unauthorized,

    #[error("forbidden")]
    Forbidden,

    #[error("internal error: {0}")]
    Internal(String),
}
