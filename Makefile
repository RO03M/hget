.PHONY: build build-release build-gui dev-gui dev-cli test check fmt clippy clean install

# Capture extra words after "dev-cli" as CLI args
ifeq (dev-cli,$(firstword $(MAKECMDGOALS)))
  _CLI_ARGS := $(wordlist 2,$(words $(MAKECMDGOALS)),$(MAKECMDGOALS))
  $(eval $(_CLI_ARGS):;@:)
endif

# Default: build CLI in debug mode
build:
	cargo build -p hget

build-release:
	cargo build -p hget --release

dev-cli:
	cargo run -p hget -- $(_CLI_ARGS)

build-gui:
	cd crates/gui && yarn tauri build

dev-gui:
	cd crates/gui && yarn tauri dev

install-gui-deps:
	cd crates/gui && yarn install

test:
	cargo test --workspace

check:
	cargo check --workspace

fmt:
	cargo fmt --all

clippy:
	cargo clippy --workspace -- -D warnings

clean:
	cargo clean

install: build-release
	cp target/release/hget ~/.local/bin/hget
