PLATFORM = $(shell uname)

ifeq ($(PLATFORM), Darwin)
	RUST_STDLIB_SRC_DIR ?= ~/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/src
endif

ifeq ($(PLATFORM), Linux)
	RUST_STDLIB_SRC_DIR ?= ~/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu//lib/rustlib/src/rust/src
endif

VERACRUZ_TARGET_PATH = $(shell cd ../../ && pwd)

all:
	XARGO_RUST_SRC=$(RUST_STDLIB_SRC_DIR) RUST_TARGET_PATH=$(VERACRUZ_TARGET_PATH) xargo build --target wasm32-arm-veracruz --release

clean:
	cargo clean
	xargo clean
	rm Cargo.lock
