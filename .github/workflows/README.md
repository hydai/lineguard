# GitHub Actions Workflows

This directory contains the GitHub Actions workflows for LineGuard.

## Workflows

### ci.yml - Continuous Integration
Runs on pushes to `master` and on pull requests that touch `src/**`,
`Cargo.*` or the workflow file itself, plus manual dispatch. Changes limited
to `tests/`, docs or other workflows do not trigger it. Pushes whose commit
message contains "Merge pull request" are skipped to avoid duplicate runs.

**Stages:**
1. **Quick Tests** (Ubuntu, macOS, Windows):
   - `cargo build`
   - Unit tests (`cargo test --lib`)
   - Doc tests (`cargo test --doc`)

2. **Full Test Suite** (Ubuntu only, after Quick Tests):
   - `cargo test --all`
   - `cargo test --all-features`

3. **Code Quality** (Ubuntu, after Quick Tests):
   - `cargo fmt --all -- --check`
   - `cargo clippy --all-targets --all-features -- -D warnings`
   - `cargo doc --no-deps --document-private-items` with `RUSTDOCFLAGS=-D warnings`

4. **Security Audit** (Ubuntu, after Quick Tests):
   - `cargo audit`

### coverage.yml - Code Coverage
Same triggers as CI. Runs `cargo tarpaulin` twice (`--lib` and `--all`) and
uploads both reports to Codecov under the `unit-tests` and `all-tests` flags.
Thresholds and the PR comment layout are configured in `codecov.yml`.

### benchmark.yml - Performance Benchmarks
Same path filters as CI. On pushes to `master` and manual runs it benchmarks
the release binary with hyperfine on generated data sets and stores the
results on the `gh-pages` branch under `dev/bench`. On pull requests the
benchmark job only runs when the PR carries the `benchmark` label; a separate
job comments a binary size report on every PR.

### prepare-release.yml and release.yml - Release Automation
Releases are driven by [knope](https://knope.tech) (see `knope.toml`) from
conventional commits:

1. `prepare-release.yml` runs on every push to `master` except the release
   commit itself. `knope prepare-release` bumps the version in `Cargo.toml`
   and `Cargo.lock`, updates `CHANGELOG.md` and opens or updates a pull
   request from the `release` branch titled `chore: prepare release <version>`.
   Nothing happens when there is no `feat`, `fix` or breaking commit since the
   last release.
2. `release.yml` runs when that pull request is merged. It builds binaries for
   Linux (x86_64 and aarch64, gnu and musl), macOS (x86_64 and aarch64) and
   Windows (x86_64) with SHA256 checksums, publishes the crate to crates.io
   through OIDC trusted publishing (`rust-lang/crates-io-auth-action`, no
   long-lived token, `release` environment) and finally runs `knope release`,
   which creates the GitHub release with the changelog and the artifacts.

To ship a release, merge conventional commits into `master` and then merge the
generated release pull request. Tags are created by knope, not by hand.

### dependencies.yml - Dependency Updates
Automated dependency update checks (existing workflow).

## Test Strategy

The CI is optimized for fast feedback:

1. **Fail Fast**: Quick tests run first and must pass before other jobs start
2. **Parallel Execution**: Quality checks and security audits run in parallel
3. **OS Coverage**: Quick tests run on all platforms, full tests on Linux only
4. **Caching**: Aggressive caching of cargo registry and build artifacts

## Local Testing

To run the same checks locally:

```bash
# Quick tests (what runs first in CI)
cargo test --lib

# Full test suite
cargo test --all

# Quality checks
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --document-private-items

# Coverage (requires cargo-tarpaulin)
cargo tarpaulin --lib --out html
```

## Dependabot

`.github/dependabot.yml` opens weekly pull requests (Monday, 00:00 UTC) for
Cargo dependencies and for the GitHub Actions used by these workflows.
