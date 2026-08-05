use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

use crate::models::{LessonAttempt, LessonProgress};

#[async_trait]
pub trait LessonRepository: Send + Sync {
    async fn progress(&self, user: Uuid, lesson: &str) -> Result<Option<LessonProgress>>;

    async fn set_progress(&self, progress: &LessonProgress) -> Result<()>;

    async fn add_attempt(&self, attempt: &LessonAttempt) -> Result<()>;

    async fn attempts(&self, user: Uuid, lesson: &str) -> Result<Vec<LessonAttempt>>;
}
