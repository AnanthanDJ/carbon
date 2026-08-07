use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

use crate::models::{LessonAttempt, LessonProgress};

#[async_trait]
pub trait LessonRepository: Send + Sync {
    async fn current_lesson(&self, user_id: Uuid) -> Result<Option<String>>;

    async fn set_current_lesson(&self, user_id: Uuid, lesson_id: &str) -> Result<()>;

    async fn progress(&self, user_id: Uuid, lesson_id: &str) -> Result<Option<LessonProgress>>;

    async fn start_lesson(&self, user_id: Uuid, lesson_id: &str) -> Result<()>;

    async fn complete_lesson(&self, user_id: Uuid, lesson_id: &str) -> Result<()>;

    async fn record_attempt(&self, attempt: LessonAttempt) -> Result<()>;
}
