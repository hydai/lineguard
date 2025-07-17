# Contributing to LineGuard

Thank you for your interest in contributing to LineGuard! This document provides guidelines and instructions for contributing to the project.

## Development Process

LineGuard follows strict Test-Driven Development (TDD) practices. Every feature must be developed following this cycle:

1. **Write Test First** - Create minimal failing test
2. **Make Test Pass** - Write minimal code to pass
3. **Commit** - Commit working code
4. **Refactor** - Improve code quality
5. **Quality Checks** - Run all checks before final commit

## Getting Started

### Prerequisites

- Rust 1.70+ (install via [rustup](https://rustup.rs/))
- Git
- cargo-tarpaulin (for coverage): `cargo install cargo-tarpaulin`

### Setup

```bash
# Clone the repository
git clone https://github.com/hydai/lineguard
cd lineguard

# Build the project
cargo build

# Run tests
cargo test

# Run quality checks
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
```

## Testing Guidelines

### Test Structure

```
src/
├── checker/
│   ├── mod.rs          # Public API
│   ├── core.rs         # Core logic with tests
│   ├── file_checker.rs # File operations with tests
│   └── tests.rs        # Additional unit tests
├── reporter/
│   ├── mod.rs          # Public API
│   ├── human.rs        # Human reporter with tests
│   ├── json.rs         # JSON reporter with tests
│   └── github.rs       # GitHub reporter with tests
└── testing/            # Test infrastructure
    ├── mocks/          # Mock implementations
    └── builders/       # Test data builders

tests/                  # Integration tests
├── cli_tests.rs
├── fix_tests.rs
└── ...
```

### Writing Tests

#### Unit Tests

Place unit tests in the same file as the code being tested:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_behavior() {
        // Arrange
        let input = "test";

        // Act
        let result = function_under_test(input);

        // Assert
        assert_eq!(result, expected);
    }
}
```

#### Integration Tests

Place integration tests in the `tests/` directory:

```rust
use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_cli_behavior() {
    let mut cmd = Command::cargo_bin("lineguard").unwrap();
    cmd.arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("LineGuard"));
}
```

### Using Mock Implementations

LineGuard provides mock implementations for testing without real I/O:

#### MockFileSystem

```rust
use lineguard::testing::mocks::{MockFileSystem, file_metadata};

#[test]
fn test_file_operations() {
    let mut fs = MockFileSystem::new();

    // Add a file with content (metadata is set automatically)
    fs.add_file("test.txt", "Hello\n");

    // Optionally set custom metadata
    fs.set_metadata("test.txt", file_metadata(6));

    // Simulate errors
    fs.add_error("error.txt", std::io::Error::new(
        std::io::ErrorKind::PermissionDenied,
        "Access denied"
    ));

    // Use in tests
    let content = fs.read_to_string(Path::new("test.txt")).unwrap();
    assert_eq!(content, "Hello\n");
}
```

#### MockOutput

```rust
#[test]
fn test_output_operations() {
    use lineguard::reporter::Color;
    use lineguard::testing::mocks::MockOutput;

    let mut output = MockOutput::new();

    // Write content
    output.write_line("Hello").unwrap();

    // Check buffer
    assert_eq!(output.get_output(), "Hello\n");

    // Test colored output
    output.write_colored("Error", Color::Red).unwrap();
    assert!(output.contains_colored("Error", Color::Red));
}
```

#### Test Builders

```rust
use lineguard::testing::builders::{TestFileBuilder, TestScenario};

#[test]
fn test_with_builders() {
    // Build test file content
    let (path, content) = TestFileBuilder::new("test.txt")
        .with_line("First line")
        .with_trailing_spaces()
        .without_final_newline()
        .build();

    // Build test scenario
    let scenario = TestScenario::new()
        .with_file("src/main.rs", "valid content\n")
        .with_file("src/bad.rs", "trailing spaces  \n")
        .build();
}
```

### Coverage Requirements

- Overall project: 90%+
- Core modules (checker, reporter): 85%+
- New features: Must include tests
- Bug fixes: Must include regression tests

Check coverage:

```bash
# Unit test coverage
cargo tarpaulin --lib --print-summary

# All test coverage
cargo tarpaulin --all --print-summary

# Generate HTML report
cargo tarpaulin --lib --out html
# Open tarpaulin-report.html in browser
```

## Code Style

### Formatting

- Use `cargo fmt` for consistent formatting
- Run before every commit: `cargo fmt --all`

### Linting

- Fix all clippy warnings: `cargo clippy --all-targets --all-features -- -D warnings`
- Common issues:
  - Use `&str` instead of `&String`
  - Use `&[T]` instead of `&Vec<T>`
  - Avoid unnecessary clones

### Naming Conventions

- Functions: `snake_case`
- Types: `PascalCase`
- Constants: `SCREAMING_SNAKE_CASE`
- Modules: `snake_case`

### Documentation

- Add doc comments to all public items
- Include examples in doc comments
- Run `cargo doc --open` to preview

```rust
/// Checks a file for line ending issues.
///
/// # Arguments
///
/// * `path` - Path to the file to check
///
/// # Examples
///
/// ```
/// use lineguard::check_file;
///
/// let result = check_file(&PathBuf::from("test.txt"));
/// assert!(result.issues.is_empty());
/// ```
pub fn check_file(path: &Path) -> CheckResult {
    // Implementation
}
```

## Making Changes

### 1. Create a Branch

```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/issue-description
```

### 2. Follow TDD Cycle

```bash
# 1. Write failing test
# Edit test file...

# 2. Run test (should fail)
cargo test test_new_feature

# 3. Implement feature
# Edit source file...

# 4. Run test (should pass)
cargo test test_new_feature

# 5. Run all checks
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

### 3. Commit Changes

```bash
# Stage changes
git add -A

# Commit with descriptive message
git commit -m "feat: add support for custom ignore patterns"
```

Commit message format:
- `feat:` New feature
- `fix:` Bug fix
- `docs:` Documentation changes
- `test:` Test changes
- `refactor:` Code refactoring
- `chore:` Maintenance tasks

### 4. Push and Create PR

```bash
git push origin feature/your-feature-name
```

Then create a Pull Request on GitHub.

## CI/CD Pipeline

All PRs must pass:

1. **Quick Tests** - Unit tests on all platforms
2. **Full Tests** - All tests including integration
3. **Code Quality** - Formatting and linting
4. **Security Audit** - Dependency scanning
5. **Coverage** - Must maintain 90%+ coverage

## Performance Considerations

- Use streaming for large files (>10MB)
- Leverage parallel processing with rayon
- Avoid unnecessary allocations
- Profile with `cargo flamegraph` for bottlenecks

## Need Help?

- Check existing issues and PRs
- Read the [implementation plan](docs/planning/IMPLEMENTATION_PLAN.md)
- Ask questions in issues
- Review test examples in the codebase

## License

By contributing, you agree that your contributions will be licensed under the Apache License 2.0.
