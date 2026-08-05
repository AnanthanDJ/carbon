use thiserror::Error;

#[derive(Debug, Error)]
pub enum FilesystemError {
    #[error("Node not found")]
    NotFound,

    #[error("Already exists")]
    AlreadyExists,

    #[error("Not a directory")]
    NotDirectory,

    #[error("Invalid path")]
    InvalidPath,
}
