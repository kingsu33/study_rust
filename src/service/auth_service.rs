use std::sync::Arc;

use bcrypt::verify;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

use crate::domain::AppError;
use crate::repository::UserRepository;

#[derive(Clone)]
pub struct AuthService<R: UserRepository> {
    repo: Arc<R>,
    jwt_secret: String,
}

impl<R: UserRepository> AuthService<R> {
    pub fn new(repo: Arc<R>, jwt_secret: String) -> Self {
        Self { repo, jwt_secret }
    }

    pub async fn login(&self, email: &str, password: &str) -> Result<String, AppError> {
        if email.trim().is_empty() || password.is_empty() {
            return Err(AppError::InvalidInput("email/password is required".into()));
        }

        let user = self
            .repo
            .find_by_email(email)
            .await?
            .ok_or(AppError::Unauthorized)?;

        let ok = verify(password, &user.password).map_err(|_| AppError::Internal)?;
        if !ok {
            return Err(AppError::Unauthorized);
        }

        let now = Utc::now();
        let exp = now + Duration::minutes(60);

        let claims = Claims {
            sub: user.id.to_string(),
            email: user.email,
            iat: now.timestamp(),
            exp: exp.timestamp(),
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )
        .map_err(|_| AppError::Internal)?;

        Ok(token)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    email: String,
    iat: i64,
    exp: i64,
}