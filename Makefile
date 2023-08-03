check:
	cargo check
	cargo fmt --all -- --check
	cargo clippy -- -D warnings