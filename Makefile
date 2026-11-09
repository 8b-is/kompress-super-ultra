.PHONY: build release test lint fmt run serve bench example clean

build:
	cargo build

release:
	cargo build --release

test:
	cargo test

lint:
	cargo clippy --all-targets -- -D warnings

fmt:
	cargo fmt --all -- --check

run:
	cargo run -- run --verbose

serve:
	cargo run -- serve --bind 0.0.0.0:8799

bench:
	cargo bench

example:
	cargo run --example basic_usage

clean:
	cargo clean
