.PHONY: debug-run-ctl
debug-run-ctl:
	./target/debug/ctl

.PHONY: run
run:
	cargo run

.PHONY: build
build:
	cargo build

.PHONY: clippy
clippy:
	cargo clippy

.PHONY: fmt
fmt:
	cargo fmt

.PHONY: clean
clean:
	cargo clean

