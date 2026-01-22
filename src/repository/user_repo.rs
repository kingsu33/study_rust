use async_trait::async_trait;
use crate::domain::{AppError, User};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError>;
}