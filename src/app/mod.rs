use std::sync::Arc;

use axum::{routing::post, Router};
use bcrypt::hash;

use crate::api;
use crate:: infra::user_repo_memory::MemoryUserRepo;
use crate::service::auth_service::AuthService;

mod config;
pub use config::AppConfig;

#[derive(Clone)]
pub struct AppState{
    pub auth_service: AuthService<MemoryUserRepo>,
}

pub fn build_app() -> Router {
    let cfg = AppConfig::from_env();

    let seed_hash = hash("pass1234", 12).expect("bcrypt hash faild");
    let repo = Arc::new(MemoryUserRepo::new_with_seed_user("test@example.com", &seed_hash));

    let auth_service = AuthService::new(repo, cfg.jwt_secret);

    let state = AppState { auth_service };

    Router::new()
        .route("/auth/login", post(api::auth::login))
        .with_state(state)
}