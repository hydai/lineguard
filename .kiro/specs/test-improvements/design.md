# Design Document

## Overview

This design addresses the systematic improvement of the test suite by implementing proper output capture and assertion mechanisms, renaming misleading tests, removing unused code, and ensuring all tests provide meaningful verification of functionality. The approach focuses on making tests more reliable and maintainable while preserving existing functionality.

## Architecture

### Test Output Capture Strategy

The design implements a consistent approach to capturing and verifying output across different test types:

1. **Reporter Tests**: Use output capture mechanisms to verify actual output content
2. **Integration Tests**: Leverage existing `assert_cmd` infrastructure for process output verification
3. **Unit Tests**: Implement helper functions for consistent output capture patterns

### Test Organization

Tests are organized into logical groups based on their verification approach:

1. **Behavioral Tests**: Tests that verify actual functionality and output
2. **Error Condition Tests**: Tests that properly simulate and verify error scenarios
3. **Configuration Tests**: Tests with specific assertions about configuration behavior
4. **Integration Tests**: End-to-end tests with proper file and process management

## Components and Interfaces

### Output Capture Utilities

```rust
// Helper functions for consistent output capture
fn capture_stdout<F>(f: F) -> String where F: FnOnce()
fn capture_stderr<F>(f: F) -> String where F: FnOnce()
fn capture_both<F>(f: F) -> (String, String) where F: FnOnce()
```

### Test Data Management

```rust
// Utilities for creating test scenarios
fn create_test_file_with_issues(dir: &Path, name: &str, content: &str) -> PathBuf
fn create_test_stdin_input(files: &[PathBuf]) -> String
fn setup_integration_test_environment() -> TempDir
```

### Reporter Verification

```rust
// Specific verification functions for each reporter type
fn verify_json_output(output: &str, expected_files: usize, expected_issues: usize)
fn verify_github_output(output: &str, expected_annotations: &[&str])
fn verify_human_output(output: &str, expected_patterns: &[&str])
```

## Data Models

### Test Case Structure

```rust
struct TestCase {
    name: String,
    input_files: Vec<TestFile>,
    expected_output: ExpectedOutput,
    expected_exit_code: i32,
}

struct TestFile {
    name: String,
    content: String,
    should_have_issues: bool,
}

enum ExpectedOutput {
    Json { files: usize, issues: usize },
    GitHub { annotations: Vec<String> },
    Human { patterns: Vec<String> },
}
```

### Error Simulation Framework

```rust
enum ErrorScenario {
    FileNotFound,
    PermissionDenied,
    InvalidUtf8,
    DirectoryAsFile,
    ReadError,
}

struct ErrorTest {
    scenario: ErrorScenario,
    setup: Box<dyn Fn(&TempDir) -> PathBuf>,
    verify: Box<dyn Fn(&CheckResult) -> bool>,
}
```

## Error Handling

### Test Failure Reporting

1. **Clear Error Messages**: All test failures include specific information about what was expected vs. actual
2. **Debugging Information**: Failed tests output relevant debugging information (file contents, captured output, etc.)
3. **Isolation**: Test failures in one area don't affect other test categories

### Error Simulation

1. **Realistic Scenarios**: Error tests create actual error conditions rather than mocking
2. **Platform Compatibility**: Error simulation works across different operating systems
3. **Cleanup**: All error simulation tests properly clean up resources

## Testing Strategy

### Phase 1: Reporter Tests Enhancement

1. Implement output capture for all reporter tests
2. Add JSON parsing and validation for JsonReporter tests
3. Verify GitHub Actions annotation format for GitHubReporter tests
4. Check human-readable output patterns for HumanReporter tests

### Phase 2: Test Renaming and Cleanup

1. Rename misleading test functions to reflect actual behavior
2. Remove unused variables and setup code
3. Fix or remove tests that don't test their intended functionality
4. Consolidate duplicate test scenarios

### Phase 3: Integration Test Improvements

1. Fix stdin testing by creating proper temporary files
2. Enhance exit code tests with realistic file scenarios
3. Improve error condition testing with actual error scenarios
4. Add proper cleanup for all integration tests

### Phase 4: Configuration Test Enhancement

1. Replace tautological assertions with specific behavior verification
2. Add comprehensive configuration validation tests
3. Implement proper error condition testing for configuration loading
4. Add edge case testing for configuration parsing

### Phase 5: File Formatting Fixes

1. Add missing newlines to all files
2. Ensure consistent formatting across the codebase
3. Add formatting validation to CI pipeline

## Implementation Details

### Output Capture Implementation

The design uses Rust's standard library capabilities for output capture:

```rust
use std::io::{self, Write};
use std::sync::{Arc, Mutex};

struct OutputCapture {
    stdout: Arc<Mutex<Vec<u8>>>,
    stderr: Arc<Mutex<Vec<u8>>>,
}

impl OutputCapture {
    fn new() -> Self { /* implementation */ }
    fn capture<F>(&self, f: F) -> (String, String) where F: FnOnce() { /* implementation */ }
}
```

### Test File Management

All tests use temporary directories and proper cleanup:

```rust
fn with_test_files<F>(files: &[(&str, &str)], test: F)
where F: FnOnce(&TempDir, &[PathBuf])
{
    let temp_dir = TempDir::new().unwrap();
    let paths: Vec<PathBuf> = files.iter()
        .map(|(name, content)| {
            let path = temp_dir.path().join(name);
            std::fs::write(&path, content).unwrap();
            path
        })
        .collect();
    test(&temp_dir, &paths);
}
```

### Reporter Verification

Each reporter type has specific verification logic:

```rust
fn verify_json_reporter_output(output: &str) -> JsonVerificationResult {
    let parsed: serde_json::Value = serde_json::from_str(output)
        .expect("Output should be valid JSON");

    // Verify structure and content
    JsonVerificationResult {
        is_valid_json: true,
        has_expected_fields: verify_json_fields(&parsed),
        issue_count: count_issues(&parsed),
        file_count: count_files(&parsed),
    }
}
```

## Success Criteria

1. **All Tests Have Meaningful Assertions**: No test should only verify that functions don't panic
2. **Test Names Match Implementation**: All test function names accurately describe what they test
3. **No Unused Code**: All variables and setup code in tests serve a purpose
4. **Proper Error Testing**: Error conditions are actually triggered and verified
5. **Output Verification**: All output-generating functions have their output captured and verified
6. **File Formatting Compliance**: All files follow standard formatting conventions
