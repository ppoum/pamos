RUST_SRC=$(shell find ./src/ -name "*.rs") Cargo.toml

target/x86_64-unknown-none/debug/pamos: $(RUST_SRC)
	cargo b

target/x86_64-unknown-none/release/pamos: $(RUST_SRC)
	cargo b --release

debug: target/x86_64-unknown-none/debug/pamos

release: target/x86_64-unknown-none/release/pamos
