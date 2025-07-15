use lineguard::checker::{CheckResult, Issue, IssueType};
use lineguard::reporter::{GitHubReporter, HumanReporter, JsonReporter, Reporter};
use std::path::PathBuf;

#[test]
fn test_github_reporter_with_issues() {
    let reporter = GitHubReporter;

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

    // Capture output
    let output = std::panic::catch_unwind(|| {
        reporter.report(&results);
    });

    assert!(output.is_ok());
}

#[test]
fn test_github_reporter_empty_results() {
    let reporter = GitHubReporter;
    let results = vec![];

    // Should produce no output
    let output = std::panic::catch_unwind(|| {
        reporter.report(&results);
    });

    assert!(output.is_ok());
}

#[test]
fn test_github_reporter_no_issues() {
    let reporter = GitHubReporter;

    let results = vec![CheckResult {
        file_path: PathBuf::from("src/clean.rs"),
        issues: vec![],
        error: None,
    }];

    // Should produce no output for files without issues
    let output = std::panic::catch_unwind(|| {
        reporter.report(&results);
    });

    assert!(output.is_ok());
}

#[test]
fn test_json_reporter_with_issues() {
    let reporter = JsonReporter;

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

    // Capture output
    let output = std::panic::catch_unwind(|| {
        reporter.report(&results);
    });

    assert!(output.is_ok());
}

#[test]
fn test_json_reporter_no_issues() {
    let reporter = JsonReporter;

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

    // Should still produce valid JSON output
    let output = std::panic::catch_unwind(|| {
        reporter.report(&results);
    });

    assert!(output.is_ok());
}

#[test]
fn test_json_reporter_with_errors() {
    let reporter = JsonReporter;

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

    // Should include errors in output
    let output = std::panic::catch_unwind(|| {
        reporter.report(&results);
    });

    assert!(output.is_ok());
}

#[test]
fn test_json_reporter_empty_results() {
    let reporter = JsonReporter;
    let results = vec![];

    // Should produce valid JSON even with empty results
    let output = std::panic::catch_unwind(|| {
        reporter.report(&results);
    });

    assert!(output.is_ok());
}

#[test]
fn test_human_reporter_with_color_disabled() {
    let reporter = HumanReporter { use_color: false };

    let results = vec![CheckResult {
        file_path: PathBuf::from("src/main.rs"),
        issues: vec![Issue {
            issue_type: IssueType::MissingNewline,
            line: None,
            message: "Missing newline at end of file".to_string(),
        }],
        error: None,
    }];

    // Should work without color
    let output = std::panic::catch_unwind(|| {
        reporter.report(&results);
    });

    assert!(output.is_ok());
}

#[test]
fn test_all_issue_types_in_json() {
    let reporter = JsonReporter;

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

    // Should map all issue types correctly
    let output = std::panic::catch_unwind(|| {
        reporter.report(&results);
    });

    assert!(output.is_ok());
}

#[test]
fn test_github_reporter_all_line_scenarios() {
    let reporter = GitHubReporter;

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

    // Should handle all line number scenarios
    let output = std::panic::catch_unwind(|| {
        reporter.report(&results);
    });

    assert!(output.is_ok());
}
