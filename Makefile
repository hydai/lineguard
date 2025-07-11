.PHONY: all build test check fmt clippy clean release help

# Default target
all: check test

# Build the project
build:
	cargo build

# Run all tests
test:
	cargo test

# Run all checks (fmt, clippy, build, test)
check: fmt clippy build test
	@echo "✅ All checks passed!"

# Format code
fmt:
	cargo fmt

# Run clippy
clippy:
	cargo clippy -- -D warnings

# Clean build artifacts
clean:
	cargo clean

# Build release version
release:
	cargo build --release

# Run the tool
run:
	cargo run --

# Install locally
install:
	cargo install --path .

# Uninstall
uninstall:
	cargo uninstall lineguard

# Show help
help:
	@echo "Available targets:"
	@echo "  make all      - Run checks and tests (default)"
	@echo "  make build    - Build the project"
	@echo "  make test     - Run all tests"
	@echo "  make check    - Run all checks (fmt, clippy, build, test)"
	@echo "  make fmt      - Format code"
	@echo "  make clippy   - Run clippy linter"
	@echo "  make clean    - Clean build artifacts"
	@echo "  make release  - Build release version"
	@echo "  make run      - Run the tool"
	@echo "  make install  - Install locally"
	@echo "  make help     - Show this help message"