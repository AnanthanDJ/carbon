use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Row, SqlitePool};

use crate::{
    models::{LessonAttempt, LessonProgress, LessonStatus},
    repository::LessonRepository,
};

#[derive(Clone)]
pub struct SqliteLessonRepository {
    pool: SqlitePool,
}

impl SqliteLessonRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl LessonRepository for SqliteLessonRepository {
    async fn current_lesson(&self, user_id: String) -> Result<Option<String>> {
        let lesson = sqlx::query_scalar::<_, String>(
            r#"
        SELECT lesson_id
        FROM users_current_lesson
        WHERE user_id = ?
        "#,
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(lesson)
    }

    async fn set_current_lesson(&self, user_id: String, lesson_id: &str) -> Result<()> {
        sqlx::query(
            r#"
        INSERT INTO users_current_lesson (
            user_id,
            lesson_id,
            updated_at
        )
        VALUES (?, ?, CURRENT_TIMESTAMP)
        ON CONFLICT(user_id)
        DO UPDATE SET
            lesson_id = excluded.lesson_id,
            updated_at = CURRENT_TIMESTAMP
        "#,
        )
        .bind(user_id)
        .bind(lesson_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn progress(&self, user_id: String, lesson_id: &str) -> Result<Option<LessonProgress>> {
        let row = sqlx::query(
            r#"
        SELECT
            status,
            started_at,
            completed_at
        FROM lesson_progress
        WHERE user_id = ?
          AND lesson_id = ?
        "#,
        )
        .bind(&user_id)
        .bind(lesson_id)
        .fetch_optional(&self.pool)
        .await?;

        let Some(row) = row else {
            return Ok(None);
        };

        let status: String = row.try_get("status")?;

        let started_at: String = row.try_get("started_at")?;
        let completed_at: Option<String> = row.try_get("completed_at")?;

        Ok(Some(LessonProgress {
            user_id,
            lesson_id: lesson_id.to_owned(),
            status: match status.as_str() {
                "completed" => LessonStatus::Completed,
                _ => LessonStatus::InProgress,
            },
            started_at: DateTime::parse_from_rfc3339(&started_at)?.with_timezone(&Utc),
            completed_at: completed_at
                .map(|t| DateTime::parse_from_rfc3339(&t).map(|dt| dt.with_timezone(&Utc)))
                .transpose()?,
        }))
    }

    async fn start_lesson(&self, user_id: String, lesson_id: &str) -> Result<()> {
        let now = Utc::now().to_rfc3339();

        sqlx::query(
            r#"
        INSERT INTO lesson_progress (
            user_id,
            lesson_id,
            status,
            started_at
        )
        VALUES (?, ?, ?, ?)
        ON CONFLICT(user_id, lesson_id) DO NOTHING
        "#,
        )
        .bind(user_id)
        .bind(lesson_id)
        .bind("InProgress")
        .bind(now)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn complete_lesson(&self, user_id: String, lesson_id: &str) -> Result<()> {
        let now = Utc::now().to_rfc3339();

        sqlx::query(
            r#"
        UPDATE lesson_progress
        SET
            status = ?,
            completed_at = ?
        WHERE
            user_id = ?
            AND lesson_id = ?
        "#,
        )
        .bind("Completed")
        .bind(now)
        .bind(user_id)
        .bind(lesson_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn record_attempt(&self, attempt: LessonAttempt) -> Result<()> {
        sqlx::query(
            r#"
        INSERT INTO lesson_attempts (
            user_id,
            lesson_id,
            command,
            successful,
            created_at
        )
        VALUES (?, ?, ?, ?, ?)
        "#,
        )
        .bind(attempt.user_id)
        .bind(&attempt.lesson_id)
        .bind(&attempt.command)
        .bind(attempt.successful)
        .bind(attempt.created_at)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn clear_current_lesson(&self, user_id: String) -> Result<()> {
        sqlx::query(
            r#"
        DELETE FROM users_current_lesson
        WHERE user_id = ?
        "#,
        )
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn has_completed_lesson(&self, user_id: String, lesson_id: &str) -> Result<bool> {
        let exists = sqlx::query_scalar::<_, i64>(
            r#"
        SELECT EXISTS(
            SELECT 1
            FROM lesson_progress
            WHERE user_id = ?
              AND lesson_id = ?
              AND status = 'Completed'
        )
        "#,
        )
        .bind(user_id)
        .bind(lesson_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(exists != 0)
    }
}
