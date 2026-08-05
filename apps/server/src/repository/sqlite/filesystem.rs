use anyhow::Result;
use async_trait::async_trait;
use sqlx::SqlitePool;

use crate::{models::FilesystemNode, repository::FilesystemRepository};

#[derive(Clone)]
pub struct SqliteFilesystemRepository {
    pool: SqlitePool,
}

impl SqliteFilesystemRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl FilesystemRepository for SqliteFilesystemRepository {
    async fn find_root(&self, user_id: &str) -> Result<Option<FilesystemNode>> {
        Ok(sqlx::query_as::<_, FilesystemNode>(
            r#"
            SELECT *
            FROM filesystem_nodes
            WHERE user_id = ?
              AND parent_id IS NULL
            LIMIT 1
            "#,
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?)
    }

    async fn find_child(&self, parent_id: &str, name: &str) -> Result<Option<FilesystemNode>> {
        Ok(sqlx::query_as::<_, FilesystemNode>(
            r#"
            SELECT *
            FROM filesystem_nodes
            WHERE parent_id = ?
              AND name = ?
            LIMIT 1
            "#,
        )
        .bind(parent_id)
        .bind(name)
        .fetch_optional(&self.pool)
        .await?)
    }

    async fn create_node(&self, node: &FilesystemNode) -> Result<()> {
        sqlx::query(
            r#"
        INSERT INTO filesystem_nodes
        (
            id,
            parent_id,
            user_id,
            name,
            kind,
            content
        )
        VALUES (?, ?, ?, ?, ?, ?)
        "#,
        )
        .bind(&node.id)
        .bind(&node.parent_id)
        .bind(&node.user_id)
        .bind(&node.name)
        .bind(match node.kind {
            crate::models::NodeKind::File => "file",
            crate::models::NodeKind::Directory => "directory",
        })
        .bind(&node.content)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<FilesystemNode>> {
        Ok(sqlx::query_as::<_, FilesystemNode>(
            r#"
            SELECT *
            FROM filesystem_nodes
            WHERE id = ?
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?)
    }

    async fn children(&self, parent_id: &str) -> Result<Vec<FilesystemNode>> {
        Ok(sqlx::query_as::<_, FilesystemNode>(
            r#"
            SELECT *
            FROM filesystem_nodes
            WHERE parent_id = ?
            ORDER BY name
            "#,
        )
        .bind(parent_id)
        .fetch_all(&self.pool)
        .await?)
    }

    async fn update_node(&self, node: &FilesystemNode) -> Result<()> {
        sqlx::query(
            r#"
        UPDATE filesystem_nodes
        SET
            parent_id = ?,
            name = ?,
            content = ?
        WHERE id = ?
        "#,
        )
        .bind(&node.parent_id)
        .bind(&node.name)
        .bind(&node.content)
        .bind(&node.id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete_node(&self, id: &str) -> Result<()> {
        sqlx::query(
            r#"
        DELETE
        FROM filesystem_nodes
        WHERE id = ?
        "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
