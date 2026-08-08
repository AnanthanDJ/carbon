use axum::{
    Json,
    extract::State,
    http::{HeaderMap, StatusCode},
};
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
    pub token: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub success: bool,
    pub token: String,
}

#[derive(Serialize)]
pub struct CurrentUserResponse {
    pub id: String,
    pub username: String,
}

pub async fn register(
    State(state): State<AppState>,
    Json(request): Json<RegisterRequest>,
) -> Result<Json<RegisterResponse>, StatusCode> {
    let token = state
        .auth
        .register(&request.username, &request.password)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    Ok(Json(RegisterResponse {
        success: true,
        token: token.value,
    }))
}

pub async fn login(
    State(state): State<AppState>,
    Json(request): Json<RegisterRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    let token = state
        .auth
        .login(&request.username, &request.password)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    Ok(Json(LoginResponse {
        success: true,
        token: token.value,
    }))
}

pub async fn me(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<CurrentUserResponse>, StatusCode> {
    let auth = headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let token = auth
        .strip_prefix("Bearer ")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let user_id = state
        .auth
        .authenticate(token)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let user = state
        .auth
        .current_user(&user_id)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    Ok(Json(CurrentUserResponse {
        id: user.id,
        username: user.username,
    }))
}
