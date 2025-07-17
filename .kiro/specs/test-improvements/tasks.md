# Implementation Plan

- [x] 1. Create output capture utilities for test infrastructure
  - Implement helper functions for capturing stdout and stderr in tests
  - Create utilities for consistent output verification patterns
  - Add test data management functions for creating test scenarios
  - _Requirements: 1.1, 1.2_

- [x] 2. Fix reporter tests to capture and verify actual output
- [x] 2.1 Enhance JsonReporter tests with output capture and JSON validation
  - Modify all JsonReporter tests to capture stdout output
  - Parse captured JSON output and verify structure and content
  - Assert specific field values and issue counts in JSON output
  - _Requirements: 1.2, 1.3_

- [x] 2.2 Enhance GitHubReporter tests with output capture and annotation verification
  - Modify all GitHubReporter tests to capture stdout output
  - Verify GitHub Actions annotation format in captured output
  - Assert correct file paths and line numbers in annotations
  - _Requirements: 1.2, 1.4_

- [x] 2.3 Enhance HumanReporter tests with output capture and content verification
  - Modify all HumanReporter tests to capture stdout output
  - Verify expected text patterns and formatting in human-readable output
  - Assert color formatting behavior when enabled/disabled
  - _Requirements: 1.2, 1.5_

- [x] 3. Fix main.rs tests to capture and verify report_fix_results output
  - Modify report_fix_results tests to capture stdout and stderr
  - Assert specific output messages for different scenarios (quiet mode, dry run, errors)
  - Verify that quiet mode produces no output and other modes produce expected messages
  - _Requirements: 1.1, 1.5_

- [x] 4. Rename and fix misleading test functions
- [x] 4.1 Fix test_discover_files_from_stdin to actually test stdin or rename appropriately
  - Either implement proper stdin testing with temporary files or rename to test_discover_files_from_args
  - Remove unused _stdin_content variable and misleading comments
  - Ensure test name matches actual functionality being tested
  - _Requirements: 2.1, 3.1, 3.2_

- [x] 4.2 Fix test_check_large_file_with_errors to match its name or rename it
  - Rename to test_check_non_existent_file to reflect actual behavior
  - Remove unused large_file creation code that doesn't contribute to the test
  - Ensure test focuses on the specific error condition being tested
  - _Requirements: 2.2, 3.1, 3.3_

- [x] 4.3 Fix test_check_streaming_seek_error to properly test seek errors or rename it
  - Either implement actual seek error testing or rename to reflect actual behavior
  - Add proper assertions for the expected issue detection (MissingNewline)
  - Remove misleading comments about seek errors if not actually testing them
  - _Requirements: 2.3, 3.2, 1.1_

- [x] 4.4 Remove or fix test_check_streaming_read_error as it duplicates other tests
  - Remove this test as it duplicates test_check_non_existent_file functionality
  - Remove unused large file creation code
  - Consolidate error testing to avoid redundant test cases
  - _Requirements: 2.4, 3.1, 3.3_

- [x] 4.5 Remove test_discover_files_directory_entry_error as it doesn't test error conditions
  - Remove this test since it doesn't actually test directory entry errors
  - The test performs normal file discovery and asserts success, providing no value
  - Focus error testing on tests that actually create error conditions
  - _Requirements: 2.5, 3.3_

- [x] 5. Enhance integration tests with proper file handling and realistic scenarios
- [x] 5.1 Fix test_stdin_mode_exit_codes to use actual temporary files
  - Create temporary files and pass their full paths to stdin
  - Ensure the test verifies actual file processing rather than just non-existent file handling
  - Verify that stdin mode properly processes the provided file paths
  - _Requirements: 4.1, 4.3_

- [x] 5.2 Enhance exit code tests with more realistic file scenarios
  - Ensure all exit code tests use files that actually exist in the test environment
  - Verify that exit codes match the documented behavior for each scenario
  - Add proper cleanup for all temporary files and directories
  - _Requirements: 4.2, 4.4_

- [x] 6. Fix configuration tests with meaningful assertions
- [x] 6.1 Replace tautological assertion in test_unknown_config_fields
  - Change assert!(config.is_ok() || config.is_err()) to assert!(config.is_ok())
  - Add comment explaining that unknown fields should be ignored by serde
  - Verify that the configuration loads successfully with default values
  - _Requirements: 5.1, 5.2_

- [x] 6.2 Enhance configuration error tests with specific behavior verification
  - Add specific assertions for each configuration error scenario
  - Verify error messages contain expected information
  - Test edge cases with meaningful assertions rather than generic success/failure checks
  - _Requirements: 5.3, 5.4_

- [x] 7. Fix file formatting issues
- [x] 7.1 Add missing newline to CLAUDE.md
  - Add a newline character at the end of CLAUDE.md file
  - Ensure the file follows standard formatting conventions
  - Verify that the change doesn't affect the content meaning
  - _Requirements: 6.1, 6.2_

- [x] 7.2 Run formatting checks on all modified files
  - Execute cargo fmt on all modified Rust files
  - Run cargo clippy to ensure no new warnings are introduced
  - Verify that all files pass the project's formatting standards
  - _Requirements: 6.3_
