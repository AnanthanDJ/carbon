use std::sync::Arc;

use crate::{
    auth::AuthService,
    filesystem::FilesystemService,
    lesson::{LessonRuntimeService, LessonService},
    repository::{
        SqlitePool,
        sqlite::{SqliteFilesystemRepository, SqliteLessonRepository, SqliteUserRepository},
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
    pub lesson_runtime: LessonRuntimeService<SqliteLessonRepository, SqliteFilesystemRepository>,
}

impl AppState {
    pub fn new(config: Config, db: SqlitePool) -> anyhow::Result<Self> {
        let user_repository = Arc::new(SqliteUserRepository::new(db.clone()));
        let filesystem_repository = Arc::new(SqliteFilesystemRepository::new(db.clone()));

        let auth = AuthService::new(user_repository, filesystem_repository.clone());
        let filesystem = FilesystemService::new(filesystem_repository);
        let lesson_repository = Arc::new(SqliteLessonRepository::new(db.clone()));

        let lesson = LessonService::load("test_content")?;
        //let lesson = LessonService::load("content/lessons")?;

        let lesson_runtime =
            LessonRuntimeService::new(lesson_repository, filesystem.clone(), lesson.clone());

        Ok(Self {
            config,
            db,
            auth,
            filesystem,
            lesson,
            lesson_runtime,
        })
    }
}
