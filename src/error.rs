//! Structured error types for `kompress-super-ultra`.

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use std::fmt;

/// Application error variants across CLI, 4-role semantic fold engine, and Axum Web API.
#[derive(Debug)]
pub enum AppError {
    /// Requested resource or profile was not found.
    NotFound(String),
    /// User or API payload input was invalid.
    InvalidInput(String),
    /// Standard I/O failure.
    IoError(std::io::Error),
    /// JSON serialization or deserialization failure.
    JsonError(serde_json::Error),
    /// Internal synchronization lock poisoned.
    StatePoisoned(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound(msg) => write!(f, "Not found: {msg}"),
            Self::InvalidInput(msg) => write!(f, "Invalid input: {msg}"),
            Self::IoError(err) => write!(f, "IO error: {err}"),
            Self::JsonError(err) => write!(f, "JSON error: {err}"),
            Self::StatePoisoned(msg) => write!(f, "State lock poisoned: {msg}"),
        }
    }
}

impl std::error::Error for AppError {}

impl From<std::io::Error> for AppError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(value: serde_json::Error) -> Self {
        Self::JsonError(value)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            Self::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            Self::InvalidInput(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            Self::IoError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
            Self::JsonError(err) => (StatusCode::UNPROCESSABLE_ENTITY, err.to_string()),
            Self::StatePoisoned(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
        };

        (status, Json(json!({ "error": message, "status": status.as_u16() }))).into_response()
    }
}

/// Canonical crate result alias.
pub type Result<T> = std::result::Result<T, AppError>;
