use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::api::dto::{LoginRequest, LoginResponse};
use crate::app::AppState;
use crate::domain::AppError;

pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let token = state
        .auth_service
        .login(&req.email, &req.password)
        .await
        .map_err(map_app_error)?;

    Ok(Json(LoginResponse {
        access_token: token,
        token_type: "Bearer",
    }))
}

fn map_app_error(err: AppError) -> (StatusCode, String) {
    match err {
        AppError::InvalidInput(msg) => (StatusCode::BAD_REQUEST, msg),
        AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "unauthorized".into()),
        AppError::Internal => (StatusCode::INTERNAL_SERVER_ERROR, "internal".into()),
    }
}