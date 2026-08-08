use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};

use crate::{app::AppState, lesson::LessonOutcome, terminal::TerminalSession};

#[derive(Deserialize)]
pub struct TerminalRequest {
    pub command: String,
    pub cwd: String,
}

#[derive(Serialize)]
pub struct TerminalResponse {
    pub stdout: String,
    pub cwd: String,
    pub lesson: Option<LessonOutcome>,
}

pub async fn execute(
    State(state): State<AppState>,
    Json(request): Json<TerminalRequest>,
) -> Result<Json<TerminalResponse>, StatusCode> {
    let mut session = TerminalSession {
        user_id: "demo".into(),
        cwd: request.cwd.as_str().into(),
    };

    let parsed = crate::terminal::parse(&request.command).ok_or(StatusCode::BAD_REQUEST)?;

    let result = crate::terminal::dispatch(&state, &mut session, &parsed)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let lesson = state
        .lesson_runtime
        .validate(
            1, // demo user for now
            &session, &parsed, &result,
        )
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(TerminalResponse {
        stdout: result.stdout,
        cwd: session.cwd.to_string(),
        lesson,
    }))
}
