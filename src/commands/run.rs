//! Implementation of the `run`, `serve`, and `bench` subcommands.

use crate::{
    cli::{BenchArgs, RunArgs, ServeArgs},
    config::{AppConfig, FoldMode},
    error::{AppError, Result},
    routes::build_router,
    services::{AdvancedKompressEngine, AppState},
};
use std::{net::SocketAddr, time::Instant};

const DEFAULT_DIAGNOSTIC_SAMPLE: &str = "\
\x1B[32m2026-11-09T06:00:00Z INFO [node-01] Initializing sovereign cluster link 550e8400-e29b-41d4-a716-446655440000\x1B[0m
00000000: 48 65 6c 6c 6f 20 57 6f 72 6c 64 20 38 62 2d 69  Hello World 8b-i
00000010: 73 20 62 6f 6e 6f 62 6f 20 33 2f 32 0a 00 00 00  s bonobo 3/2....
  [trace] polling telemetry heartbeat...
  [trace] polling telemetry heartbeat...
  [trace] polling telemetry heartbeat...
  [trace] polling telemetry heartbeat...
{ \"agent\": \"bonobo\", \"rung\": 0, \"harmonic\": \"3/2\", \"status\": \"active\" }";

/// Execute a single offline 4-role semantic fold pass with exact `p/q` accounting.
pub async fn execute(args: RunArgs) -> Result<()> {
    let raw = args.input.as_deref().unwrap_or(DEFAULT_DIAGNOSTIC_SAMPLE);
    let engine = AdvancedKompressEngine::new();
    let outcome = engine.fold_with_metrics(raw, args.mode);

    if args.verbose {
        println!("=== KOMPRESS-SUPER-ULTRA 4-ROLE SEMANTIC FOLD ===");
        println!("Mode:              {:?}", args.mode);
        println!("Original Tokens:   {}", outcome.original_tokens);
        println!("Compressed Tokens: {}", outcome.compressed_tokens);
        println!("Saved Tokens:      {}", outcome.saved_tokens);
        println!(
            "Compression p/q:   {}/{}",
            outcome.compression_ratio.numer(),
            outcome.compression_ratio.denom()
        );
        println!(
            "Savings p/q:       {}/{} ({} bps)",
            outcome.savings_ratio.numer(),
            outcome.savings_ratio.denom(),
            outcome.savings_bps
        );
        println!("Bonobo Trit:       {:+}", outcome.resonance_trit);
        println!("Latency:           {} us", outcome.latency_us);
        println!("--- Folded Output ---\n{}", outcome.compressed);
    } else {
        println!("{}", outcome.compressed);
    }

    Ok(())
}

/// Start the Axum + Live WebSocket WebUI server on the configured socket address.
pub async fn serve(args: ServeArgs) -> Result<()> {
    let cfg = AppConfig { bind_addr: args.bind.clone(), ..AppConfig::default() };
    let addr: SocketAddr = cfg.bind_addr.parse().map_err(|e| {
        AppError::InvalidInput(format!("invalid bind address '{}': {e}", cfg.bind_addr))
    })?;

    let state = AppState::new(cfg);
    let app = build_router(state);

    println!("🚀 Advanced Kompress-Super-Ultra CORE + Live WS WebUI on http://{addr}");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

/// Execute a deterministic throughput benchmark of the 4-role semantic fold engine.
pub fn execute_bench(args: BenchArgs) -> Result<()> {
    let iterations = args.iterations.max(1);
    let engine = AdvancedKompressEngine::new();
    let start = Instant::now();
    let mut total_saved: u64 = 0;

    for _ in 0..iterations {
        let outcome = engine.fold_with_metrics(DEFAULT_DIAGNOSTIC_SAMPLE, FoldMode::Standard);
        total_saved = total_saved.saturating_add(outcome.saved_tokens);
    }

    let elapsed = start.elapsed();
    let elapsed_ns = elapsed.as_nanos() as u64;
    let ns_per_op = elapsed_ns.div_ceil(iterations);
    let ops_per_sec =
        iterations.saturating_mul(1_000_000_000).checked_div(elapsed_ns).unwrap_or(iterations);

    println!(
        "Benchmarked {iterations} folds in {:.2} ms ({ns_per_op} ns/op, {ops_per_sec} ops/sec, total_saved_tokens={total_saved})",
        elapsed.as_secs_f64() * 1000.0
    );
    Ok(())
}
