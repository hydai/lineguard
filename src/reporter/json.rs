//! JSON reporter implementation
//!
//! This module provides a reporter that outputs results in JSON format.

use crate::reporter::{Output, Reporter, ReporterWithOutput};
use crate::{CheckResult, IssueType};
use serde_json::json;
use std::io;

/// JSON format reporter
pub struct JsonReporter {
    /// Whether to pretty-print the JSON output
    pub pretty: bool,
}

impl JsonReporter {
    /// Create a new JSON reporter with pretty printing
    pub fn new() -> Self {
        Self { pretty: true }
    }

    /// Create a new JSON reporter without pretty printing
    pub fn compact() -> Self {
        Self { pretty: false }
    }
}

impl Default for JsonReporter {
    fn default() -> Self {
        Self::new()
    }
}

impl Reporter for JsonReporter {
    fn report(&self, results: &[CheckResult]) {
        // Use StdOutput as the default output
        let mut output = crate::reporter::StdOutput::new();

        // Ignore any errors from output operations in the legacy interface
        let _ = self.report_to(results, &mut output);
    }
}

impl ReporterWithOutput for JsonReporter {
    fn report_to(&self, results: &[CheckResult], output: &mut dyn Output) -> io::Result<()> {
        let files_checked = results.len();
        let files_with_issues = results.iter().filter(|r| !r.issues.is_empty()).count();
        let total_issues: usize = results.iter().map(|r| r.issues.len()).sum();

        let mut issues = Vec::new();
        let mut errors = Vec::new();

        for result in results {
            // Collect errors
            if let Some(error) = &result.error {
                errors.push(json!({
                    "file": result.file_path.display().to_string(),
                    "error": error,
                }));
            }

            if !result.issues.is_empty() {
                let file_issues: Vec<_> = result
                    .issues
                    .iter()
                    .map(|issue| {
                        json!({
                            "type": match issue.issue_type {
                                IssueType::MissingNewline => "missing_newline",
                                IssueType::MultipleNewlines => "multiple_newlines",
                                IssueType::TrailingSpace => "trailing_space",
                            },
                            "line": issue.line,
                            "message": issue.message,
                        })
                    })
                    .collect();

                issues.push(json!({
                    "file": result.file_path.display().to_string(),
                    "issues": file_issues,
                }));
            }
        }

        let mut json_output = json!({
            "files_checked": files_checked,
            "files_with_issues": files_with_issues,
            "total_issues": total_issues,
            "issues": issues,
        });

        if !errors.is_empty() {
            json_output["errors"] = json!(errors);
        }

        // Format and write the JSON
        let json_string = if self.pretty {
            serde_json::to_string_pretty(&json_output)?
        } else {
            serde_json::to_string(&json_output)?
        };

        output.write_line(&json_string)?;
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
                file_path: PathBuf::from("test1.txt"),
                issues: vec![
                    Issue {
                        issue_type: IssueType::TrailingSpace,
                        line: Some(5),
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
                file_path: PathBuf::from("test2.txt"),
                issues: vec![],
                error: None,
            },
            CheckResult {
                file_path: PathBuf::from("test3.txt"),
                issues: vec![],
                error: Some("Permission denied".to_string()),
            },
        ]
    }

    #[test]
    fn test_json_reporter_basic() {
        let reporter = JsonReporter::new();
        let mut output = MockOutput::new();
        let results = create_test_results();

        reporter.report_to(&results, &mut output).unwrap();

        let json_str = output.get_output();
        let json: serde_json::Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json["files_checked"], 3);
        assert_eq!(json["files_with_issues"], 1);
        assert_eq!(json["total_issues"], 2);
        assert_eq!(json["issues"].as_array().unwrap().len(), 1);
        assert_eq!(json["errors"].as_array().unwrap().len(), 1);
    }

    #[test]
    fn test_json_reporter_no_issues() {
        let reporter = JsonReporter::new();
        let mut output = MockOutput::new();
        let results = vec![
            CheckResult {
                file_path: PathBuf::from("clean1.txt"),
                issues: vec![],
                error: None,
            },
            CheckResult {
                file_path: PathBuf::from("clean2.txt"),
                issues: vec![],
                error: None,
            },
        ];

        reporter.report_to(&results, &mut output).unwrap();

        let json_str = output.get_output();
        let json: serde_json::Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json["files_checked"], 2);
        assert_eq!(json["files_with_issues"], 0);
        assert_eq!(json["total_issues"], 0);
        assert_eq!(json["issues"].as_array().unwrap().len(), 0);
        assert!(json.get("errors").is_none());
    }

    #[test]
    fn test_json_reporter_compact() {
        let reporter = JsonReporter::compact();
        let mut output = MockOutput::new();
        let results = vec![CheckResult {
            file_path: PathBuf::from("test.txt"),
            issues: vec![Issue {
                issue_type: IssueType::TrailingSpace,
                line: Some(1),
                message: "Trailing spaces".to_string(),
            }],
            error: None,
        }];

        reporter.report_to(&results, &mut output).unwrap();

        let json_str = output.get_output();
        // Compact JSON should not contain newlines within the JSON structure
        assert_eq!(json_str.matches('\n').count(), 1); // Only the final newline
    }

    #[test]
    fn test_json_reporter_issue_types() {
        let reporter = JsonReporter::new();
        let mut output = MockOutput::new();
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
                    line: None,
                    message: "Multiple newlines".to_string(),
                },
                Issue {
                    issue_type: IssueType::TrailingSpace,
                    line: Some(10),
                    message: "Trailing space".to_string(),
                },
            ],
            error: None,
        }];

        reporter.report_to(&results, &mut output).unwrap();

        let json_str = output.get_output();
        let json: serde_json::Value = serde_json::from_str(&json_str).unwrap();

        let file_issues = &json["issues"][0]["issues"];
        assert_eq!(file_issues[0]["type"], "missing_newline");
        assert_eq!(file_issues[1]["type"], "multiple_newlines");
        assert_eq!(file_issues[2]["type"], "trailing_space");
        assert_eq!(file_issues[2]["line"], 10);
    }

    #[test]
    fn test_json_reporter_legacy_interface() {
        let reporter = JsonReporter::new();
        let results = create_test_results();

        // This should not panic
        reporter.report(&results);
    }

    #[test]
    fn test_json_reporter_empty_results() {
        let reporter = JsonReporter::new();
        let mut output = MockOutput::new();
        let results = vec![];

        reporter.report_to(&results, &mut output).unwrap();

        let json_str = output.get_output();
        let json: serde_json::Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json["files_checked"], 0);
        assert_eq!(json["files_with_issues"], 0);
        assert_eq!(json["total_issues"], 0);
        assert_eq!(json["issues"].as_array().unwrap().len(), 0);
    }

    #[test]
    fn test_json_reporter_with_errors_only() {
        let reporter = JsonReporter::new();
        let mut output = MockOutput::new();
        let results = vec![
            CheckResult {
                file_path: PathBuf::from("error1.txt"),
                issues: vec![],
                error: Some("File not found".to_string()),
            },
            CheckResult {
                file_path: PathBuf::from("error2.txt"),
                issues: vec![],
                error: Some("Permission denied".to_string()),
            },
        ];

        reporter.report_to(&results, &mut output).unwrap();

        let json_str = output.get_output();
        let json: serde_json::Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json["files_checked"], 2);
        assert_eq!(json["files_with_issues"], 0);
        assert_eq!(json["total_issues"], 0);
        assert_eq!(json["errors"].as_array().unwrap().len(), 2);
        assert_eq!(json["errors"][0]["file"], "error1.txt");
        assert_eq!(json["errors"][0]["error"], "File not found");
    }
}
