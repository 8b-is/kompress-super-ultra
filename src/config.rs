//! Application configuration and semantic fold modes for `kompress-super-ultra`.

use serde::{Deserialize, Serialize};

/// Semantic folding aggressiveness level.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum FoldMode {
    /// Standard 4-role fold: strips ANSI, prunes hex dumps, minifies JSON, deduplicates runs.
    #[default]
    Standard,
    /// Aggressive fold: also normalizes ISO-8601 timestamps and UUIDv4 identifiers.
    Aggressive,
    /// JSON-only structural minification and line deduplication.
    JsonOnly,
}

impl FoldMode {
    /// Parse a mode string (`"standard"`, `"aggressive"`, `"json"`).
    pub fn parse_opt(mode: Option<&str>) -> Self {
        match mode.map(|s| s.trim().to_ascii_lowercase()).as_deref() {
            Some("aggressive" | "ultra" | "max") => Self::Aggressive,
            Some("json" | "json_only" | "json-only") => Self::JsonOnly,
            _ => Self::Standard,
        }
    }
}

/// Runtime configuration for the `kompress-super-ultra` service and CLI.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppConfig {
    /// TCP bind address for the Axum + WebSocket server (default: `0.0.0.0:8799`).
    pub bind_addr: String,
    /// Maximum number of recent compression entries retained in the ring buffer.
    pub history_capacity: usize,
    /// Capacity of the Tokio broadcast channel for live WebSocket events.
    pub broadcast_capacity: usize,
    /// Default semantic fold mode when none is specified in the request.
    pub default_fold_mode: FoldMode,
    /// Enable verbose diagnostic output.
    pub verbose: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        let port = std::env::var("PORT").unwrap_or_else(|_| "8799".to_string());
        Self {
            bind_addr: format!("0.0.0.0:{port}"),
            history_capacity: 100,
            broadcast_capacity: 128,
            default_fold_mode: FoldMode::Standard,
            verbose: false,
        }
    }
}
