.PHONY: fromat
format:
	cargo +nightly fmt


.PHONY: lint
lint:
	cargo +nightly fmt --check
	cargo clippy --tests ${ARGS}

