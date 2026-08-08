use anyhow::Result;
use async_trait::async_trait;

use crate::models::{Session, User};

#[async_trait]
pub trait UserRepository: Send + Sync + 'static {
    async fn create(&self, user: &User) -> Result<()>;

    async fn find_by_id(&self, id: &str) -> Result<Option<User>>;

    async fn find_by_username(&self, username: &str) -> Result<Option<User>>;

    async fn create_session(&self, session: &Session) -> Result<()>;

    async fn find_session(&self, token: &str) -> Result<Option<Session>>;

    async fn delete_session(&self, token: &str) -> Result<()>;
}
