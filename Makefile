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
	cargo clippy --all-targets --all-features -- -D warnings

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

# Setup git hooks
setup-hooks:
	@echo "Installing git hooks..."
	@mkdir -p .git/hooks
	@cp examples/pre-commit.sh .git/hooks/pre-commit
	@chmod +x .git/hooks/pre-commit
	@echo "Git hooks installed!"

# Run benchmarks
bench:
	cargo build --release
	@echo "Running benchmarks..."
	@command -v hyperfine >/dev/null 2>&1 || { echo "hyperfine not installed. Install with: cargo install hyperfine"; exit 1; }
	hyperfine --warmup 3 './target/release/lineguard src' './target/release/lineguard "src/**/*.rs"'

# Generate coverage report
coverage:
	@command -v cargo-tarpaulin >/dev/null 2>&1 || cargo install cargo-tarpaulin
	cargo tarpaulin --out html --avoid-cfg-tarpaulin
	@echo "Coverage report generated at: target/tarpaulin/tarpaulin-report.html"

# Security audit
audit:
	@command -v cargo-audit >/dev/null 2>&1 || cargo install cargo-audit
	cargo audit

# Update dependencies
update-deps:
	@command -v cargo-edit >/dev/null 2>&1 || cargo install cargo-edit
	cargo upgrade --workspace --incompatible

# Development workflow
dev: fmt test
	@echo "Ready for commit!"

# Show help
help:
	@echo "Available targets:"
	@echo "  make all         - Run checks and tests (default)"
	@echo "  make build       - Build the project"
	@echo "  make test        - Run all tests"
	@echo "  make check       - Run all checks (fmt, clippy, build, test)"
	@echo "  make fmt         - Format code"
	@echo "  make clippy      - Run clippy linter"
	@echo "  make clean       - Clean build artifacts"
	@echo "  make release     - Build release version"
	@echo "  make run         - Run the tool"
	@echo "  make install     - Install locally"
	@echo "  make setup-hooks - Install git hooks"
	@echo "  make bench       - Run benchmarks"
	@echo "  make coverage    - Generate coverage report"
	@echo "  make audit       - Run security audit"
	@echo "  make update-deps - Update dependencies"
	@echo "  make dev         - Development workflow (fmt + test)"
	@echo "  make help        - Show this help message"