.PHONY: fromat
format:
	cargo +nightly fmt


.PHONY: lint
lint:
	cargo +nightly fmt --check
	cargo clippy --tests ${ARGS}

.PHONY: run
run:
	cargo run --package rust-sprite-game --bin rust-sprite-game ${ARGS}

.PHONY: run-debug
run-debug:
	cargo run --package rust-sprite-game --bin rust-sprite-game --features debug-bbox

.PHONY: test
test:
	cargo test --workspace
