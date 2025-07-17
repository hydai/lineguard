# GitHub Actions Workflows

This directory contains GitHub Actions workflows for the LineLint project.

## Workflows

### ci.yml - Continuous Integration
The main CI pipeline that runs on every push and pull request.

**Stages:**
1. **Quick Tests** (runs on all OS):
   - Unit tests (`cargo test --lib`)
   - Doc tests
   - Runs in parallel on Ubuntu, macOS, and Windows
   - Typically completes in < 30 seconds

2. **Full Test Suite** (runs on Ubuntu only):
   - All tests including integration tests
   - Runs after quick tests pass
   - Typically completes in < 2 minutes

3. **Code Quality** (runs in parallel with full tests):
   - Format checking (`cargo fmt`)
   - Linting (`cargo clippy`)
   - Documentation build

4. **Security Audit** (runs in parallel with full tests):
   - Dependency vulnerability scanning (`cargo audit`)

### coverage.yml - Code Coverage
Separate workflow for code coverage analysis.

- Runs on push to master and PRs
- Generates coverage for unit tests and all tests separately
- Uploads results to Codecov
- Comments coverage report on PRs

### benchmark.yml - Performance Benchmarks
Tracks performance over time (existing workflow).

### release.yml - Release Automation
Handles releases when tags are pushed (existing workflow).

### dependencies.yml - Dependency Updates
Automated dependency update checks (existing workflow).

## Test Strategy

The CI is optimized for fast feedback:

1. **Fail Fast**: Quick tests run first and must pass before other jobs start
2. **Parallel Execution**: Quality checks and security audits run in parallel
3. **OS Coverage**: Quick tests run on all platforms, full tests on Linux only
4. **Caching**: Aggressive caching of cargo registry and build artifacts

## Local Testing

To run the same test stages locally:

```bash
# Quick tests (what runs first in CI)
cargo test --lib

# Full test suite
cargo test --all

# Quality checks
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings

# Coverage (requires cargo-tarpaulin)
cargo tarpaulin --lib --out html
```
