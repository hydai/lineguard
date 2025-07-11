use lineguard::checker::{CheckResult, Issue, IssueType};
use lineguard::reporter::{GitHubReporter, Reporter};
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
