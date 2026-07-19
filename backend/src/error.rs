use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("{message}")]
    Config {
        message: String,
        detail: Option<String>,
    },
    #[error("database operation failed")]
    Database {
        #[source]
        source: sqlx::Error,
    },
    #[error("database migration failed")]
    Migration {
        #[source]
        source: sqlx::migrate::MigrateError,
    },
    #[error("password hashing failed")]
    PasswordHash { detail: String },
    #[error("authentication failed")]
    Auth { detail: String },
    #[error("request is not authenticated")]
    Unauthorized,
}

#[derive(Debug, Serialize)]
struct ErrorBody<'a> {
    error: &'a str,
    message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match &self {
            AppError::Config { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Database { .. }
            | AppError::Migration { .. }
            | AppError::PasswordHash { .. } => StatusCode::SERVICE_UNAVAILABLE,
            AppError::Auth { .. } | AppError::Unauthorized => StatusCode::UNAUTHORIZED,
        };
        let message = match &self {
            AppError::Config { message, detail } => detail
                .as_ref()
                .map_or_else(|| message.clone(), |detail| format!("{message}: {detail}")),
            AppError::Database { .. } => "database is not ready".to_owned(),
            AppError::Migration { .. } => "database migrations did not complete".to_owned(),
            AppError::PasswordHash { detail } => {
                format!("account bootstrap did not complete: {detail}")
            }
            AppError::Auth { detail } => {
                tracing::warn!(%detail, "authentication failed");
                "authentication required".to_owned()
            }
            AppError::Unauthorized => "authentication required".to_owned(),
        };

        (
            status,
            Json(ErrorBody {
                error: status.canonical_reason().unwrap_or("error"),
                message,
            }),
        )
            .into_response()
    }
}
