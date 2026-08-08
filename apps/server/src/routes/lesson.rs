use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use crate::{app::AppState, lesson::Lesson};

pub async fn list(State(state): State<AppState>) -> Json<Vec<crate::lesson::Lesson>> {
    Json(state.lesson.list().into_iter().cloned().collect())
}

pub async fn get(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<crate::lesson::Lesson>, StatusCode> {
    let lesson = state
        .lesson
        .get(&id)
        .cloned()
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(lesson))
}

pub async fn current(State(state): State<AppState>) -> Result<Json<Lesson>, StatusCode> {
    let lesson = state
        .lesson_runtime
        .current(1) // demo user for now
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(lesson))
}
