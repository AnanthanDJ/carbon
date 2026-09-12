mod config;
mod error;
mod router;
mod state;

use crate::app::config::Config;
use anyhow::Result;
use axum::http::{HeaderValue, Method};
pub use state::AppState;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
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

    let state = AppState::new(config.clone(), db)?;

    let cors = CorsLayer::new()
        .allow_origin(
            "https://carbon-dw3.pages.dev"
                .parse::<HeaderValue>()
                .unwrap(),
        )
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers(Any);

    let app = router::build(state).layer(cors);

    let address = format!("{}:{}", config.host, config.port,);

    let listener = TcpListener::bind(&address).await?;

    tracing::info!("Listening on http://{address}");

    axum::serve(listener, app).await?;

    Ok(())
}
