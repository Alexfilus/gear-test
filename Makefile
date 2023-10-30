run:
	cargo run --release
.PHONY: run

test:
	cargo test
.PHONY: test

all_tests:
	cargo test --benches
.PHONY: all_tests

bench:
	cargo bench
.PHONY: bench
