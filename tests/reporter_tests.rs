use lineguard::checker::{CheckResult, Issue, IssueType};
// Reporter imports are now handled through test_utils module
use std::path::PathBuf;

mod test_utils;
use test_utils::{
    TestableGitHubReporter, TestableHumanReporter, TestableJsonReporter, TestableReporter,
    verification,
};

#[test]
fn test_github_reporter_with_issues() {
    let reporter = TestableGitHubReporter;

    let results = vec![
        CheckResult {
            file_path: PathBuf::from("src/main.rs"),
            issues: vec![
                Issue {
                    issue_type: IssueType::MissingNewline,
                    line: None,
                    message: "Missing newline at end of file".to_string(),
                },
                Issue {
                    issue_type: IssueType::TrailingSpace,
                    line: Some(45),
                    message: "Trailing spaces found".to_string(),
                },
            ],
            error: None,
        },
        CheckResult {
            file_path: PathBuf::from("tests/test_utils.rs"),
            issues: vec![Issue {
                issue_type: IssueType::TrailingSpace,
                line: Some(23),
                message: "Trailing spaces found".to_string(),
            }],
            error: None,
        },
    ];

    // Capture and verify GitHub Actions annotation output
    let output = reporter.report_to_string(&results);

    // Verify GitHub Actions annotation format
    assert!(
        output.contains("::error"),
        "Should contain GitHub Actions error annotations"
    );

    // Verify specific file paths and line numbers in annotations
    assert!(
        output.contains("::error file=src/main.rs::Missing newline at end of file"),
        "Should contain annotation for missing newline without line number"
    );
    assert!(
        output.contains("::error file=src/main.rs,line=45::Trailing spaces found"),
        "Should contain annotation for trailing space with line number 45"
    );
    assert!(
        output.contains("::error file=tests/test_utils.rs,line=23::Trailing spaces found"),
        "Should contain annotation for trailing space with line number 23"
    );

    // Verify annotation format using verification utility
    let expected_annotations = &[
        "::error file=src/main.rs::Missing newline at end of file",
        "::error file=src/main.rs,line=45::Trailing spaces found",
        "::error file=tests/test_utils.rs,line=23::Trailing spaces found",
    ];
    assert!(
        verification::verify_github_output(&output, expected_annotations),
        "Should contain all expected GitHub Actions annotations"
    );
}

#[test]
fn test_github_reporter_empty_results() {
    let reporter = TestableGitHubReporter;
    let results = vec![];

    // Capture and verify GitHub Actions output with empty results
    let output = reporter.report_to_string(&results);

    // Should produce no output for empty results
    assert!(
        output.is_empty(),
        "Should produce no output for empty results"
    );

    // Verify no annotations are present
    assert!(
        !output.contains("::error"),
        "Should not contain any error annotations"
    );
}

#[test]
fn test_github_reporter_no_issues() {
    let reporter = TestableGitHubReporter;

    let results = vec![CheckResult {
        file_path: PathBuf::from("src/clean.rs"),
        issues: vec![],
        error: None,
    }];

    // Capture and verify GitHub Actions output with no issues
    let output = reporter.report_to_string(&results);

    // Should produce no output for files without issues
    assert!(
        output.is_empty(),
        "Should produce no output for files without issues"
    );

    // Verify no annotations are present
    assert!(
        !output.contains("::error"),
        "Should not contain any error annotations"
    );
    assert!(
        !output.contains("src/clean.rs"),
        "Should not mention files without issues"
    );
}

#[test]
fn test_json_reporter_with_issues() {
    let reporter = TestableJsonReporter;

    let results = vec![
        CheckResult {
            file_path: PathBuf::from("src/main.rs"),
            issues: vec![
                Issue {
                    issue_type: IssueType::MissingNewline,
                    line: None,
                    message: "Missing newline at end of file".to_string(),
                },
                Issue {
                    issue_type: IssueType::TrailingSpace,
                    line: Some(45),
                    message: "Trailing spaces found".to_string(),
                },
            ],
            error: None,
        },
        CheckResult {
            file_path: PathBuf::from("tests/test_utils.rs"),
            issues: vec![Issue {
                issue_type: IssueType::MultipleNewlines,
                line: Some(100),
                message: "Multiple trailing newlines".to_string(),
            }],
            error: None,
        },
    ];

    // Capture and verify JSON output
    let output = reporter.report_to_string(&results);

    // Verify it's valid JSON
    assert!(
        verification::is_valid_json(&output),
        "Output should be valid JSON"
    );

    // Verify structure and counts
    assert!(
        verification::verify_json_output(&output, 2, 3),
        "Should have 2 files and 3 total issues"
    );
    assert_eq!(verification::count_files_in_json(&output), Some(2));
    assert_eq!(verification::count_issues_in_json(&output), Some(3));

    // Verify specific content
    assert!(
        output.contains("src/main.rs"),
        "Should contain first file path"
    );
    assert!(
        output.contains("tests/test_utils.rs"),
        "Should contain second file path"
    );
    assert!(
        output.contains("missing_newline"),
        "Should contain missing_newline issue type"
    );
    assert!(
        output.contains("trailing_space"),
        "Should contain trailing_space issue type"
    );
    assert!(
        output.contains("multiple_newlines"),
        "Should contain multiple_newlines issue type"
    );
    assert!(
        output.contains("\"line\": 45"),
        "Should contain line number 45"
    );
    assert!(
        output.contains("\"line\": 100"),
        "Should contain line number 100"
    );
}

#[test]
fn test_json_reporter_no_issues() {
    let reporter = TestableJsonReporter;

    let results = vec![
        CheckResult {
            file_path: PathBuf::from("src/clean.rs"),
            issues: vec![],
            error: None,
        },
        CheckResult {
            file_path: PathBuf::from("src/perfect.rs"),
            issues: vec![],
            error: None,
        },
    ];

    // Capture and verify JSON output
    let output = reporter.report_to_string(&results);

    // Should still produce valid JSON output
    assert!(
        verification::is_valid_json(&output),
        "Output should be valid JSON"
    );

    // Verify structure and counts - no issues but files were checked
    assert!(
        verification::verify_json_output(&output, 2, 0),
        "Should have 2 files and 0 issues"
    );
    assert_eq!(verification::count_files_in_json(&output), Some(2));
    assert_eq!(verification::count_issues_in_json(&output), Some(0));

    // Verify specific content
    assert!(
        output.contains("\"files_checked\": 2"),
        "Should show 2 files checked"
    );
    assert!(
        output.contains("\"files_with_issues\": 0"),
        "Should show 0 files with issues"
    );
    assert!(
        output.contains("\"total_issues\": 0"),
        "Should show 0 total issues"
    );
}

#[test]
fn test_json_reporter_with_errors() {
    let reporter = TestableJsonReporter;

    let results = vec![
        CheckResult {
            file_path: PathBuf::from("src/readable.rs"),
            issues: vec![Issue {
                issue_type: IssueType::MissingNewline,
                line: None,
                message: "Missing newline at end of file".to_string(),
            }],
            error: None,
        },
        CheckResult {
            file_path: PathBuf::from("src/unreadable.rs"),
            issues: vec![],
            error: Some("Permission denied".to_string()),
        },
    ];

    // Capture and verify JSON output with errors
    let output = reporter.report_to_string(&results);

    // Should include errors in output
    assert!(
        verification::is_valid_json(&output),
        "Output should be valid JSON"
    );

    // Verify structure and counts - 2 files, 1 issue
    assert!(
        verification::verify_json_output(&output, 2, 1),
        "Should have 2 files and 1 issue"
    );
    assert_eq!(verification::count_files_in_json(&output), Some(2));
    assert_eq!(verification::count_issues_in_json(&output), Some(1));

    // Verify specific content
    assert!(
        output.contains("src/readable.rs"),
        "Should contain readable file path"
    );
    assert!(
        output.contains("src/unreadable.rs"),
        "Should contain unreadable file path"
    );
    assert!(
        output.contains("missing_newline"),
        "Should contain issue type"
    );
    assert!(
        output.contains("\"errors\""),
        "Should contain errors section"
    );
    assert!(
        output.contains("Permission denied"),
        "Should contain error message"
    );
}

#[test]
fn test_json_reporter_empty_results() {
    let reporter = TestableJsonReporter;
    let results = vec![];

    // Capture and verify JSON output with empty results
    let output = reporter.report_to_string(&results);

    // Should produce valid JSON even with empty results
    assert!(
        verification::is_valid_json(&output),
        "Output should be valid JSON"
    );

    // Verify structure and counts - no files, no issues
    assert!(
        verification::verify_json_output(&output, 0, 0),
        "Should have 0 files and 0 issues"
    );
    assert_eq!(verification::count_files_in_json(&output), Some(0));
    assert_eq!(verification::count_issues_in_json(&output), Some(0));

    // Verify specific content
    assert!(
        output.contains("\"files_checked\": 0"),
        "Should show 0 files checked"
    );
    assert!(
        output.contains("\"files_with_issues\": 0"),
        "Should show 0 files with issues"
    );
    assert!(
        output.contains("\"total_issues\": 0"),
        "Should show 0 total issues"
    );
    assert!(
        output.contains("\"issues\": []"),
        "Should have empty issues array"
    );
}

#[test]
fn test_human_reporter_with_color_disabled() {
    let reporter = TestableHumanReporter { use_color: false };

    let results = vec![CheckResult {
        file_path: PathBuf::from("src/main.rs"),
        issues: vec![Issue {
            issue_type: IssueType::MissingNewline,
            line: None,
            message: "Missing newline at end of file".to_string(),
        }],
        error: None,
    }];

    // Capture and verify human-readable output without color
    let output = reporter.report_to_string(&results);

    // Should work without color and contain expected text patterns
    assert!(
        !output.is_empty(),
        "Should produce output for files with issues"
    );

    // Verify expected text patterns and formatting
    assert!(
        output.contains("✗ src/main.rs"),
        "Should contain file path with error indicator"
    );
    assert!(
        output.contains("Missing newline at end of file"),
        "Should contain issue message"
    );
    assert!(
        output.contains("Found 1 issues in 1 files"),
        "Should contain summary with issue count"
    );
    assert!(
        output.contains("Files checked: 1"),
        "Should contain files checked count"
    );

    // Verify human-readable output using verification utility
    let expected_patterns = &[
        "✗ src/main.rs",
        "Missing newline at end of file",
        "Found 1 issues in 1 files",
        "Files checked: 1",
    ];
    assert!(
        verification::verify_human_output(&output, expected_patterns),
        "Should contain all expected human-readable patterns"
    );
}

#[test]
fn test_human_reporter_with_color_enabled() {
    let reporter = TestableHumanReporter { use_color: true };

    let results = vec![CheckResult {
        file_path: PathBuf::from("docs/README.md"),
        issues: vec![
            Issue {
                issue_type: IssueType::TrailingSpace,
                line: Some(15),
                message: "Trailing spaces found".to_string(),
            },
            Issue {
                issue_type: IssueType::MultipleNewlines,
                line: Some(42),
                message: "Multiple trailing newlines".to_string(),
            },
        ],
        error: None,
    }];

    // Capture and verify human-readable output with color enabled
    let output = reporter.report_to_string(&results);

    // Should contain expected text patterns and formatting with color enabled
    assert!(
        !output.is_empty(),
        "Should produce output for files with issues"
    );

    // Verify expected text patterns and formatting
    assert!(
        output.contains("✗ docs/README.md"),
        "Should contain file path with error indicator"
    );
    assert!(
        output.contains("Line 15: Trailing spaces found"),
        "Should contain line number and message"
    );
    assert!(
        output.contains("Line 42: Multiple trailing newlines"),
        "Should contain line number and message"
    );
    assert!(
        output.contains("Found 2 issues in 1 files"),
        "Should contain summary with issue count"
    );
    assert!(
        output.contains("Files checked: 1"),
        "Should contain files checked count"
    );

    // Verify human-readable output using verification utility
    let expected_patterns = &[
        "✗ docs/README.md",
        "Line 15: Trailing spaces found",
        "Line 42: Multiple trailing newlines",
        "Found 2 issues in 1 files",
        "Files checked: 1",
    ];
    assert!(
        verification::verify_human_output(&output, expected_patterns),
        "Should contain all expected human-readable patterns with color enabled"
    );
}

#[test]
fn test_human_reporter_no_issues() {
    let reporter = TestableHumanReporter { use_color: false };

    let results = vec![
        CheckResult {
            file_path: PathBuf::from("src/clean.rs"),
            issues: vec![],
            error: None,
        },
        CheckResult {
            file_path: PathBuf::from("src/perfect.rs"),
            issues: vec![],
            error: None,
        },
    ];

    // Capture and verify human-readable output with no issues
    let output = reporter.report_to_string(&results);

    // Should show success message when no issues are found
    assert!(
        !output.is_empty(),
        "Should produce output even when no issues found"
    );

    // Verify expected success patterns
    assert!(
        output.contains("✓ All files passed lint checks!"),
        "Should contain success message"
    );
    assert!(
        output.contains("Files checked: 2"),
        "Should contain files checked count"
    );
    assert!(!output.contains("✗"), "Should not contain error indicators");
    assert!(
        !output.contains("Found"),
        "Should not contain issue count message"
    );

    // Verify human-readable output using verification utility
    let expected_patterns = &["✓ All files passed lint checks!", "Files checked: 2"];
    assert!(
        verification::verify_human_output(&output, expected_patterns),
        "Should contain all expected success patterns"
    );
}

#[test]
fn test_human_reporter_empty_results() {
    let reporter = TestableHumanReporter { use_color: true };
    let results = vec![];

    // Capture and verify human-readable output with empty results
    let output = reporter.report_to_string(&results);

    // Should show success message even with empty results
    assert!(
        !output.is_empty(),
        "Should produce output even with empty results"
    );

    // Verify expected success patterns
    assert!(
        output.contains("✓ All files passed lint checks!"),
        "Should contain success message"
    );
    assert!(
        output.contains("Files checked: 0"),
        "Should show 0 files checked"
    );
    assert!(!output.contains("✗"), "Should not contain error indicators");

    // Verify human-readable output using verification utility
    let expected_patterns = &["✓ All files passed lint checks!", "Files checked: 0"];
    assert!(
        verification::verify_human_output(&output, expected_patterns),
        "Should contain all expected success patterns for empty results"
    );
}

#[test]
fn test_human_reporter_multiple_files_with_issues() {
    let reporter = TestableHumanReporter { use_color: false };

    let results = vec![
        CheckResult {
            file_path: PathBuf::from("src/file1.rs"),
            issues: vec![Issue {
                issue_type: IssueType::MissingNewline,
                line: None,
                message: "Missing newline at end of file".to_string(),
            }],
            error: None,
        },
        CheckResult {
            file_path: PathBuf::from("src/file2.rs"),
            issues: vec![
                Issue {
                    issue_type: IssueType::TrailingSpace,
                    line: Some(10),
                    message: "Trailing spaces found".to_string(),
                },
                Issue {
                    issue_type: IssueType::MultipleNewlines,
                    line: Some(25),
                    message: "Multiple trailing newlines".to_string(),
                },
            ],
            error: None,
        },
        CheckResult {
            file_path: PathBuf::from("src/clean.rs"),
            issues: vec![],
            error: None,
        },
    ];

    // Capture and verify human-readable output with multiple files
    let output = reporter.report_to_string(&results);

    // Should contain information for all files with issues
    assert!(
        !output.is_empty(),
        "Should produce output for files with issues"
    );

    // Verify file paths and issues are present
    assert!(
        output.contains("✗ src/file1.rs"),
        "Should contain first file with issues"
    );
    assert!(
        output.contains("✗ src/file2.rs"),
        "Should contain second file with issues"
    );
    assert!(
        !output.contains("src/clean.rs"),
        "Should not mention clean file"
    );

    // Verify issue details
    assert!(
        output.contains("Missing newline at end of file"),
        "Should contain first file's issue"
    );
    assert!(
        output.contains("Line 10: Trailing spaces found"),
        "Should contain second file's first issue"
    );
    assert!(
        output.contains("Line 25: Multiple trailing newlines"),
        "Should contain second file's second issue"
    );

    // Verify summary
    assert!(
        output.contains("Found 3 issues in 2 files"),
        "Should show correct issue and file counts"
    );
    assert!(
        output.contains("Files checked: 3"),
        "Should show total files checked"
    );

    // Verify human-readable output using verification utility
    let expected_patterns = &[
        "✗ src/file1.rs",
        "✗ src/file2.rs",
        "Missing newline at end of file",
        "Line 10: Trailing spaces found",
        "Line 25: Multiple trailing newlines",
        "Found 3 issues in 2 files",
        "Files checked: 3",
    ];
    assert!(
        verification::verify_human_output(&output, expected_patterns),
        "Should contain all expected patterns for multiple files with issues"
    );
}

#[test]
fn test_all_issue_types_in_json() {
    let reporter = TestableJsonReporter;

    let results = vec![CheckResult {
        file_path: PathBuf::from("test.txt"),
        issues: vec![
            Issue {
                issue_type: IssueType::MissingNewline,
                line: None,
                message: "Missing newline".to_string(),
            },
            Issue {
                issue_type: IssueType::MultipleNewlines,
                line: Some(50),
                message: "Multiple newlines".to_string(),
            },
            Issue {
                issue_type: IssueType::TrailingSpace,
                line: Some(25),
                message: "Trailing space".to_string(),
            },
        ],
        error: None,
    }];

    // Capture and verify JSON output with all issue types
    let output = reporter.report_to_string(&results);

    // Should map all issue types correctly
    assert!(
        verification::is_valid_json(&output),
        "Output should be valid JSON"
    );

    // Verify structure and counts - 1 file, 3 issues
    assert!(
        verification::verify_json_output(&output, 1, 3),
        "Should have 1 file and 3 issues"
    );
    assert_eq!(verification::count_files_in_json(&output), Some(1));
    assert_eq!(verification::count_issues_in_json(&output), Some(3));

    // Verify all issue types are present
    assert!(output.contains("test.txt"), "Should contain file path");
    assert!(
        output.contains("missing_newline"),
        "Should contain missing_newline issue type"
    );
    assert!(
        output.contains("multiple_newlines"),
        "Should contain multiple_newlines issue type"
    );
    assert!(
        output.contains("trailing_space"),
        "Should contain trailing_space issue type"
    );

    // Verify line numbers are correctly mapped
    assert!(
        output.contains("\"line\": null"),
        "Should contain null line for missing_newline"
    );
    assert!(
        output.contains("\"line\": 50"),
        "Should contain line number 50"
    );
    assert!(
        output.contains("\"line\": 25"),
        "Should contain line number 25"
    );

    // Verify messages are included
    assert!(
        output.contains("Missing newline"),
        "Should contain missing newline message"
    );
    assert!(
        output.contains("Multiple newlines"),
        "Should contain multiple newlines message"
    );
    assert!(
        output.contains("Trailing space"),
        "Should contain trailing space message"
    );
}

#[test]
fn test_github_reporter_all_line_scenarios() {
    let reporter = TestableGitHubReporter;

    let results = vec![CheckResult {
        file_path: PathBuf::from("test.rs"),
        issues: vec![
            Issue {
                issue_type: IssueType::MissingNewline,
                line: None,
                message: "No line number".to_string(),
            },
            Issue {
                issue_type: IssueType::TrailingSpace,
                line: Some(1),
                message: "Line 1".to_string(),
            },
            Issue {
                issue_type: IssueType::TrailingSpace,
                line: Some(999),
                message: "Line 999".to_string(),
            },
        ],
        error: None,
    }];

    // Capture and verify GitHub Actions output with all line number scenarios
    let output = reporter.report_to_string(&results);

    // Should handle all line number scenarios
    assert!(
        output.contains("::error"),
        "Should contain GitHub Actions error annotations"
    );

    // Verify specific line number handling
    assert!(
        output.contains("::error file=test.rs::No line number"),
        "Should contain annotation without line number"
    );
    assert!(
        output.contains("::error file=test.rs,line=1::Line 1"),
        "Should contain annotation with line number 1"
    );
    assert!(
        output.contains("::error file=test.rs,line=999::Line 999"),
        "Should contain annotation with line number 999"
    );

    // Verify annotation format using verification utility
    let expected_annotations = &[
        "::error file=test.rs::No line number",
        "::error file=test.rs,line=1::Line 1",
        "::error file=test.rs,line=999::Line 999",
    ];
    assert!(
        verification::verify_github_output(&output, expected_annotations),
        "Should contain all expected GitHub Actions annotations with correct line number handling"
    );
}
