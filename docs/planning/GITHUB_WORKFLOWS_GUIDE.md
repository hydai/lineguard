# GitHub Workflows Usage Guide

## Overview

This guide explains how to use the GitHub Actions workflows configured for LineGuard.

## Workflows

### 1. Continuous Integration (CI)

**File**: `.github/workflows/ci.yml`

**When it runs**:
- Every push to main/master branch
- Every pull request
- Manual trigger via GitHub UI

**What it does**:
- Tests on multiple platforms (Linux, macOS, Windows)
- Tests with multiple Rust versions (stable, beta, nightly)
- Checks code formatting
- Runs clippy linting
- Builds documentation
- Performs security audit
- Generates code coverage report

**Required for merge**: Yes (configure branch protection)

### 2. Release

**File**: `.github/workflows/release.yml`

**When it runs**:
- Push a tag matching `v*.*.*` (e.g., `v1.0.0`)
- Manual trigger with version input

**What it does**:
- Creates GitHub release with changelog
- Builds binaries for multiple platforms
- Uploads binaries and checksums to release
- Publishes to crates.io

**How to create a release**:
```bash
# Tag the release
git tag -a v1.0.0 -m "Release version 1.0.0"
git push origin v1.0.0

# Or trigger manually from GitHub Actions UI
```

### 3. Dependencies

**File**: `.github/workflows/dependencies.yml`

**When it runs**:
- Every Monday at 00:00 UTC
- Manual trigger via GitHub UI

**What it does**:
- Updates all dependencies to latest versions
- Runs tests with updated dependencies
- Creates PR if updates are available
- Performs security audit

### 4. Benchmark

**File**: `.github/workflows/benchmark.yml`

**When it runs**:
- Every push to main/master branch
- Every pull request
- Manual trigger via GitHub UI

**What it does**:
- Runs performance benchmarks
- Checks binary size
- Comments results on PRs
- Tracks performance over time

## Setup Requirements

### 1. Repository Secrets

Add these secrets in your repository settings:

- **`CARGO_REGISTRY_TOKEN`** (Required for releases)
  - Get from: https://crates.io/settings/tokens
  - Used for publishing to crates.io

- **`CODECOV_TOKEN`** (Optional)
  - Get from: https://codecov.io/
  - Used for coverage reporting

### 2. Branch Protection Rules

Configure these rules for your main branch:

1. Go to Settings → Branches
2. Add rule for `main` or `master`
3. Enable:
   - Require status checks to pass
   - Select: `Test`, `Code Quality`, `Security Audit`
   - Require branches to be up to date
   - Include administrators (optional)

### 3. GitHub Pages (for benchmarks)

1. Go to Settings → Pages
2. Source: Deploy from a branch
3. Branch: `gh-pages` / `root`

## Workflow Status Badges

Add these badges to your README.md:

```markdown
[![CI](https://github.com/hydai/lineguard/workflows/CI/badge.svg)](https://github.com/hydai/lineguard/actions/workflows/ci.yml)
[![Release](https://github.com/hydai/lineguard/workflows/Release/badge.svg)](https://github.com/hydai/lineguard/actions/workflows/release.yml)
[![Dependencies](https://github.com/hydai/lineguard/workflows/Dependencies/badge.svg)](https://github.com/hydai/lineguard/actions/workflows/dependencies.yml)
[![codecov](https://codecov.io/gh/hydai/lineguard/branch/main/graph/badge.svg)](https://codecov.io/gh/hydai/lineguard)
```

## Release Process

### Semantic Versioning

LineGuard follows semantic versioning:
- **MAJOR**: Breaking changes
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes (backward compatible)

### Steps to Release

1. **Update version in Cargo.toml**
   ```toml
   version = "1.0.0"
   ```

2. **Update CHANGELOG.md** (optional, auto-generated)

3. **Commit changes**
   ```bash
   git add -A
   git commit -m "chore: prepare release v1.0.0"
   ```

4. **Create and push tag**
   ```bash
   git tag -a v1.0.0 -m "Release version 1.0.0"
   git push origin main
   git push origin v1.0.0
   ```

5. **Monitor release workflow**
   - Check GitHub Actions for progress
   - Verify binaries are uploaded
   - Confirm crates.io publication

### Manual Release

If automatic release fails:

1. Go to Actions → Release workflow
2. Click "Run workflow"
3. Enter version (without 'v' prefix)
4. Click "Run workflow"

## Troubleshooting

### CI Failures

**Format check failed**:
```bash
cargo fmt --all
git add -A
git commit -m "style: format code"
```

**Clippy warnings**:
```bash
cargo clippy --all-targets --all-features --fix
```

**Test failures**:
- Check test output in workflow logs
- Run locally: `cargo test --verbose`
- Platform-specific issues: test on Docker/VM

### Release Failures

**Cross-compilation failed**:
- Check if target is properly supported
- May need to adjust build matrix

**Crates.io publish failed**:
- Verify CARGO_REGISTRY_TOKEN is set
- Check version doesn't already exist
- Ensure all dependencies are published

### Dependency Updates

**PR has conflicts**:
- Manually resolve in the PR
- Or close PR and trigger workflow again

**Tests fail with new dependencies**:
- Review breaking changes in changelog
- Update code as needed
- Pin dependency if necessary

## Best Practices

1. **Keep workflows fast**
   - Use caching effectively
   - Run jobs in parallel
   - Skip unnecessary steps

2. **Security**
   - Pin action versions
   - Minimize token permissions
   - Regular dependency audits

3. **Reliability**
   - Add retry logic for flaky steps
   - Use continue-on-error wisely
   - Monitor workflow execution times

4. **Cost optimization**
   - Use appropriate runner sizes
   - Cancel outdated workflows
   - Archive old artifacts

## Future Enhancements

1. **Container builds**
   - Docker images for each release
   - Multi-arch support

2. **Signed releases**
   - GPG signing for binaries
   - Sigstore/cosign integration

3. **Advanced testing**
   - Fuzzing
   - Property-based testing
   - Integration test suite

4. **Deployment**
   - Homebrew formula updates
   - AUR package updates
   - Snap/Flatpak packages