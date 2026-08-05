use anyhow::Result;
use std::sync::Arc;

use crate::models::FilesystemNode;
use crate::repository::FilesystemRepository;
use crate::terminal::TerminalSession;

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
    ) -> Result<Option<FilesystemNode>> {
        let path = crate::filesystem::path::resolve(&session.cwd, input);

        let mut current = match self.repository.find_root(&session.user_id).await? {
            Some(root) => root,
            None => return Ok(None),
        };

        for component in path.components() {
            let Some(child) = self.repository.find_child(&current.id, component).await? else {
                return Ok(None);
            };

            current = child;
        }

        Ok(Some(current))
    }
}
