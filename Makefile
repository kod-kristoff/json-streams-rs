.PHONY: check-rustfmt check-clippy run-all-tests

check-rustfmt:
	cargo fmt -- --check

check-clippy:
	cargo clippy

run-all-tests:
	cargo test
