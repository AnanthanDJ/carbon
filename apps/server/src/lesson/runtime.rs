use anyhow::Result;
use chrono::Utc;
use serde::Serialize;
use std::sync::Arc;

use super::LessonService;
use crate::{
    filesystem::FilesystemService,
    lesson::{Lesson, LessonValidator},
    models::LessonAttempt,
    repository::{FilesystemRepository, LessonRepository},
    terminal::{CommandResult, ParsedCommand, TerminalSession},
};

#[derive(Clone)]
pub struct LessonRuntimeService<R, F>
where
    R: LessonRepository,
    F: FilesystemRepository,
{
    repository: Arc<R>,
    filesystem: FilesystemService<F>,
    lessons: LessonService,
}

#[derive(Debug, Clone, Serialize)]
pub struct LessonOutcome {
    pub completed: bool,
    pub current: Lesson,
    pub next: Option<Lesson>,
}

impl<R, F> LessonRuntimeService<R, F>
where
    R: LessonRepository,
    F: FilesystemRepository,
{
    pub fn new(
        repository: Arc<R>,
        filesystem: FilesystemService<F>,
        lessons: LessonService,
    ) -> Self {
        Self {
            repository,
            filesystem,
            lessons,
        }
    }

    pub async fn current(&self, user_id: String) -> Result<Option<Lesson>> {
        let id = match self.repository.current_lesson(user_id.clone()).await? {
            Some(id) => id,
            None => {
                let binding = self.lessons.list();
                let first = binding
                    .first()
                    .ok_or_else(|| anyhow::anyhow!("no lessons loaded"))?;

                self.start(user_id.clone(), &first.id).await?;

                first.id.clone()
            }
        };

        Ok(self.lessons.get(&id).cloned())
    }

    pub async fn start(&self, user_id: String, lesson_id: &str) -> Result<()> {
        self.repository
            .start_lesson(user_id.clone(), lesson_id)
            .await?;

        self.repository
            .set_current_lesson(user_id, lesson_id)
            .await?;

        Ok(())
    }

    pub async fn complete(&self, user_id: String, lesson_id: &str) -> Result<()> {
        self.repository.complete_lesson(user_id, lesson_id).await?;

        Ok(())
    }

    pub async fn advance(&self, user_id: String) -> Result<Option<Lesson>> {
        let Some(current) = self.current(user_id.clone()).await? else {
            return Ok(None);
        };

        let Some(next) = self.lessons.next(&current) else {
            return Ok(None);
        };

        self.start(user_id, &next.id).await?;

        Ok(Some(next.clone()))
    }

    pub async fn validate(
        &self,
        user_id: String,
        session: &TerminalSession,
        command: &ParsedCommand,
        result: &CommandResult,
    ) -> Result<Option<LessonOutcome>> {
        let Some(current) = self.current(user_id.clone()).await? else {
            return Ok(None);
        };

        let validation =
            LessonValidator::validate(&current, &self.filesystem, session, command, result).await;

        self.repository
            .record_attempt(LessonAttempt {
                id: 0,
                user_id: user_id.clone(),
                lesson_id: current.id.clone(),
                command: command.raw.clone(),
                successful: validation.passed,
                created_at: Utc::now().to_rfc2822(),
            })
            .await?;

        if !validation.passed {
            return Ok(Some(LessonOutcome {
                completed: false,
                current,
                next: None,
            }));
        }

        self.complete(user_id.clone(), &current.id).await?;

        let next = self.advance(user_id.clone()).await?;

        Ok(Some(LessonOutcome {
            completed: true,
            current,
            next,
        }))
    }
}
