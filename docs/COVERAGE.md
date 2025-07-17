# Code Coverage Report

This document tracks the code coverage progress for LineGuard.

## Current Coverage Status

As of the latest measurement:

### Overall Coverage
- **Total Coverage**: ~85%
- **Unit Test Coverage**: ~82%
- **Integration Test Coverage**: ~88%

### Module Breakdown

#### Checker Module (78.23%)
- `core.rs`: 100% ✅
- `file_checker.rs`: 63.01%
- `mod.rs`: 100% ✅

**Note**: The `file_checker.rs` coverage is limited due to the `check_final_newline_streaming` function using `std::fs::File` directly, which cannot be mocked in unit tests.

#### Reporter Module (91.89%)
- `traits.rs`: 100% ✅
- `human.rs`: 89.06% ✅
- `json.rs`: 91.89% ✅
- `github.rs`: 88.89% ✅

#### Other Modules
- `cli.rs`: ~95%
- `config.rs`: ~90%
- `discovery.rs`: ~85%
- `testing/`: 100% ✅

## Running Coverage Analysis

### Prerequisites

Install cargo-tarpaulin:
```bash
cargo install cargo-tarpaulin
```

### Generate Coverage Reports

#### Quick Summary
```bash
# Unit tests only (fast)
cargo tarpaulin --lib --print-summary

# All tests
cargo tarpaulin --all --print-summary
```

#### Detailed HTML Report
```bash
# Generate HTML report
cargo tarpaulin --lib --out html

# Open in browser
open tarpaulin-report.html  # macOS
xdg-open tarpaulin-report.html  # Linux
```

#### Module-Specific Coverage
```bash
# Check specific module
cargo tarpaulin --lib --print-summary -- checker::

# Exclude tests from coverage
cargo tarpaulin --lib --exclude-tests
```

#### CI Integration
```bash
# Generate Cobertura XML for CI
cargo tarpaulin --lib --out xml

# With line-by-line coverage
cargo tarpaulin --lib --out lcov
```

## Coverage Guidelines

### Minimum Requirements
- New features: 90%+ coverage
- Bug fixes: Must include regression tests
- Core modules: 85%+ coverage
- Utility modules: 80%+ coverage

### What to Test

#### Must Test
- All public API functions
- Error handling paths
- Edge cases and boundaries
- Configuration parsing
- File I/O operations (with mocks)

#### Can Skip
- Simple getter/setter methods
- Derived trait implementations
- Panic branches (unreachable code)
- External tool integration (e.g., terminal color detection)

### Writing Testable Code

#### Use Dependency Injection
```rust
// Bad: Hard to test
fn check_file(path: &Path) -> Result<CheckResult> {
    let content = std::fs::read_to_string(path)?;
    // ...
}

// Good: Testable with mocks
fn check_file<R: FileReader>(path: &Path, reader: &R) -> Result<CheckResult> {
    let content = reader.read_to_string(path)?;
    // ...
}
```

#### Separate I/O from Logic
```rust
// Bad: Mixed concerns
fn process_file(path: &Path) -> Result<()> {
    let content = std::fs::read_to_string(path)?;
    let processed = content.trim();
    std::fs::write(path, processed)?;
    Ok(())
}

// Good: Separated concerns
fn process_content(content: &str) -> String {
    content.trim().to_string()
}

fn process_file<R: FileReader>(path: &Path, reader: &R) -> Result<String> {
    let content = reader.read_to_string(path)?;
    let processed = process_content(&content);
    Ok(processed)
}
```

## Improving Coverage

### Current Gaps

1. **file_checker.rs streaming function**
   - Issue: Direct `std::fs::File` usage
   - Solution: Refactor to use trait-based approach
   - Impact: ~15% improvement potential

2. **Error handling paths**
   - Some error branches in reporters
   - Rare error conditions in CLI

3. **Platform-specific code**
   - Windows-specific path handling
   - Color detection on different terminals

### Action Items

- [ ] Refactor streaming functions to use traits
- [ ] Add more error case tests
- [ ] Mock platform-specific behavior
- [ ] Test concurrent file access scenarios
- [ ] Add fuzzing for parser functions

## Historical Progress

| Date | Total | Checker | Reporter | Notes |
|------|-------|---------|----------|-------|
| Initial | 56% | 45% | 60% | Baseline |
| Phase 1 | 65% | 55% | 70% | Added mocks |
| Phase 2 | 78% | 78% | 77% | DI refactor |
| Current | 85% | 78% | 92% | Added tests |
| Target | 95% | 90% | 95% | Goal |

## Tools and Resources

### Coverage Tools
- **cargo-tarpaulin**: Main coverage tool
- **grcov**: Alternative with more features
- **cargo-llvm-cov**: LLVM-based coverage

### Visualization
- **Codecov**: Cloud-based reporting
- **Coveralls**: Alternative service
- **HTML reports**: Local visualization

### IDE Integration
- VS Code: Coverage Gutters extension
- IntelliJ: Built-in coverage support
- Vim: vim-coverage plugin

## Best Practices

1. **Run coverage locally before pushing**
   ```bash
   cargo tarpaulin --lib --print-summary
   ```

2. **Focus on meaningful coverage**
   - Test behavior, not implementation
   - Cover error paths
   - Test edge cases

3. **Keep tests fast**
   - Use mocks for I/O
   - Minimize test data size
   - Run unit tests separately

4. **Document untestable code**
   ```rust
   // COVERAGE: This function requires real filesystem
   // access and is tested in integration tests
   fn system_specific_operation() {
       // ...
   }
   ```

5. **Regular coverage reviews**
   - Check coverage in PR reviews
   - Address gaps incrementally
   - Maintain coverage over time
