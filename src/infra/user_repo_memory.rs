use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::{AppError, User};
use crate::repository::UserRepository;

#[derive(Clone)]
pub struct MemoryUserRepo {
    users: Arc<RwLock<HashMap<String, User>>>,
}

impl MemoryUserRepo {
    pub fn new_with_seed_user(email: &str, password: &str) -> Self {
        let mut map = HashMap::new();
        map.insert(
            email.to_string(),
            User {
                id: Uuid::new_v4(),
                email: email.to_string(),
                password: password.to_string(),
            },
        );
        Self { users: Arc::new(RwLock::new(map))}
    }
}

#[async_trait]
impl UserRepository for MemoryUserRepo {
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        let guard = self.users.read().map_err(|_| AppError::Internal)?;
        Ok(guard.get(email).cloned())
    }
}