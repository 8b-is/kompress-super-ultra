//! 4-Role Semantic Fold Engine and shared `AppState` with exact `p/q` rational accounting.

use crate::{
    config::{AppConfig, FoldMode},
    error::{AppError, Result},
    models::{CompressionLogEntry, KompressStats},
};
use num_rational::Ratio;
use regex::Regex;
use std::{
    sync::{Arc, Mutex},
    time::{Instant, SystemTime, UNIX_EPOCH},
};
use tokio::sync::broadcast;

/// Exact integer cross-multiplication check for Rung 0 Bonobo resonance (`in / out >= 3 / 2`).
///
/// Returns:
/// - `+1` when `in_tokens * 2 >= out_tokens * 3` (at least 1.5x compression / 33.33% token savings)
/// - `0` when `in_tokens > out_tokens` (positive savings below Rung 0 threshold)
/// - `-1` when no tokens were saved
#[inline]
pub const fn evaluate_bonobo_resonance(in_tokens: u64, out_tokens: u64) -> i8 {
    if out_tokens > 0 && in_tokens.saturating_mul(2) >= out_tokens.saturating_mul(3) {
        1
    } else if in_tokens > out_tokens {
        0
    } else {
        -1
    }
}

/// Compute exact integer basis points (`0..=10_000`) of token savings without floating-point math.
#[inline]
pub const fn compute_savings_bps(saved_tokens: u64, in_tokens: u64) -> u64 {
    if let Some(bps) = saved_tokens.saturating_mul(10_000).checked_div(in_tokens) {
        bps
    } else {
        0
    }
}

/// Complete telemetry outcome of a single 4-role semantic fold operation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FoldOutcome {
    /// Folded output text.
    pub compressed: String,
    /// Input token count.
    pub original_tokens: u64,
    /// Folded token count.
    pub compressed_tokens: u64,
    /// Tokens saved (`original_tokens - compressed_tokens`).
    pub saved_tokens: u64,
    /// Canonical reduced rational `compressed_tokens / original_tokens`.
    pub compression_ratio: Ratio<u64>,
    /// Canonical reduced rational `saved_tokens / original_tokens`.
    pub savings_ratio: Ratio<u64>,
    /// Exact integer basis points of savings (`0..=10_000`).
    pub savings_bps: u64,
    /// Balanced ternary resonance trit (`+1`, `0`, `-1`).
    pub resonance_trit: i8,
    /// Execution time in microseconds.
    pub latency_us: u64,
}

/// Precompiled 4-role semantic fold engine.
#[derive(Clone, Debug)]
pub struct AdvancedKompressEngine {
    ansi_re: Regex,
    hex_dump_re: Regex,
    timestamp_re: Regex,
    guid_re: Regex,
}

impl Default for AdvancedKompressEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl AdvancedKompressEngine {
    /// Construct a new `AdvancedKompressEngine` with precompiled regular expressions.
    pub fn new() -> Self {
        Self {
            ansi_re: Regex::new(r"\x1B\[[0-9;]*[a-zA-Z]").expect("valid ANSI regex"),
            hex_dump_re: Regex::new(r"(?m)^[0-9a-fA-F]{4,8}:[ \t]+([0-9a-fA-F]{2}[ \t]*){4,16}.*$")
                .expect("valid hex dump regex"),
            timestamp_re: Regex::new(
                r"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(\.\d+)?(Z|[+-]\d{2}:\d{2})",
            )
            .expect("valid ISO-8601 regex"),
            guid_re: Regex::new(
                r"[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}",
            )
            .expect("valid UUID regex"),
        }
    }

    /// Execute the default (`FoldMode::Standard`) 4-role semantic fold.
    pub fn fold(&self, input: &str) -> String {
        self.fold_with_mode(input, FoldMode::Standard)
    }

    /// Execute the 4-role semantic fold under the specified [`FoldMode`]:
    /// - **Role 1 (Pruner)**: Strips ANSI escape sequences, voluminous hex dumps, and (in
    ///   `Aggressive` mode) volatile timestamps and UUIDs.
    /// - **Role 2 (Rewriter)**: Minifies JSON structures and normalizes multi-space trace noise.
    /// - **Role 3 (Composer)**: Deduplicates consecutive identical lines into run-length markers.
    /// - **Role 4 (Circulator)**: Exposes exact rational (`p/q`) and basis-point telemetry via
    ///   [`Self::fold_with_metrics`].
    pub fn fold_with_mode(&self, input: &str, mode: FoldMode) -> String {
        let stage1 = if mode == FoldMode::JsonOnly {
            input.to_string()
        } else {
            let no_ansi = self.ansi_re.replace_all(input, "");
            let no_hex = self.hex_dump_re.replace_all(&no_ansi, "[...hex dump pruned...]");
            if mode == FoldMode::Aggressive {
                let no_ts = self.timestamp_re.replace_all(&no_hex, "[TS]");
                self.guid_re.replace_all(&no_ts, "[UUID]").into_owned()
            } else {
                no_hex.into_owned()
            }
        };

        let trimmed_stage1 = stage1.trim();
        let processed_text = if (trimmed_stage1.starts_with('{') && trimmed_stage1.ends_with('}'))
            || (trimmed_stage1.starts_with('[') && trimmed_stage1.ends_with(']'))
        {
            if let Ok(val) = serde_json::from_str::<serde_json::Value>(trimmed_stage1) {
                serde_json::to_string(&val).unwrap_or_else(|_| stage1.clone())
            } else {
                stage1
            }
        } else {
            stage1
        };

        let mut folded_lines = Vec::new();
        let mut prev_line = String::new();
        let mut duplicate_count: usize = 0;

        for line in processed_text.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            let candidate = if (trimmed.starts_with('{') && trimmed.ends_with('}'))
                || (trimmed.starts_with('[') && trimmed.ends_with(']'))
            {
                if let Ok(val) = serde_json::from_str::<serde_json::Value>(trimmed) {
                    serde_json::to_string(&val).unwrap_or_else(|_| trimmed.to_string())
                } else {
                    trimmed.to_string()
                }
            } else {
                trimmed.replace("    ", "  ").replace("  ", " ")
            };

            if candidate == prev_line {
                duplicate_count += 1;
                continue;
            }

            if duplicate_count > 0 {
                folded_lines.push(format!("... [repeated {duplicate_count} times] ..."));
                duplicate_count = 0;
            }

            prev_line.clone_from(&candidate);
            folded_lines.push(candidate);
        }

        if duplicate_count > 0 {
            folded_lines.push(format!("... [repeated {duplicate_count} times] ..."));
        }

        folded_lines.join("\n")
    }

    /// Execute a 4-role semantic fold and return exact `p/q` rational and integer basis-point metrics.
    pub fn fold_with_metrics(&self, input: &str, mode: FoldMode) -> FoldOutcome {
        let start = Instant::now();
        let in_tokens = (input.split_whitespace().count() as u64).max(1);
        let compressed = self.fold_with_mode(input, mode);
        let out_tokens = (compressed.split_whitespace().count() as u64).max(1);
        let saved_tokens = in_tokens.saturating_sub(out_tokens);

        let compression_ratio = Ratio::new(out_tokens, in_tokens);
        let savings_ratio = Ratio::new(saved_tokens, in_tokens);
        let savings_bps = compute_savings_bps(saved_tokens, in_tokens);
        let resonance_trit = evaluate_bonobo_resonance(in_tokens, out_tokens);
        let latency_us = start.elapsed().as_micros() as u64;

        FoldOutcome {
            compressed,
            original_tokens: in_tokens,
            compressed_tokens: out_tokens,
            saved_tokens,
            compression_ratio,
            savings_ratio,
            savings_bps,
            resonance_trit,
            latency_us,
        }
    }
}

/// Shared application state across Axum routes and WebSocket subscribers.
#[derive(Clone)]
pub struct AppState {
    /// Active service configuration.
    pub config: AppConfig,
    /// Shared precompiled 4-role semantic fold engine.
    pub engine: Arc<AdvancedKompressEngine>,
    /// Cumulative exact rational telemetry statistics.
    pub stats: Arc<Mutex<KompressStats>>,
    /// Ring buffer of recent compression log entries.
    pub history: Arc<Mutex<Vec<CompressionLogEntry>>>,
    /// Broadcast channel sender for live WebSocket telemetry.
    pub tx: broadcast::Sender<String>,
}

impl Default for AppState {
    fn default() -> Self {
        Self::new(AppConfig::default())
    }
}

impl AppState {
    /// Create a new `AppState` with precompiled regex engine and broadcast channel.
    pub fn new(config: AppConfig) -> Self {
        let (tx, _rx) = broadcast::channel(config.broadcast_capacity.max(16));
        Self {
            config,
            engine: Arc::new(AdvancedKompressEngine::new()),
            stats: Arc::new(Mutex::new(KompressStats::default())),
            history: Arc::new(Mutex::new(Vec::new())),
            tx,
        }
    }

    /// Record a completed [`FoldOutcome`] into cumulative statistics, history, and WebSocket stream.
    pub fn record_fold(
        &self,
        input_bytes: usize,
        outcome: &FoldOutcome,
    ) -> Result<CompressionLogEntry> {
        let mut stats = self
            .stats
            .lock()
            .map_err(|_| AppError::StatePoisoned("stats mutex poisoned".to_string()))?;

        stats.total_requests += 1;
        stats.input_tokens += outcome.original_tokens;
        stats.output_tokens += outcome.compressed_tokens;
        stats.saved_tokens += outcome.saved_tokens;

        let total_savings_ratio = if stats.input_tokens > 0 {
            Ratio::new(stats.saved_tokens, stats.input_tokens)
        } else {
            Ratio::new(0, 1)
        };
        let total_compression_ratio = if stats.input_tokens > 0 {
            Ratio::new(stats.output_tokens, stats.input_tokens)
        } else {
            Ratio::new(1, 1)
        };

        stats.savings_ratio_pq =
            format!("{}/{}", total_savings_ratio.numer(), total_savings_ratio.denom());
        stats.compression_ratio_pq =
            format!("{}/{}", total_compression_ratio.numer(), total_compression_ratio.denom());
        stats.savings_bps = compute_savings_bps(stats.saved_tokens, stats.input_tokens);
        stats.target_resonance_trit =
            evaluate_bonobo_resonance(stats.input_tokens, stats.output_tokens);
        stats.latency_us_avg = if stats.total_requests == 1 {
            outcome.latency_us
        } else {
            (stats.latency_us_avg + outcome.latency_us).div_ceil(2)
        };

        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();

        let entry = CompressionLogEntry {
            id: stats.total_requests,
            timestamp,
            input_bytes,
            output_bytes: outcome.compressed.len(),
            input_tokens: outcome.original_tokens,
            output_tokens: outcome.compressed_tokens,
            saved_tokens: outcome.saved_tokens,
            ratio_pq: format!(
                "{}/{}",
                outcome.compression_ratio.numer(),
                outcome.compression_ratio.denom()
            ),
            savings_pq: format!(
                "{}/{}",
                outcome.savings_ratio.numer(),
                outcome.savings_ratio.denom()
            ),
            savings_bps: outcome.savings_bps,
            resonance_trit: outcome.resonance_trit,
            latency_us: outcome.latency_us,
            snippet: outcome.compressed.lines().take(2).collect::<Vec<_>>().join(" "),
        };

        drop(stats);

        let mut hist = self
            .history
            .lock()
            .map_err(|_| AppError::StatePoisoned("history mutex poisoned".to_string()))?;
        hist.insert(0, entry.clone());
        if hist.len() > self.config.history_capacity {
            hist.truncate(self.config.history_capacity);
        }
        drop(hist);

        if let Ok(payload) = serde_json::to_string(&entry) {
            let _ = self.tx.send(payload);
        }

        Ok(entry)
    }
}
