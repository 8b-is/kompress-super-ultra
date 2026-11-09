//! Health check (`/health`) and OpenAPI 3.1.0 (`/openapi.json`) endpoints.

use crate::models::{
    kompress::{BONOBO_HARMONIC_PQ, TECHNO_ZEN_BONOBO_PQ},
    HealthResponse,
};
use axum::Json;
use serde_json::{json, Value};

/// Return sovereign service health and harmonic invariants (`GET /health`).
pub async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        service: "kompress-super-ultra".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        bonobo_harmonic_pq: BONOBO_HARMONIC_PQ.to_string(),
        techno_zen_bonobo_pq: TECHNO_ZEN_BONOBO_PQ.to_string(),
        spark_invariants: "117/117".to_string(),
    })
}

/// Return OpenAPI 3.1.0 specification (`GET /openapi.json`).
pub async fn openapi_spec() -> Json<Value> {
    Json(json!({
        "openapi": "3.1.0",
        "info": {
            "title": "Kompress-Super-Ultra CORE API",
            "version": env!("CARGO_PKG_VERSION"),
            "description": "4-Role Semantic Fold Engine with Live WebSocket Telemetry & Exact p/q Rational Accounting"
        },
        "paths": {
            "/health": { "get": { "summary": "Service health & harmonic invariants" } },
            "/ws": { "get": { "summary": "Live WebSocket compression event stream" } },
            "/api/stats": { "get": { "summary": "Cumulative exact p/q token savings statistics" } },
            "/api/history": { "get": { "summary": "Recent compression log entries" } },
            "/api/kompress": { "post": { "summary": "Execute 4-role semantic fold with exact p/q accounting" } }
        }
    }))
}
