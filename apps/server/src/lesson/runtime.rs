use std::sync::Arc;

use anyhow::Result;

use super::LessonService;
use crate::{lesson::Lesson, repository::LessonRepository};

#[derive(Clone)]
pub struct LessonRuntimeService<R>
where
    R: LessonRepository,
{
    repository: Arc<R>,
    lessons: LessonService,
}

#[derive(Debug, Clone)]
pub struct LessonOutcome {
    pub completed: bool,
    pub current: Lesson,
    pub next: Option<Lesson>,
}

impl<R> LessonRuntimeService<R>
where
    R: LessonRepository,
{
    pub fn new(repository: Arc<R>, lessons: LessonService) -> Self {
        Self {
            repository,
            lessons,
        }
    }

    pub async fn current(&self, user_id: i64) -> Result<Option<Lesson>> {
        let Some(id) = self.repository.current_lesson(user_id).await? else {
            return Ok(None);
        };

        Ok(self.lessons.get(&id).cloned())
    }

    pub async fn start(&self, user_id: i64, lesson_id: &str) -> Result<()> {
        self.repository.start_lesson(user_id, lesson_id).await?;

        self.repository
            .set_current_lesson(user_id, lesson_id)
            .await?;

        Ok(())
    }

    pub async fn complete(&self, user_id: i64, lesson_id: &str) -> Result<()> {
        self.repository.complete_lesson(user_id, lesson_id).await?;

        Ok(())
    }

    pub async fn advance(&self, user_id: i64) -> Result<Option<Lesson>> {
        let Some(current) = self.current(user_id).await? else {
            return Ok(None);
        };

        let Some(next) = self.lessons.next(&current) else {
            return Ok(None);
        };

        self.repository
            .set_current_lesson(user_id, &next.id)
            .await?;

        Ok(Some(next.clone()))
    }
}
