use anyhow::Result;
use async_trait::async_trait;

use crate::models::FilesystemNode;

#[async_trait]
pub trait FilesystemRepository: Send + Sync + 'static {
    async fn create_node(&self, node: &FilesystemNode) -> Result<()>;

    async fn update_node(&self, node: &FilesystemNode) -> Result<()>;

    async fn delete_node(&self, id: &str) -> Result<()>;

    async fn find_by_id(&self, id: &str) -> Result<Option<FilesystemNode>>;

    async fn find_root(&self, user_id: &str) -> Result<Option<FilesystemNode>>;

    async fn find_child(&self, parent_id: &str, name: &str) -> Result<Option<FilesystemNode>>;

    async fn children(&self, parent_id: &str) -> Result<Vec<FilesystemNode>>;
}
