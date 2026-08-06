use axum::{Json, Router, extract::State, routing::get};
use serde::Serialize;

use crate::routes;

use super::AppState;

#[derive(Serialize)]
struct HealthResponse {
    message: &'static str,
}

async fn health(State(_): State<AppState>) -> Json<HealthResponse> {
    Json(HealthResponse {
        message: "Server running",
    })
}

pub fn build(state: AppState) -> Router {
    Router::new()
        .route("/", get(health))
        .route(
            "/auth/register",
            axum::routing::post(routes::auth::register),
        )
        .route("/terminal", axum::routing::post(routes::terminal::execute))
        .with_state(state)
}
