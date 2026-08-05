use std::sync::Arc;

use crate::{auth::AuthService, repository::SqlitePool, repository::sqlite::SqliteUserRepository};

use super::Config;

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub db: SqlitePool,

    pub auth: AuthService<SqliteUserRepository>,
}

impl AppState {
    pub fn new(config: Config, db: SqlitePool) -> Self {
        let user_repository = Arc::new(SqliteUserRepository::new(db.clone()));

        let auth = AuthService::new(user_repository);

        Self { config, db, auth }
    }
}
