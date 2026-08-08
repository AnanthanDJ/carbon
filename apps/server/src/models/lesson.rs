use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LessonStatus {
    InProgress,
    Completed,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct LessonProgress {
    pub user_id: String,
    pub lesson_id: String,
    pub status: LessonStatus,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct LessonAttempt {
    pub id: i64,
    pub user_id: String,
    pub lesson_id: String,
    pub command: String,
    pub successful: bool,
    pub created_at: String,
}
