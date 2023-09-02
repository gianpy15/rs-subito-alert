INSTALL_DIR = $(HOME)/.subito-alert/bin

check:
	cargo check
	cargo fmt --all -- --check
	cargo clippy -- -D warnings

run:
	RUST_LOG=info cargo r

install:
	cargo build --release
	mkdir -p $(INSTALL_DIR)
	cp target/release/rs-subito-alert $(INSTALL_DIR)/subito-alert
	chmod +x $(INSTALL_DIR)/subito-alert
	echo "Remember to add $(INSTALL_DIR) to your PATH variable"

delete-release:
	gh release delete --cleanup-tag -y v$(shell cargo pkgid | cut -d# -f2)
	git tag -d v$(shell cargo pkgid | cut -d# -f2)

deploy:
	git tag v$(shell cargo pkgid | cut -d# -f2)
	git push --tags
