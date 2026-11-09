#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."
echo "=== [1/5] cargo fmt --all -- --check ==="
cargo fmt --all -- --check

echo "=== [2/5] cargo clippy --all-targets -- -D warnings ==="
cargo clippy --all-targets -- -D warnings

echo "=== [3/5] cargo test ==="
cargo test

echo "=== [4/5] cargo run --example basic_usage ==="
cargo run --example basic_usage

echo "=== [5/5] cargo bench ==="
cargo bench
