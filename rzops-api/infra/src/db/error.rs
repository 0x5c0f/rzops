use rzops_domain::errors::RepositoryError;

/// 将 `Result<T, sqlx::Error>` 映射为领域错误。
///
/// 孤儿规则禁止在 infra 中直接实现 `From<sqlx::Error> for RepositoryError`
/// （trait 与两个类型均非本 crate 所有），故以扩展方法落地映射。
pub trait IntoRepoResult<T> {
    fn repo(self) -> Result<T, RepositoryError>;
}

impl<T> IntoRepoResult<T> for Result<T, sqlx::Error> {
    fn repo(self) -> Result<T, RepositoryError> {
        self.map_err(|e| RepositoryError::Database(e.to_string()))
    }
}
