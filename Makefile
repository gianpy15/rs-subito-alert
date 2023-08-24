check:
	cargo check
	cargo fmt --all -- --check
	cargo clippy -- -D warnings

run:
	RUST_LOG=info cargo r