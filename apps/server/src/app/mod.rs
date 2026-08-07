mod config;
mod error;
mod router;
mod seed;
mod state;

use crate::app::config::Config;
use anyhow::Result;
pub use seed::*;
pub use state::AppState;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub async fn run() -> Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::from_env();

    let db = crate::repository::connect(&config.database_url).await?;

    tracing::info!("Connected to SQLite");

    crate::app::seed_demo_user(&db).await?;

    let state = AppState::new(config.clone(), db)?;

    let app = router::build(state);

    let address = format!("{}:{}", config.host, config.port,);

    let listener = TcpListener::bind(&address).await?;

    tracing::info!("Listening on http://{address}");

    axum::serve(listener, app).await?;

    Ok(())
}
