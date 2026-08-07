use std::sync::Arc;

use crate::{
    auth::AuthService,
    filesystem::FilesystemService,
    lesson::LessonService,
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
    pub lesson: LessonService,
}

impl AppState {
    pub fn new(config: Config, db: SqlitePool) -> anyhow::Result<Self> {
        let user_repository = Arc::new(SqliteUserRepository::new(db.clone()));
        let filesystem_repository = Arc::new(SqliteFilesystemRepository::new(db.clone()));

        let auth = AuthService::new(user_repository, filesystem_repository.clone());
        let filesystem = FilesystemService::new(filesystem_repository);
        let lesson = LessonService::load("test_content")?;
        //let lesson = LessonService::load("content/lessons")?;

        Ok(Self {
            config,
            db,
            auth,
            filesystem,
            lesson,
        })
    }
}
