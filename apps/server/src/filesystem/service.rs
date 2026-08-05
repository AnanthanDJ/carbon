use std::sync::Arc;

use crate::filesystem::{FilesystemError, resolve};
use crate::models::FilesystemNode;
use crate::repository::FilesystemRepository;
use crate::terminal::TerminalSession;

#[derive(Clone)]
pub struct FilesystemService<R>
where
    R: FilesystemRepository,
{
    repository: Arc<R>,
}

impl<R> FilesystemService<R>
where
    R: FilesystemRepository,
{
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    pub async fn resolve_node(
        &self,
        session: &TerminalSession,
        input: &str,
    ) -> anyhow::Result<FilesystemNode> {
        let path = resolve(&session.cwd, input);

        let mut current = self
            .repository
            .find_root(&session.user_id)
            .await?
            .ok_or(FilesystemError::NotFound)?;

        for component in path.components() {
            let child = self
                .repository
                .find_child(&current.id, component)
                .await?
                .ok_or(FilesystemError::NotFound)?;

            current = child;
        }

        Ok(current)
    }

    async fn resolve_parent(
        &self,
        session: &TerminalSession,
        input: &str,
    ) -> anyhow::Result<(FilesystemNode, String)> {
        let path = resolve(&session.cwd, input);

        let parent_path = path.parent();

        let name = path
            .file_name()
            .ok_or(FilesystemError::InvalidPath)?
            .to_string();

        let parent = self.resolve_node(session, &parent_path.to_string()).await?;

        Ok((parent, name))
    }

    pub async fn ls(
        &self,
        session: &TerminalSession,
        input: &str,
    ) -> anyhow::Result<Vec<FilesystemNode>> {
        let node = self.resolve_node(session, input).await?;

        if !matches!(node.kind, crate::models::NodeKind::Directory) {
            return Err(FilesystemError::NotDirectory.into());
        }

        self.repository.children(&node.id).await
    }

    pub fn pwd(&self, session: &TerminalSession) -> String {
        session.cwd.to_string()
    }

    pub async fn cd(&self, session: &mut TerminalSession, input: &str) -> anyhow::Result<()> {
        let node = self.resolve_node(session, input).await?;

        if !matches!(node.kind, crate::models::NodeKind::Directory) {
            return Err(FilesystemError::NotDirectory.into());
        }

        session.cwd = resolve(&session.cwd, input);

        Ok(())
    }

    pub async fn mkdir(&self, session: &TerminalSession, input: &str) -> anyhow::Result<()> {
        let (parent, name) = self.resolve_parent(session, input).await?;

        if !matches!(parent.kind, crate::models::NodeKind::Directory) {
            return Err(FilesystemError::NotDirectory.into());
        }

        if self
            .repository
            .find_child(&parent.id, &name)
            .await?
            .is_some()
        {
            return Err(FilesystemError::AlreadyExists.into());
        }

        let node = FilesystemNode {
            id: uuid::Uuid::new_v4().to_string(),
            parent_id: Some(parent.id),
            user_id: session.user_id.clone(),
            name,
            kind: crate::models::NodeKind::Directory,
            content: None,
        };

        self.repository.create_node(&node).await?;

        Ok(())
    }

    pub async fn touch(&self, session: &TerminalSession, input: &str) -> anyhow::Result<()> {
        let (parent, name) = self.resolve_parent(session, input).await?;

        if !matches!(parent.kind, crate::models::NodeKind::Directory) {
            return Err(FilesystemError::NotDirectory.into());
        }

        if self
            .repository
            .find_child(&parent.id, &name)
            .await?
            .is_some()
        {
            return Err(FilesystemError::AlreadyExists.into());
        }

        let node = FilesystemNode {
            id: uuid::Uuid::new_v4().to_string(),
            parent_id: Some(parent.id),
            user_id: session.user_id.clone(),
            name,
            kind: crate::models::NodeKind::File,
            content: Some(String::new()),
        };

        self.repository.create_node(&node).await?;

        Ok(())
    }

    pub async fn cat(&self, session: &TerminalSession, input: &str) -> anyhow::Result<String> {
        let node = self.resolve_node(session, input).await?;

        if !matches!(node.kind, crate::models::NodeKind::File) {
            return Err(FilesystemError::NotFile.into());
        }

        Ok(node.content.unwrap_or_default())
    }

    pub async fn rm(&self, session: &TerminalSession, input: &str) -> anyhow::Result<()> {
        let node = self.resolve_node(session, input).await?;

        if !matches!(node.kind, crate::models::NodeKind::File) {
            return Err(FilesystemError::NotFile.into());
        }

        self.repository.delete_node(&node.id).await?;

        Ok(())
    }

    pub async fn rmdir(&self, session: &TerminalSession, input: &str) -> anyhow::Result<()> {
        let node = self.resolve_node(session, input).await?;

        if !matches!(node.kind, crate::models::NodeKind::Directory) {
            return Err(FilesystemError::NotDirectory.into());
        }

        if !self.repository.children(&node.id).await?.is_empty() {
            return Err(FilesystemError::DirectoryNotEmpty.into());
        }

        self.repository.delete_node(&node.id).await?;

        Ok(())
    }
}
