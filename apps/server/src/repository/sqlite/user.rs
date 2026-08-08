use anyhow::Result;
use async_trait::async_trait;
use sqlx::SqlitePool;

use crate::{
    models::{Session, User},
    repository::UserRepository,
};

#[derive(Clone)]
pub struct SqliteUserRepository {
    pool: SqlitePool,
}

impl SqliteUserRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for SqliteUserRepository {
    async fn create(&self, user: &User) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO users
            (
                id,
                username,
                password_hash,
                created_at
            )
            VALUES (?, ?, ?, ?)
            "#,
        )
        .bind(user.id.to_string())
        .bind(&user.username)
        .bind(&user.password_hash)
        .bind(&user.created_at)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT *
            FROM users
            WHERE id = ?
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT *
            FROM users
            WHERE username = ?
            "#,
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    async fn create_session(&self, session: &Session) -> Result<()> {
        sqlx::query(
            r#"
        INSERT INTO sessions (token, user_id, created_at)
        VALUES (?, ?, ?)
        "#,
        )
        .bind(&session.token)
        .bind(session.user_id.clone())
        .bind(session.created_at.clone())
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn find_session(&self, token: &str) -> Result<Option<Session>> {
        let session = sqlx::query_as::<_, Session>(
            r#"
        SELECT token, user_id, created_at
        FROM sessions
        WHERE token = ?
        "#,
        )
        .bind(token)
        .fetch_optional(&self.pool)
        .await?;

        Ok(session)
    }

    async fn delete_session(&self, token: &str) -> Result<()> {
        sqlx::query(
            r#"
        DELETE FROM sessions
        WHERE token = ?
        "#,
        )
        .bind(token)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
