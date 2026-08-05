use crate::filesystem::ResolvedPath;

#[derive(Debug, Clone)]
pub struct TerminalSession {
    pub user_id: String,
    pub cwd: ResolvedPath,
}

impl TerminalSession {
    pub fn new(user_id: impl Into<String>) -> Self {
        Self {
            user_id: user_id.into(),
            cwd: ResolvedPath::root(),
        }
    }
}
