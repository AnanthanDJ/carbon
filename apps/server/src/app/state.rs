use std::sync::Arc;

use crate::{
    auth::AuthService,
    filesystem::FilesystemService,
    repository::{
        SqlitePool,
        sqlite::{SqliteFilesystemRepository, SqliteUserRepository},
    },
};

use super::Config;

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub db: SqlitePool,

    pub auth: AuthService<SqliteUserRepository, SqliteFilesystemRepository>,
    pub filesystem: FilesystemService<SqliteFilesystemRepository>,
}

impl AppState {
    pub fn new(config: Config, db: SqlitePool) -> Self {
        let user_repository = Arc::new(SqliteUserRepository::new(db.clone()));
        let filesystem_repository = Arc::new(SqliteFilesystemRepository::new(db.clone()));

        let auth = AuthService::new(user_repository, filesystem_repository.clone());
        let filesystem = FilesystemService::new(filesystem_repository);

        Self {
            config,
            db,
            auth,
            filesystem,
        }
    }
}
