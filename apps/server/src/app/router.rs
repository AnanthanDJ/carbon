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
        .route("/auth/login", axum::routing::post(routes::auth::login))
        .route("/auth/me", get(routes::auth::me))
        .route("/terminal", axum::routing::post(routes::terminal::execute))
        .route("/lessons", get(routes::lesson::list))
        .route("/lessons/{id}", get(routes::lesson::get))
        .route("/lessons/current", get(routes::lesson::current))
        .route("/health", get(routes::health::health))
        .with_state(state)
}
