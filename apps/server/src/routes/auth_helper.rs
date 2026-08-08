use axum::http::{HeaderMap, StatusCode};

use crate::app::AppState;

pub async fn authenticated_user_id(
    state: &AppState,
    headers: &HeaderMap,
) -> Result<String, StatusCode> {
    let auth = headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let token = auth
        .strip_prefix("Bearer ")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    state
        .auth
        .authenticate(token)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)
}
