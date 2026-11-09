//! Exact rational (`p/q`) and integer basis-point models for `kompress-super-ultra`.
//!
//! Per Rule 4 & Rule 7 in `GEMINI.md`, all state coordinates and token accounting use
//! canonical reduced rationals (`Ratio<u64>`) and integer basis points (`0..=10_000`)
//! without floating-point accumulator drift.

use serde::{Deserialize, Serialize};

/// Canonical Rung 0 Bonobo harmonic ratio (`3/2`).
pub const BONOBO_HARMONIC_PQ: &str = "3/2";

/// Canonical Techno-Zen-Bonobo compound ratio (`(3/2) * (3/4) = 9/8`).
pub const TECHNO_ZEN_BONOBO_PQ: &str = "9/8";

/// Cumulative telemetry snapshot for the `kompress-super-ultra` cluster node.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct KompressStats {
    /// Total compression requests processed since daemon startup.
    pub total_requests: u64,
    /// Cumulative raw input tokens processed.
    pub input_tokens: u64,
    /// Cumulative folded output tokens emitted.
    pub output_tokens: u64,
    /// Cumulative tokens saved (`input_tokens - output_tokens`).
    pub saved_tokens: u64,
    /// Canonical reduced rational `saved_tokens / input_tokens` as `"p/q"`.
    pub savings_ratio_pq: String,
    /// Canonical reduced rational `output_tokens / input_tokens` as `"p/q"`.
    pub compression_ratio_pq: String,
    /// Exact integer basis points of token savings (`0..=10_000`, where `100 bps = 1.00%`).
    pub savings_bps: u64,
    /// Rung 0 Bonobo target resonance (`"3/2"`).
    pub bonobo_harmonic_pq: String,
    /// Techno-Zen-Bonobo harmonic (`"9/8"`).
    pub techno_zen_bonobo_pq: String,
    /// Balanced ternary resonance trit (`+1` when `in/out >= 3/2`, `0` when `saved > 0`, `-1` otherwise).
    pub target_resonance_trit: i8,
    /// Exponential moving average fold latency in microseconds.
    pub latency_us_avg: u64,
}

impl Default for KompressStats {
    fn default() -> Self {
        Self {
            total_requests: 0,
            input_tokens: 0,
            output_tokens: 0,
            saved_tokens: 0,
            savings_ratio_pq: "0/1".to_string(),
            compression_ratio_pq: "1/1".to_string(),
            savings_bps: 0,
            bonobo_harmonic_pq: BONOBO_HARMONIC_PQ.to_string(),
            techno_zen_bonobo_pq: TECHNO_ZEN_BONOBO_PQ.to_string(),
            target_resonance_trit: 0,
            latency_us_avg: 0,
        }
    }
}

/// Individual fold event stored in history and broadcast over WebSocket (`/ws`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompressionLogEntry {
    /// Monotonically increasing request identifier.
    pub id: u64,
    /// Unix epoch timestamp in seconds.
    pub timestamp: u64,
    /// Raw input length in bytes.
    pub input_bytes: usize,
    /// Folded output length in bytes.
    pub output_bytes: usize,
    /// Raw input token count.
    pub input_tokens: u64,
    /// Folded output token count.
    pub output_tokens: u64,
    /// Tokens saved on this request.
    pub saved_tokens: u64,
    /// Canonical reduced rational `output_tokens / input_tokens` as `"p/q"`.
    pub ratio_pq: String,
    /// Canonical reduced rational `saved_tokens / input_tokens` as `"p/q"`.
    pub savings_pq: String,
    /// Exact integer basis points of token savings (`0..=10_000`).
    pub savings_bps: u64,
    /// Balanced ternary resonance trit (`+1`, `0`, `-1`).
    pub resonance_trit: i8,
    /// Execution time in microseconds.
    pub latency_us: u64,
    /// Preview snippet of the folded output.
    pub snippet: String,
}

/// Request payload for `POST /api/kompress`.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct KompressRequest {
    /// Raw text, tool log, ANSI stream, or JSON payload to fold.
    pub text: String,
    /// Optional fold mode (`"standard"`, `"aggressive"`, `"json"`).
    pub mode: Option<String>,
}

/// Response payload for `POST /api/kompress`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct KompressResponse {
    /// Folded text output.
    pub compressed: String,
    /// Original token count.
    pub original_tokens: u64,
    /// Folded token count.
    pub compressed_tokens: u64,
    /// Tokens eliminated (`original_tokens - compressed_tokens`).
    pub saved_tokens: u64,
    /// Canonical reduced rational `compressed_tokens / original_tokens` as `"p/q"`.
    pub compression_ratio_pq: String,
    /// Canonical reduced rational `saved_tokens / original_tokens` as `"p/q"`.
    pub savings_ratio_pq: String,
    /// Exact integer basis points of savings (`0..=10_000`).
    pub compression_savings_bps: u64,
    /// Balanced ternary resonance trit (`+1` if `in/out >= 3/2`, `0` if `saved > 0`, `-1` otherwise).
    pub bonobo_resonance_trit: i8,
    /// Fold execution latency in microseconds.
    pub latency_us: u64,
}

/// Health check response payload for `GET /health`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HealthResponse {
    /// Service status (`"ok"`).
    pub status: String,
    /// Service identifier (`"kompress-super-ultra"`).
    pub service: String,
    /// Crate version.
    pub version: String,
    /// Canonical Rung 0 Bonobo harmonic (`"3/2"`).
    pub bonobo_harmonic_pq: String,
    /// Canonical Techno-Zen-Bonobo harmonic (`"9/8"`).
    pub techno_zen_bonobo_pq: String,
    /// Verified SPARK proof count badge.
    pub spark_invariants: String,
}
