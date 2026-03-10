BINARY        := dev-container
INSTALL_DIR   := $(HOME)/.local/bin
CARGO         := cargo
TARGET_RELEASE := target/release/$(BINARY)
TARGET_DEV    := target/debug/$(BINARY)

.PHONY: all build build-dev build-static test test-verbose test-docker \
        install uninstall fmt lint check clean help

all: build

build:
	$(CARGO) build --release

build-dev:
	$(CARGO) build

build-static:
	RUSTFLAGS="-C target-feature=+crt-static" \
	$(CARGO) build --release --target x86_64-unknown-linux-musl

test: build
	$(CARGO) test

test-verbose: build
	$(CARGO) test -- --nocapture

test-docker: build
	$(CARGO) test -- --include-ignored

check:
	$(CARGO) check

fmt:
	$(CARGO) fmt

lint:
	$(CARGO) clippy -- -D warnings

install: build
	install -Dm755 $(TARGET_RELEASE) $(INSTALL_DIR)/$(BINARY)
	@echo "Installed to $(INSTALL_DIR)/$(BINARY)"

uninstall:
	rm -f $(INSTALL_DIR)/$(BINARY)
	@echo "Removed $(INSTALL_DIR)/$(BINARY)"

clean:
	$(CARGO) clean

help:
	@echo "Targets:"
	@echo "  build          Build release binary"
	@echo "  build-dev      Build debug binary"
	@echo "  build-static   Build fully static binary (requires musl target)"
	@echo "  test           Run integration tests (no Docker required)"
	@echo "  test-verbose   Run integration tests with stdout output"
	@echo "  test-docker    Run all tests including Docker-dependent ones"
	@echo "  check          Run cargo check"
	@echo "  fmt            Format code with cargo fmt"
	@echo "  lint           Lint with cargo clippy"
	@echo "  install        Install binary to $(INSTALL_DIR)"
	@echo "  uninstall      Remove binary from $(INSTALL_DIR)"
	@echo "  clean          Remove build artifacts"
