use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Invalid username or password")]
    InvalidCredentials,

    #[error("Username already exists")]
    UsernameTaken,

    #[error("Internal error")]
    Internal,
}
