use axum::{
    Json,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
};

use crate::{
    app::AppState, lesson::Lesson, models::CourseProgress,
    routes::auth_helper::authenticated_user_id,
};

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

pub async fn current(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Lesson>, StatusCode> {
    let user_id = authenticated_user_id(&state, &headers).await?;

    let lesson = state
        .lesson_runtime
        .current(user_id.clone())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(lesson))
}

pub async fn progress(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<CourseProgress>, StatusCode> {
    let user_id = authenticated_user_id(&state, &headers).await?;

    let progress = state
        .lesson_runtime
        .progress(user_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(progress))
}
