use thiserror::Error;

/// 领域层仓储错误 —— 端口 trait 的统一错误类型。
///
/// `infra` 层负责将 `sqlx::Error` 映射为本类型（`impl From<sqlx::Error>`），
/// 领域层不感知任何基础设施 crate。
#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("database error: {0}")]
    Database(String),

    #[error("not found: {0}")]
    NotFound(String),

    #[error("constraint violation: {0}")]
    Constraint(String),
}

#[cfg(test)]
mod tests {
    use super::RepositoryError;

    #[test]
    fn database_error_display_contains_cause() {
        let err = RepositoryError::Database("connection refused".to_string());
        assert_eq!(err.to_string(), "database error: connection refused");
    }

    #[test]
    fn not_found_error_display_contains_resource() {
        let err = RepositoryError::NotFound("server 123".to_string());
        assert_eq!(err.to_string(), "not found: server 123");
    }

    #[test]
    fn constraint_error_display_contains_reason() {
        let err = RepositoryError::Constraint("duplicate key".to_string());
        assert_eq!(err.to_string(), "constraint violation: duplicate key");
    }

    #[test]
    fn variants_are_distinct() {
        let db = RepositoryError::Database("x".into());
        let nf = RepositoryError::NotFound("x".into());
        let con = RepositoryError::Constraint("x".into());
        assert!(!matches!(db, RepositoryError::NotFound(_)));
        assert!(!matches!(nf, RepositoryError::Constraint(_)));
        assert!(!matches!(con, RepositoryError::Database(_)));
    }
}
