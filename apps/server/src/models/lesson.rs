use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct LessonProgress {
    pub user_id: String,

    pub lesson_id: String,

    pub completed: bool,
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
