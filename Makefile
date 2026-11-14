.PHONY: all
all: check test

.PHONY: check
check:
	cargo clippy -- -D warnings
	cargo fmt --all -- --check

.PHONY: test
test:
	cargo test
