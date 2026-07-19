use axum::{extract::State, routing::get, Json, Router};
use serde::Serialize;
use tower_http::{services::ServeDir, trace::TraceLayer};

use crate::{app_state::AppState, auth, config::Config, db::Database, error::AppError, mail};

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
    let app_state = AppState::new(&config, database);
    let mailbox = auth::protect_mailbox_routes(mail::routes(), app_state.clone());
    let api = Router::new()
        .route("/health", get(health))
        .route("/health/db", get(db_ready))
        .route("/auth/login", get(auth::login))
        .route("/auth/session", get(auth::session))
        .nest("/mailbox", mailbox);

    Router::new()
        .nest("/api", api)
        .fallback_service(
            ServeDir::new(config.frontend_dist).append_index_html_on_directories(true),
        )
        .layer(TraceLayer::new_for_http())
        .with_state(app_state)
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}

async fn db_ready(State(state): State<AppState>) -> Result<Json<DbReadyResponse>, AppError> {
    state.database.check_ready().await?;

    Ok(Json(DbReadyResponse {
        status: "ok",
        database: "ready",
    }))
}
