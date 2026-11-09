//! `kompress-super-ultra` — Headroom + Kompress-Super-Ultra CORE library and Web API.
//!
//! Provides a 4-role semantic folding engine (Pruner, Rewriter, Composer, Circulator),
//! exact rational (`p/q`) and integer basis-point (`bps`) token accounting, and an
//! Axum + WebSocket real-time telemetry server.
//!
//! # Examples
//!
//! ```
//! use kompress_super_ultra::{AdvancedKompressEngine, FoldMode};
//!
//! let engine = AdvancedKompressEngine::new();
//! let sample = "\x1B[31mERROR\x1B[0m\nretrying upstream connection after timeout\nretrying upstream connection after timeout\nretrying upstream connection after timeout";
//! let outcome = engine.fold_with_metrics(sample, FoldMode::Standard);
//! assert!(outcome.compressed.contains("... [repeated 2 times] ..."));
//! assert!(outcome.saved_tokens > 0);
//! ```

pub mod cli;
pub mod commands;
pub mod config;
pub mod error;
pub mod handlers;
pub mod middleware;
pub mod models;
pub mod routes;
pub mod services;

pub use config::{AppConfig, FoldMode};
pub use error::{AppError, Result};
pub use models::{
    CompressionLogEntry, HealthResponse, KompressRequest, KompressResponse, KompressStats,
};
pub use routes::build_router;
pub use services::{
    compute_savings_bps, evaluate_bonobo_resonance, AdvancedKompressEngine, AppState, FoldOutcome,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bonobo_resonance_exact_integer_threshold() {
        // 30 -> 20 tokens is 3/2 (1.5x) -> +1 trit
        assert_eq!(evaluate_bonobo_resonance(30, 20), 1);
        // 30 -> 25 tokens is 6/5 (1.2x) -> 0 trit
        assert_eq!(evaluate_bonobo_resonance(30, 25), 0);
        // 20 -> 20 tokens is 1/1 -> -1 trit
        assert_eq!(evaluate_bonobo_resonance(20, 20), -1);
    }

    #[test]
    fn fold_engine_strips_ansi_and_deduplicates() {
        let engine = AdvancedKompressEngine::new();
        let raw = "\x1B[32mINFO start\x1B[0m\n00000000: 41 42 43 44 45 46\ntick\ntick\ntick";
        let outcome = engine.fold_with_metrics(raw, FoldMode::Standard);
        assert!(!outcome.compressed.contains("\x1B[32m"));
        assert!(outcome.compressed.contains("[...hex dump pruned...]"));
        assert!(outcome.compressed.contains("... [repeated 2 times] ..."));
        assert_eq!(
            compute_savings_bps(outcome.saved_tokens, outcome.original_tokens),
            outcome.savings_bps
        );
    }
}
