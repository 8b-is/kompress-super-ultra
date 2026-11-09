//! Basic usage example for `kompress-super-ultra`:
//! Runs the 4-role semantic fold engine across ANSI logs, hex dumps, and JSON payloads,
//! then records exact `p/q` rational metrics into `AppState`.

use kompress_super_ultra::{AppConfig, AppState, FoldMode};

fn main() {
    let state = AppState::new(AppConfig::default());
    let sample = "\
\x1B[36m2026-11-09T06:50:00Z INFO [bonobo-r0] upstream token stream 550e8400-e29b-41d4-a716-446655440000\x1B[0m
00000000: 48 65 61 64 72 6f 6f 6d 20 4b 6f 6d 70 72 65 73  Headroom Kompres
  step: verifying SPARK invariants (117/117)
  step: verifying SPARK invariants (117/117)
  step: verifying SPARK invariants (117/117)
{ \"engine\": \"kompress-super-ultra\", \"bonobo_pq\": \"3/2\", \"techno_zen_pq\": \"9/8\" }";

    let outcome = state.engine.fold_with_metrics(sample, FoldMode::Aggressive);
    let entry = state.record_fold(sample.len(), &outcome).expect("record fold");

    println!("=== Kompress-Super-Ultra Basic Usage Example ===");
    println!(
        "Tokens: {} -> {} (saved={}, compression_pq={}, savings_pq={}, savings_bps={} bps, trit={:+})",
        entry.input_tokens,
        entry.output_tokens,
        entry.saved_tokens,
        entry.ratio_pq,
        entry.savings_pq,
        entry.savings_bps,
        entry.resonance_trit
    );
    println!("Folded Output:\n{}", outcome.compressed);
}
