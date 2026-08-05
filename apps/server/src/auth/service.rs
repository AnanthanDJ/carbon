use std::sync::Arc;

use anyhow::Result;

use crate::repository::UserRepository;

use chrono::Utc;
use uuid::Uuid;

use crate::{auth::password, models::User};

#[derive(Clone)]
pub struct AuthService<R>
where
    R: UserRepository,
{
    repository: Arc<R>,
}

impl<R> AuthService<R>
where
    R: UserRepository,
{
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    pub async fn register(
        &self,
        username: &str,
        password: &str,
    ) -> Result<crate::auth::jwt::Token> {
        let _ = (username, password);

        if self.repository.find_by_username(username).await?.is_some() {
            anyhow::bail!("username already exists");
        }

        let user = User {
            id: Uuid::new_v4().to_string(),
            username: username.to_owned(),
            password_hash: password::hash(password)?,
            created_at: Utc::now().to_rfc3339(),
        };

        self.repository.create(&user).await?;

        Ok(crate::auth::jwt::Token {
            value: String::new(),
        })
    }

    pub async fn login(&self, username: &str, password: &str) -> Result<crate::auth::jwt::Token> {
        let _ = (username, password);

        let user = self
            .repository
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
