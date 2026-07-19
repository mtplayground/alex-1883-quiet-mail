use axum::{routing::get, Json, Router};
use serde::Serialize;
use tower_http::{services::ServeDir, trace::TraceLayer};

use crate::config::Config;

#[derive(Debug, Serialize)]
struct HealthResponse {
    status: &'static str,
}

pub fn build_router(config: Config) -> Router {
    let api = Router::new().route("/health", get(health));

    Router::new()
        .nest("/api", api)
        .fallback_service(
            ServeDir::new(config.frontend_dist).append_index_html_on_directories(true),
        )
        .layer(TraceLayer::new_for_http())
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}
