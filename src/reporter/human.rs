//! Human-readable reporter implementation
//!
//! This module provides a reporter that outputs results in a human-friendly format
//! with optional color support.

use crate::CheckResult;
use crate::reporter::{Color, ColoredOutput, Output, Reporter, ReporterWithOutput};
use std::io;

/// Human-readable reporter with color support
pub struct HumanReporter {
    /// Whether to use colored output
    pub use_color: bool,
}

impl HumanReporter {
    /// Create a new human reporter without color
    pub fn new() -> Self {
        Self { use_color: false }
    }

    /// Create a new human reporter with color support
    pub fn with_color() -> Self {
        Self { use_color: true }
    }
}

impl Default for HumanReporter {
    fn default() -> Self {
        Self::new()
    }
}

impl Reporter for HumanReporter {
    fn report(&self, results: &[CheckResult]) {
        // Use StdOutput as the default output
        let mut output = if self.use_color {
            crate::reporter::StdOutput::with_color()
        } else {
            crate::reporter::StdOutput::new()
        };

        // Ignore any errors from output operations in the legacy interface
        let _ = self.report_to_colored(results, &mut output);
    }
}

impl ReporterWithOutput for HumanReporter {
    fn report_to(&self, results: &[CheckResult], output: &mut dyn Output) -> io::Result<()> {
        let mut total_issues = 0;
        let mut files_with_issues = 0;

        // Report individual file issues
        for result in results {
            if !result.issues.is_empty() {
                files_with_issues += 1;
                total_issues += result.issues.len();

                // File header
                output.write_line(&format!("✗ {}", result.file_path.display()))?;

                // Issues
                for issue in &result.issues {
                    match issue.line {
                        Some(line) => {
                            output.write_line(&format!("  - Line {line}: {}", issue.message))?;
                        },
                        None => {
                            output.write_line(&format!("  - {}", issue.message))?;
                        },
                    }
                }
                output.write_line("")?;
            }
        }

        // Summary
        if total_issues == 0 {
            output.write_line("✓ All files passed lint checks!")?;
            let files_checked = results.len();
            output.write_line(&format!("  Files checked: {files_checked}"))?;
        } else {
            output.write_line(&format!(
                "✗ Found {total_issues} issues in {files_with_issues} files"
            ))?;
            let files_checked = results.len();
            output.write_line(&format!("  Files checked: {files_checked}"))?;
        }

        output.flush()?;
        Ok(())
    }
}

impl HumanReporter {
    /// Report with colored output support
    pub fn report_to_colored(
        &self,
        results: &[CheckResult],
        output: &mut dyn ColoredOutput,
    ) -> io::Result<()> {
        let mut total_issues = 0;
        let mut files_with_issues = 0;

        // Report individual file issues
        for result in results {
            if !result.issues.is_empty() {
                files_with_issues += 1;
                total_issues += result.issues.len();

                // File header with color
                if self.use_color {
                    output.write_colored("✗", Color::Red)?;
                    output.write(" ")?;
                    output.write(&format!("{}", result.file_path.display()))?;
                    output.write_line("")?;
                } else {
                    output.write_line(&format!("✗ {}", result.file_path.display()))?;
                }

                // Issues
                for issue in &result.issues {
                    match issue.line {
                        Some(line) => {
                            output.write_line(&format!("  - Line {line}: {}", issue.message))?;
                        },
                        None => {
                            output.write_line(&format!("  - {}", issue.message))?;
                        },
                    }
                }
                output.write_line("")?;
            }
        }

        // Summary
        if total_issues == 0 {
            if self.use_color {
                output.write_colored("✓", Color::Green)?;
                output.write_line(" All files passed lint checks!")?;
            } else {
                output.write_line("✓ All files passed lint checks!")?;
            }
            let files_checked = results.len();
            output.write_line(&format!("  Files checked: {files_checked}"))?;
        } else {
            if self.use_color {
                output.write_colored("✗", Color::Red)?;
                output.write(&format!(
                    " Found {total_issues} issues in {files_with_issues} files"
                ))?;
                output.write_line("")?;
            } else {
                output.write_line(&format!(
                    "✗ Found {total_issues} issues in {files_with_issues} files"
                ))?;
            }
            let files_checked = results.len();
            output.write_line(&format!("  Files checked: {files_checked}"))?;
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
                issues: vec![Issue {
                    issue_type: IssueType::MultipleNewlines,
                    line: None,
                    message: "Multiple newlines at end of file".to_string(),
                }],
                error: None,
            },
        ]
    }

    #[test]
    fn test_human_reporter_no_issues() {
        let reporter = HumanReporter::new();
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

        let buffer = output.get_output();
        assert!(buffer.contains("✓ All files passed lint checks!"));
        assert!(buffer.contains("Files checked: 2"));
        assert!(!buffer.contains("✗"));
    }

    #[test]
    fn test_human_reporter_with_issues() {
        let reporter = HumanReporter::new();
        let mut output = MockOutput::new();
        let results = create_test_results();

        reporter.report_to(&results, &mut output).unwrap();

        let buffer = output.get_output();

        // Check file headers
        assert!(buffer.contains("✗ test1.txt"));
        assert!(buffer.contains("✗ test3.txt"));
        assert!(!buffer.contains("✗ test2.txt")); // This file has no issues

        // Check issues
        assert!(buffer.contains("Line 5: Trailing spaces found"));
        assert!(buffer.contains("Missing newline at end of file"));
        assert!(buffer.contains("Multiple newlines at end of file"));

        // Check summary
        assert!(buffer.contains("✗ Found 3 issues in 2 files"));
        assert!(buffer.contains("Files checked: 3"));
    }

    #[test]
    fn test_human_reporter_with_color() {
        let reporter = HumanReporter::with_color();
        let mut output = MockOutput::new();
        let results = create_test_results();

        reporter.report_to(&results, &mut output).unwrap();

        // For now, MockOutput doesn't implement ColoredOutput,
        // so we should still get plain output
        let buffer = output.get_output();
        assert!(buffer.contains("✗ test1.txt"));
        assert!(buffer.contains("✗ Found 3 issues in 2 files"));
    }

    #[test]
    fn test_human_reporter_legacy_interface() {
        let reporter = HumanReporter::new();
        let results = create_test_results();

        // This should not panic
        reporter.report(&results);
    }

    #[test]
    fn test_human_reporter_empty_results() {
        let reporter = HumanReporter::new();
        let mut output = MockOutput::new();
        let results = vec![];

        reporter.report_to(&results, &mut output).unwrap();

        let buffer = output.get_output();
        assert!(buffer.contains("✓ All files passed lint checks!"));
        assert!(buffer.contains("Files checked: 0"));
    }

    #[test]
    fn test_human_reporter_single_issue() {
        let reporter = HumanReporter::new();
        let mut output = MockOutput::new();
        let results = vec![CheckResult {
            file_path: PathBuf::from("single.txt"),
            issues: vec![Issue {
                issue_type: IssueType::TrailingSpace,
                line: Some(10),
                message: "Trailing spaces found".to_string(),
            }],
            error: None,
        }];

        reporter.report_to(&results, &mut output).unwrap();

        let buffer = output.get_output();
        assert!(buffer.contains("✗ single.txt"));
        assert!(buffer.contains("Line 10: Trailing spaces found"));
        assert!(buffer.contains("✗ Found 1 issues in 1 files"));
    }

    #[test]
    fn test_human_reporter_default() {
        let reporter = HumanReporter::default();
        assert!(!reporter.use_color);
    }

    #[test]
    fn test_human_reporter_report_to_colored_with_issues() {
        let reporter = HumanReporter::new();
        let mut output = MockOutput::new();
        let results = create_test_results();

        reporter.report_to_colored(&results, &mut output).unwrap();

        // Note: report_to_colored doesn't actually use colors in current implementation
        // It just outputs to ColoredOutput trait object
        let output_str = output.get_output();
        assert!(output_str.contains("test1.txt"));
        assert!(output_str.contains("Trailing spaces found"));
    }

    #[test]
    fn test_human_reporter_report_to_colored_no_issues() {
        let reporter = HumanReporter::new();
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

        reporter.report_to_colored(&results, &mut output).unwrap();

        // Note: report_to_colored doesn't actually use colors in current implementation
        let output_str = output.get_output();
        assert!(output_str.contains("All files passed lint checks!"));
    }

    #[test]
    fn test_human_reporter_legacy_with_color() {
        let reporter = HumanReporter { use_color: true };
        let results = create_test_results();

        // This should not panic and should use colored output
        reporter.report(&results);
    }

    #[test]
    fn test_human_reporter_multiple_files_no_issues() {
        let reporter = HumanReporter::new();
        let mut output = MockOutput::new();
        let results = vec![
            CheckResult {
                file_path: PathBuf::from("file1.txt"),
                issues: vec![],
                error: None,
            },
            CheckResult {
                file_path: PathBuf::from("file2.txt"),
                issues: vec![],
                error: None,
            },
            CheckResult {
                file_path: PathBuf::from("file3.txt"),
                issues: vec![],
                error: None,
            },
        ];

        reporter.report_to(&results, &mut output).unwrap();

        let output_str = output.get_output();
        assert!(output_str.contains("✓ All files passed lint checks!"));
        assert!(output_str.contains("Files checked: 3"));
    }
}
