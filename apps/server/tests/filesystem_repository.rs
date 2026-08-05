use anyhow::Result;
use chrono::Utc;
use std::sync::Arc;

use server::{
    filesystem::{FilesystemService, ResolvedPath, resolve},
    models::{FilesystemNode, NodeKind, User},
    repository::{
        FilesystemRepository, UserRepository, connect,
        sqlite::{SqliteFilesystemRepository, SqliteUserRepository},
    },
    terminal::TerminalSession,
};

#[tokio::test]
async fn resolve_existing_path() -> Result<()> {
    let db = connect("sqlite::memory:").await?;
    let user_repo = SqliteUserRepository::new(db.clone());

    let user = User {
        id: "user".into(),
        username: "tester".into(),
        password_hash: "hash".into(),
        created_at: Utc::now().to_rfc3339(),
    };

    user_repo.create(&user).await?;

    let repo = SqliteFilesystemRepository::new(db);

    let root = FilesystemNode {
        id: "root".into(),
        parent_id: None,
        user_id: "user".into(),
        name: "/".into(),
        kind: NodeKind::Directory,
        content: None,
    };

    repo.create_node(&root).await?;

    let home = FilesystemNode {
        id: "home".into(),
        parent_id: Some(root.id.clone()),
        user_id: "user".into(),
        name: "home".into(),
        kind: NodeKind::Directory,
        content: None,
    };

    repo.create_node(&home).await?;

    let projects = FilesystemNode {
        id: "projects".into(),
        parent_id: Some(home.id.clone()),
        user_id: "user".into(),
        name: "projects".into(),
        kind: NodeKind::Directory,
        content: None,
    };

    repo.create_node(&projects).await?;

    let rust = FilesystemNode {
        id: "rust".into(),
        parent_id: Some(projects.id.clone()),
        user_id: "user".into(),
        name: "rust".into(),
        kind: NodeKind::Directory,
        content: None,
    };

    repo.create_node(&rust).await?;

    let fs = FilesystemService::new(Arc::new(repo));

    let session = TerminalSession {
        user_id: user.id.clone(),
        cwd: ResolvedPath::root(),
    };

    let node = fs.resolve_node(&session, "home/projects").await?;

    assert_eq!(node.name, "projects",);

    Ok(())
}

#[tokio::test]
async fn resolve_relative_path() -> Result<()> {
    let db = connect("sqlite::memory:").await?;
    let user_repo = SqliteUserRepository::new(db.clone());

    let user = User {
        id: "user".into(),
        username: "tester".into(),
        password_hash: "hash".into(),
        created_at: Utc::now().to_rfc3339(),
    };

    user_repo.create(&user).await?;

    let repo = SqliteFilesystemRepository::new(db);

    let root = FilesystemNode {
        id: "root".into(),
        parent_id: None,
        user_id: "user".into(),
        name: "/".into(),
        kind: NodeKind::Directory,
        content: None,
    };

    repo.create_node(&root).await?;

    let home = FilesystemNode {
        id: "home".into(),
        parent_id: Some(root.id.clone()),
        user_id: "user".into(),
        name: "home".into(),
        kind: NodeKind::Directory,
        content: None,
    };

    repo.create_node(&home).await?;

    let projects = FilesystemNode {
        id: "projects".into(),
        parent_id: Some(home.id.clone()),
        user_id: "user".into(),
        name: "projects".into(),
        kind: NodeKind::Directory,
        content: None,
    };

    repo.create_node(&projects).await?;

    let rust = FilesystemNode {
        id: "rust".into(),
        parent_id: Some(projects.id.clone()),
        user_id: "user".into(),
        name: "rust".into(),
        kind: NodeKind::Directory,
        content: None,
    };

    repo.create_node(&rust).await?;

    let fs = FilesystemService::new(Arc::new(repo));

    let session = TerminalSession {
        user_id: user.id.clone(),
        cwd: resolve(&ResolvedPath::root(), "home"),
    };

    let node = fs.resolve_node(&session, "projects/rust").await?;

    assert_eq!(node.name, "rust",);

    Ok(())
}

#[tokio::test]
async fn resolve_parent_directory() -> Result<()> {
    let db = connect("sqlite::memory:").await?;
    let user_repo = SqliteUserRepository::new(db.clone());

    let user = User {
        id: "user".into(),
        username: "tester".into(),
        password_hash: "hash".into(),
        created_at: Utc::now().to_rfc3339(),
    };

    user_repo.create(&user).await?;

    let repo = SqliteFilesystemRepository::new(db);

    let root = FilesystemNode {
        id: "root".into(),
        parent_id: None,
        user_id: "user".into(),
        name: "/".into(),
        kind: NodeKind::Directory,
        content: None,
    };

    repo.create_node(&root).await?;

    let home = FilesystemNode {
        id: "home".into(),
        parent_id: Some(root.id.clone()),
        user_id: "user".into(),
        name: "home".into(),
        kind: NodeKind::Directory,
        content: None,
    };

    repo.create_node(&home).await?;

    let projects = FilesystemNode {
        id: "projects".into(),
        parent_id: Some(home.id.clone()),
        user_id: "user".into(),
        name: "projects".into(),
        kind: NodeKind::Directory,
        content: None,
    };

    repo.create_node(&projects).await?;

    let fs = FilesystemService::new(Arc::new(repo));

    let session = TerminalSession {
        user_id: user.id.clone(),
        cwd: resolve(&ResolvedPath::root(), "home/projects"),
    };

    let node = fs.resolve_node(&session, "..").await?;

    assert_eq!(node.name, "home",);

    Ok(())
}

#[tokio::test]
async fn resolve_missing_path() -> Result<()> {
    let db = connect("sqlite::memory:").await?;
    let user_repo = SqliteUserRepository::new(db.clone());

    let user = User {
        id: "user".into(),
        username: "tester".into(),
        password_hash: "hash".into(),
        created_at: Utc::now().to_rfc3339(),
    };

    user_repo.create(&user).await?;

    let repo = SqliteFilesystemRepository::new(db);

    let root = FilesystemNode {
        id: "root".into(),
        parent_id: None,
        user_id: "user".into(),
        name: "/".into(),
        kind: NodeKind::Directory,
        content: None,
    };

    repo.create_node(&root).await?;

    let fs = FilesystemService::new(Arc::new(repo));

    let session = TerminalSession {
        user_id: user.id.clone(),
        cwd: resolve(&ResolvedPath::root(), "home"),
    };

    let err = fs.resolve_node(&session, "does_not_exist").await;

    assert!(matches!(err, Err(_)));

    Ok(())
}
