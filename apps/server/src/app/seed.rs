use anyhow::Result;
use chrono::Utc;
use uuid::Uuid;

use crate::{
    models::{FilesystemNode, NodeKind, User},
    repository::{
        FilesystemRepository, SqlitePool, UserRepository,
        sqlite::{SqliteFilesystemRepository, SqliteUserRepository},
    },
};

pub async fn seed_demo_user(pool: &SqlitePool) -> Result<()> {
    let users = SqliteUserRepository::new(pool.clone());

    if users.find_by_username("demo").await?.is_some() {
        return Ok(());
    }

    let user = User {
        id: "demo".into(),
        username: "demo".into(),
        password_hash: String::new(),
        created_at: Utc::now().to_rfc3339(),
    };

    users.create(&user).await?;

    let filesystem = SqliteFilesystemRepository::new(pool.clone());

    let root = FilesystemNode {
        id: Uuid::new_v4().to_string(),
        parent_id: None,
        user_id: user.id.clone(),
        name: "/".into(),
        kind: NodeKind::Directory,
        content: None,
    };

    filesystem.create_node(&root).await?;

    let home = FilesystemNode {
        id: Uuid::new_v4().to_string(),
        parent_id: Some(root.id.clone()),
        user_id: user.id.clone(),
        name: "home".into(),
        kind: NodeKind::Directory,
        content: None,
    };

    filesystem.create_node(&home).await?;

    Ok(())
}
