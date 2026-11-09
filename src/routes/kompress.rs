//! Kompress API and WebSocket sub-router (`/ws`, `/api/stats`, `/api/history`, `/api/kompress`).

use crate::{
    handlers::{get_history, get_stats, handle_kompress, ws_handler},
    services::AppState,
};
use axum::{
    routing::{get, post},
    Router,
};

/// Build the `/ws` and `/api/*` router.
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/ws", get(ws_handler))
        .route("/api/stats", get(get_stats))
        .route("/api/history", get(get_history))
        .route("/api/kompress", post(handle_kompress))
}
