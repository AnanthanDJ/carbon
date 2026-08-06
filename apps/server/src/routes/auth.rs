use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};

use crate::app::AppState;

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct RegisterResponse {
    pub success: bool,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub success: bool,
}

pub async fn register(
    State(state): State<AppState>,
    Json(request): Json<RegisterRequest>,
) -> Result<Json<RegisterResponse>, StatusCode> {
    state
        .auth
        .register(&request.username, &request.password)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    Ok(Json(RegisterResponse { success: true }))
}

pub async fn login(
    State(state): State<AppState>,
    Json(request): Json<RegisterRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    state
        .auth
        .login(&request.username, &request.password)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    Ok(Json(LoginResponse { success: true }))
}
