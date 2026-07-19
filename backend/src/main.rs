mod account;
mod config;
mod db;
mod error;
mod router;

use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    account::bootstrap_single_account, config::Config, db::Database, error::AppError,
    router::build_router,
};

#[tokio::main]
async fn main() {
    if let Err(error) = run().await {
        tracing::error!(error = %error, "server failed");
        std::process::exit(1);
    }
}

async fn run() -> Result<(), AppError> {
    init_tracing();

    let config = Config::from_env()?;
    config.log_startup_summary();
    let database = Database::connect(&config).await?;
    database.run_migrations().await?;
    bootstrap_single_account(&database, config.bootstrap_account.as_ref()).await?;

    let address = config.socket_addr()?;
    let listener = TcpListener::bind(address)
        .await
        .map_err(|source| AppError::Config {
            message: format!("failed to bind server to {address}"),
            detail: Some(source.to_string()),
        })?;

    tracing::info!(%address, "listening");
    axum::serve(listener, build_router(config, database))
        .with_graceful_shutdown(shutdown_signal())
        .await
        .map_err(|source| AppError::Config {
            message: "server stopped unexpectedly".to_owned(),
            detail: Some(source.to_string()),
        })
}

fn init_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "mailbox_backend=info,tower_http=info,axum=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

async fn shutdown_signal() {
    if let Err(error) = tokio::signal::ctrl_c().await {
        tracing::warn!(%error, "failed to install shutdown signal handler");
    }
}
