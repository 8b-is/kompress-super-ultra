//! Route table construction for `kompress-super-ultra`.

pub mod health;
pub mod kompress;

use crate::{handlers::serve_webui, services::AppState};
use axum::{routing::get, Router};
use tower_http::cors::CorsLayer;

/// Construct the full Axum [`Router`] with WebUI, WebSocket, Health, OpenAPI, and Kompress routes.
pub fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(serve_webui))
        .route("/health", get(health::health_check))
        .route("/openapi.json", get(health::openapi_spec))
        .merge(kompress::router())
        .layer(CorsLayer::permissive())
        .with_state(state)
}
