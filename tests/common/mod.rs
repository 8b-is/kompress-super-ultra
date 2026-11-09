//! Shared test fixtures and helpers for `kompress-super-ultra` integration tests.

use kompress_super_ultra::{AppConfig, AppState};

/// Construct an isolated in-memory `AppState` for testing.
pub fn test_state() -> AppState {
    AppState::new(AppConfig {
        bind_addr: "127.0.0.1:0".to_string(),
        history_capacity: 16,
        broadcast_capacity: 32,
        ..AppConfig::default()
    })
}

/// Sample noisy multi-role log payload containing ANSI codes, hex dumps, JSON, and duplicates.
pub const SAMPLE_NOISY_PAYLOAD: &str = "\
\x1B[33m2026-11-09T06:45:00Z WARN [bonobo-gw] upstream retry 550e8400-e29b-41d4-a716-446655440000\x1B[0m
00000000: 7f 45 4c 46 02 01 01 00 00 00 00 00 00 00 00 00  .ELF............
00000010: 02 00 3e 00 01 00 00 00 c0 10 40 00 00 00 00 00  ..>.......@.....
  frame #0: 0x00007ff800123456
  frame #0: 0x00007ff800123456
  frame #0: 0x00007ff800123456
  frame #0: 0x00007ff800123456
{ \"service\": \"kompress-super-ultra\", \"rung\": 0, \"pq\": \"3/2\", \"ok\": true }";
