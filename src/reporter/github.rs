//! GitHub Actions reporter implementation
//!
//! This module provides a reporter that outputs results in GitHub Actions annotation format.

use crate::CheckResult;
use crate::reporter::{Output, Reporter, ReporterWithOutput};
use std::io;

/// GitHub Actions format reporter
///
/// This reporter outputs issues as GitHub Actions error annotations
/// that will be displayed in pull request reviews and workflow runs.
pub struct GitHubReporter;

impl GitHubReporter {
    /// Create a new GitHub reporter
    pub fn new() -> Self {
        Self
    }
}

impl Default for GitHubReporter {
    fn default() -> Self {
        Self::new()
    }
}

impl Reporter for GitHubReporter {
    fn report(&self, results: &[CheckResult]) {
        // Use StdOutput as the default output
        let mut output = crate::reporter::StdOutput::new();

        // Ignore any errors from output operations in the legacy interface
        let _ = self.report_to(results, &mut output);
    }
}

impl ReporterWithOutput for GitHubReporter {
    fn report_to(&self, results: &[CheckResult], output: &mut dyn Output) -> io::Result<()> {
        for result in results {
            // Report errors first
            if let Some(error) = &result.error {
                output.write_line(&format!(
                    "::error file={}::{}",
                    result.file_path.display(),
                    error
                ))?;
            }

            // Report issues
            for issue in &result.issues {
                let file = result.file_path.display();
                match issue.line {
                    Some(line) => {
                        output.write_line(&format!(
                            "::error file={},line={}::{}",
                            file, line, issue.message
                        ))?;
                    },
                    None => {
                        output.write_line(&format!("::error file={}::{}", file, issue.message))?;
                    },
                }
            }
        }

        output.flush()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::mocks::MockOutput;
    use crate::{Issue, IssueType};
    use std::path::PathBuf;

    fn create_test_results() -> Vec<CheckResult> {
        vec![
            CheckResult {
                file_path: PathBuf::from("src/main.rs"),
                issues: vec![
                    Issue {
                        issue_type: IssueType::TrailingSpace,
                        line: Some(42),
                        message: "Trailing spaces found".to_string(),
                    },
                    Issue {
                        issue_type: IssueType::MissingNewline,
                        line: None,
                        message: "Missing newline at end of file".to_string(),
                    },
                ],
                error: None,
            },
            CheckResult {
                file_path: PathBuf::from("src/lib.rs"),
                issues: vec![],
                error: None,
            },
            CheckResult {
                file_path: PathBuf::from("src/error.rs"),
                issues: vec![],
                error: Some("Permission denied".to_string()),
            },
        ]
    }

    #[test]
    fn test_github_reporter_basic() {
        let reporter = GitHubReporter::new();
        let mut output = MockOutput::new();
        let results = create_test_results();

        reporter.report_to(&results, &mut output).unwrap();

        let output_str = output.get_output();

        // Check issues with line numbers
        assert!(output_str.contains("::error file=src/main.rs,line=42::Trailing spaces found"));

        // Check issues without line numbers
        assert!(output_str.contains("::error file=src/main.rs::Missing newline at end of file"));

        // Check error reporting
        assert!(output_str.contains("::error file=src/error.rs::Permission denied"));
    }

    #[test]
    fn test_github_reporter_no_issues() {
        let reporter = GitHubReporter::new();
        let mut output = MockOutput::new();
        let results = vec![
            CheckResult {
                file_path: PathBuf::from("clean1.rs"),
                issues: vec![],
                error: None,
            },
            CheckResult {
                file_path: PathBuf::from("clean2.rs"),
                issues: vec![],
                error: None,
            },
        ];

        reporter.report_to(&results, &mut output).unwrap();

        // Should produce no output for clean files
        assert_eq!(output.get_output(), "");
    }

    #[test]
    fn test_github_reporter_multiple_issues() {
        let reporter = GitHubReporter::new();
        let mut output = MockOutput::new();
        let results = vec![CheckResult {
            file_path: PathBuf::from("test.txt"),
            issues: vec![
                Issue {
                    issue_type: IssueType::TrailingSpace,
                    line: Some(10),
                    message: "Trailing spaces on line 10".to_string(),
                },
                Issue {
                    issue_type: IssueType::TrailingSpace,
                    line: Some(20),
                    message: "Trailing spaces on line 20".to_string(),
                },
                Issue {
                    issue_type: IssueType::MultipleNewlines,
                    line: None,
                    message: "Multiple newlines at end of file".to_string(),
                },
            ],
            error: None,
        }];

        reporter.report_to(&results, &mut output).unwrap();

        let output_str = output.get_output();
        let lines: Vec<&str> = output_str.lines().collect();

        // Should have exactly 3 error annotations
        assert_eq!(lines.len(), 3);
        assert!(lines[0].contains("line=10"));
        assert!(lines[1].contains("line=20"));
        assert!(!lines[2].contains("line="));
    }

    #[test]
    fn test_github_reporter_legacy_interface() {
        let reporter = GitHubReporter::new();
        let results = create_test_results();

        // This should not panic
        reporter.report(&results);
    }

    #[test]
    fn test_github_reporter_empty_results() {
        let reporter = GitHubReporter::new();
        let mut output = MockOutput::new();
        let results = vec![];

        reporter.report_to(&results, &mut output).unwrap();

        assert_eq!(output.get_output(), "");
    }

    #[test]
    fn test_github_reporter_special_characters_in_path() {
        let reporter = GitHubReporter::new();
        let mut output = MockOutput::new();
        let results = vec![CheckResult {
            file_path: PathBuf::from("src/path with spaces/file.rs"),
            issues: vec![Issue {
                issue_type: IssueType::TrailingSpace,
                line: Some(1),
                message: "Issue in file with spaces".to_string(),
            }],
            error: None,
        }];

        reporter.report_to(&results, &mut output).unwrap();

        let output_str = output.get_output();
        assert!(output_str.contains("::error file=src/path with spaces/file.rs,line=1::"));
    }

    #[test]
    fn test_github_reporter_error_only() {
        let reporter = GitHubReporter::new();
        let mut output = MockOutput::new();
        let results = vec![CheckResult {
            file_path: PathBuf::from("unreadable.txt"),
            issues: vec![],
            error: Some("File not found".to_string()),
        }];

        reporter.report_to(&results, &mut output).unwrap();

        let output_str = output.get_output();
        assert_eq!(
            output_str.trim(),
            "::error file=unreadable.txt::File not found"
        );
    }

    #[test]
    fn test_github_reporter_mixed_errors_and_issues() {
        let reporter = GitHubReporter::new();
        let mut output = MockOutput::new();
        let results = vec![CheckResult {
            file_path: PathBuf::from("problematic.txt"),
            issues: vec![Issue {
                issue_type: IssueType::TrailingSpace,
                line: Some(5),
                message: "Trailing space detected".to_string(),
            }],
            error: Some("Partial read error".to_string()),
        }];

        reporter.report_to(&results, &mut output).unwrap();

        let output_str = output.get_output();
        let lines: Vec<&str> = output_str.lines().collect();

        // Should report both error and issue
        assert_eq!(lines.len(), 2);
        assert!(lines[0].contains("::error file=problematic.txt::Partial read error"));
        assert!(lines[1].contains("::error file=problematic.txt,line=5::Trailing space detected"));
    }
}
