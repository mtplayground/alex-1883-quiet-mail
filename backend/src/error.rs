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
        };
        let message = match &self {
            AppError::Config { message, detail } => detail
                .as_ref()
                .map_or_else(|| message.clone(), |detail| format!("{message}: {detail}")),
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
