use anyhow::Result;
use async_trait::async_trait;
use sqlx::SqlitePool;

use crate::{models::User, repository::UserRepository};

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
}
