use axum::{extract::State, routing::get, Json, Router};
use serde::Serialize;
use tower_http::{services::ServeDir, trace::TraceLayer};

use crate::{config::Config, db::Database, error::AppError};

#[derive(Debug, Serialize)]
struct HealthResponse {
    status: &'static str,
}

#[derive(Debug, Serialize)]
struct DbReadyResponse {
    status: &'static str,
    database: &'static str,
}

pub fn build_router(config: Config, database: Database) -> Router {
    let api = Router::new()
        .route("/health", get(health))
        .route("/health/db", get(db_ready));

    Router::new()
        .nest("/api", api)
        .fallback_service(
            ServeDir::new(config.frontend_dist).append_index_html_on_directories(true),
        )
        .layer(TraceLayer::new_for_http())
        .with_state(database)
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}

async fn db_ready(State(database): State<Database>) -> Result<Json<DbReadyResponse>, AppError> {
    database.check_ready().await?;

    Ok(Json(DbReadyResponse {
        status: "ok",
        database: "ready",
    }))
}
