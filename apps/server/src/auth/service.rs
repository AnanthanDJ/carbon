use anyhow::Result;
use chrono::Utc;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    auth::password,
    models::{FilesystemNode, NodeKind, User},
    repository::{FilesystemRepository, UserRepository},
};

#[derive(Clone)]
pub struct AuthService<U, F>
where
    U: UserRepository,
    F: FilesystemRepository,
{
    users: Arc<U>,
    filesystem: Arc<F>,
}

impl<U, F> AuthService<U, F>
where
    U: UserRepository,
    F: FilesystemRepository,
{
    pub fn new(users: Arc<U>, filesystem: Arc<F>) -> Self {
        Self { users, filesystem }
    }

    pub async fn register(
        &self,
        username: &str,
        password: &str,
    ) -> Result<crate::auth::jwt::Token> {
        let _ = (username, password);

        if self.users.find_by_username(username).await?.is_some() {
            anyhow::bail!("username already exists");
        }

        let user = User {
            id: Uuid::new_v4().to_string(),
            username: username.to_owned(),
            password_hash: password::hash(password)?,
            created_at: Utc::now().to_rfc3339(),
        };

        self.users.create(&user).await?;

        let root = FilesystemNode {
            id: Uuid::new_v4().to_string(),
            parent_id: None,
            user_id: user.id.clone(),
            name: "/".into(),
            kind: NodeKind::Directory,
            content: None,
        };

        self.filesystem.create_node(&root).await?;

        let home = FilesystemNode {
            id: Uuid::new_v4().to_string(),
            parent_id: Some(root.id.clone()),
            user_id: user.id.clone(),
            name: "home".into(),
            kind: NodeKind::Directory,
            content: None,
        };

        self.filesystem.create_node(&home).await?;

        Ok(crate::auth::jwt::Token {
            value: String::new(),
        })
    }

    pub async fn login(&self, username: &str, password: &str) -> Result<crate::auth::jwt::Token> {
        let _ = (username, password);

        let user = self
            .users
            .find_by_username(username)
            .await?
            .ok_or_else(|| anyhow::anyhow!("invalid credentials"))?;

        if !password::verify(password, &user.password_hash)? {
            anyhow::bail!("invalid credentials");
        }

        Ok(crate::auth::jwt::Token {
            value: String::new(),
        })
    }
}
