# ⚡ `kompress-super-ultra` v0.2.0 — Sovereign 4-Role Semantic Fold Engine & Live WebUI

**Headroom + Kompress-Super-Ultra CORE with Precompiled 4-Role Semantic Folding, Exact Rational ($p/q$) & Integer Basis-Point (`bps`) Accounting, Rung 0 Bonobo Resonance ($p/q = 3/2$), Techno-Zen-Bonobo Harmonic ($9/8$), and Real-Time Axum + WebSocket Telemetry (`:8799`).**

---

## 1. Idiomatic Rust Project Structure (`/systems-programming-rust-project`)

```text
kompress-super-ultra/
├── Cargo.toml                  # Crate manifest (v0.2.0, edition 2021, LTO fat, panic = "abort")
├── Makefile                    # Standard dev targets (build, release, test, lint, fmt, run, serve, bench, example)
├── rustfmt.toml                # Rustfmt configuration (edition 2021, max_width 100)
├── clippy.toml                 # Clippy cognitive-complexity threshold (30)
├── src/
│   ├── lib.rs                  # Library root, crate docs, doctests, and unit tests
│   ├── main.rs                 # Async binary entrypoint dispatching CLI subcommands
│   ├── cli.rs                  # Command-line parser (init, run, serve, bench)
│   ├── config.rs               # Runtime AppConfig and FoldMode (Standard, Aggressive, JsonOnly)
│   ├── error.rs                # Structured AppError + Axum IntoResponse implementation
│   ├── commands/
│   │   ├── mod.rs
│   │   ├── init.rs             # Profile initialization command
│   │   └── run.rs              # Offline fold, live Axum server, and throughput benchmark runner
│   ├── models/
│   │   ├── mod.rs
│   │   └── kompress.rs         # Exact p/q rational & integer bps telemetry models
│   ├── services/
│   │   ├── mod.rs
│   │   └── engine.rs           # Precompiled 4-Role Semantic Fold Engine & shared AppState
│   ├── handlers/
│   │   ├── mod.rs
│   │   └── kompress_handler.rs # Axum HTTP + WebSocket (/ws) + Live Dashboard handlers
│   ├── routes/
│   │   ├── mod.rs
│   │   ├── health.rs           # GET /health & GET /openapi.json (OpenAPI 3.1.0)
│   │   └── kompress.rs         # /ws, /api/stats, /api/history, /api/kompress router
│   └── middleware/
│       ├── mod.rs
│       └── telemetry.rs        # Sovereign Bonobo (3/2) & Techno-Zen (9/8) harmonic headers
├── tests/
│   ├── common/
│   │   └── mod.rs              # Shared test fixtures and noisy multi-role log payloads
│   ├── integration_test.rs     # 4-role fold engine, CLI, and exact p/q state tests
│   └── api_tests.rs            # End-to-end TCP HTTP + Axum router integration tests
├── benches/
│   └── benchmark.rs            # Deterministic 10,000-iteration throughput benchmark
├── examples/
│   └── basic_usage.rs          # End-to-end library usage example
└── scripts/
    ├── codeberg_ci_agent.sh    # Dual-forge CI release helper
    └── verify_workspace.sh     # End-to-end fmt, clippy, test, example, and bench verifier
```

---

## 2. Core Invariants & v0.2.0 Highlights

- **Precompiled 4-Role Semantic Fold Engine (`src/services/engine.rs`)**:
  1. **Role 1 — Pruner**: Strips ANSI CSI escape sequences, voluminous hex dumps (`[...hex dump pruned...]`), and (in `Aggressive` mode) ISO-8601 timestamps (`[TS]`) and UUIDv4 identifiers (`[UUID]`).
  2. **Role 2 — Rewriter**: Minifies top-level and line-embedded JSON payloads via `serde_json` and normalizes indentation noise.
  3. **Role 3 — Composer**: Collapses consecutive identical lines and stack frames into `... [repeated N times] ...` markers.
  4. **Role 4 — Circulator**: Computes canonical reduced rationals `Ratio<u64>` (`compression_ratio_pq`, `savings_ratio_pq`), exact integer basis points (`savings_bps`, `0..=10_000`), and Rung 0 Bonobo resonance trits (`+1` when `in / out >= 3 / 2`, `0` when `saved > 0`, `-1` otherwise) with zero floating-point accumulator drift.

---

## 3. Quickstart & Makefile Targets

```bash
make fmt       # cargo fmt --all -- --check
make lint      # cargo clippy --all-targets -- -D warnings
make test      # cargo test (unit + integration + E2E TCP API + doctests)
make example   # cargo run --example basic_usage
make bench     # cargo bench (10,000 4-role folds)
make run       # cargo run -- run --verbose
make serve     # cargo run -- serve --bind 0.0.0.0:8799
```

### Live HTTP / WebSocket Endpoints (`http://127.0.0.1:8799`)
- `GET /` — Techno-Zen-Bonobo Era Live Dashboard (`{<3,<3,<3}+1 · (3/2)×(3/4)=9/8`)
- `GET /health` — Sovereign health status & harmonic invariants (`3/2`, `9/8`, `SPARK 117/117`)
- `GET /openapi.json` — OpenAPI 3.1.0 specification
- `GET /ws` — Real-time WebSocket stream broadcasting `CompressionLogEntry` frames
- `GET /api/stats` — Cumulative `KompressStats` with exact `p/q` and `savings_bps`
- `GET /api/history` — Ring buffer of recent `CompressionLogEntry` items
- `POST /api/kompress` — Fold raw text/logs (`{"text": "...", "mode": "standard|aggressive|json"}`)
