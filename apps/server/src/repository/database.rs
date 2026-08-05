use anyhow::Result;
use sqlx::{SqlitePool, migrate::Migrator, sqlite::SqlitePoolOptions};

static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

pub async fn connect(database_url: &str) -> Result<SqlitePool> {
    let pool = SqlitePoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await?;

    MIGRATOR.run(&pool).await?;

    Ok(pool)
}
