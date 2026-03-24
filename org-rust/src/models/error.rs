use axum::{http::StatusCode, response::IntoResponse, Json};
use thiserror::Error;

use crate::models::response::Tip;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("{0}")]
    BadRequest(String),
    #[error("{0}")]
    NotFound(String),
    #[error("{0}")]
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let tip = Tip::<serde_json::Value>::error(-1, &message);
        (status, Json(tip)).into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;

