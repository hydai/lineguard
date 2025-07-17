//! Trait definitions for the checker module
//!
//! This module defines the traits used for abstraction in the checker,
//! allowing for easier testing and better separation of concerns.

use crate::Issue;
use crate::config::Config;

/// Trait for checking individual lines for issues
pub trait LineChecker {
    /// Check a single line for trailing whitespace issues
    fn check_line(&self, line: &str, line_number: usize) -> Option<Issue>;

    /// Check if the file content has proper newline ending
    fn check_final_newline(&self, content: &str) -> Option<Issue>;

    /// Get the configuration
    fn config(&self) -> &Config;
}

/// Trait for checking entire file content
pub trait ContentChecker {
    /// Check the entire content and return all issues found
    fn check_content(&self, content: &str) -> Vec<Issue>;

    /// Check content line by line (useful for streaming)
    fn check_lines<I>(&self, lines: I) -> Vec<Issue>
    where
        I: Iterator<Item = String>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::IssueType;

    struct MockLineChecker {
        config: Config,
    }

    impl LineChecker for MockLineChecker {
        fn check_line(&self, line: &str, line_number: usize) -> Option<Issue> {
            if self.config.checks.trailing_spaces && line.trim_end().len() < line.len() {
                Some(Issue {
                    issue_type: IssueType::TrailingSpace,
                    line: Some(line_number),
                    message: "Trailing spaces found".to_string(),
                })
            } else {
                None
            }
        }

        fn check_final_newline(&self, content: &str) -> Option<Issue> {
            if !self.config.checks.newline_ending {
                return None;
            }

            if content.is_empty() {
                None
            } else if !content.ends_with('\n') {
                Some(Issue {
                    issue_type: IssueType::MissingNewline,
                    line: None,
                    message: "Missing newline at end of file".to_string(),
                })
            } else if content.ends_with("\n\n") {
                Some(Issue {
                    issue_type: IssueType::MultipleNewlines,
                    line: None,
                    message: "Multiple newlines at end of file".to_string(),
                })
            } else {
                None
            }
        }

        fn config(&self) -> &Config {
            &self.config
        }
    }

    #[test]
    fn test_line_checker_trait() {
        let checker = MockLineChecker {
            config: Config::default(),
        };

        // Test trailing spaces detection
        let issue = checker.check_line("test   ", 1);
        assert!(issue.is_some());
        assert_eq!(issue.unwrap().issue_type, IssueType::TrailingSpace);

        // Test clean line
        let issue = checker.check_line("clean line", 1);
        assert!(issue.is_none());
    }

    #[test]
    fn test_newline_checker_trait() {
        let checker = MockLineChecker {
            config: Config::default(),
        };

        // Test missing newline
        let issue = checker.check_final_newline("no newline");
        assert!(issue.is_some());
        assert_eq!(issue.unwrap().issue_type, IssueType::MissingNewline);

        // Test proper newline
        let issue = checker.check_final_newline("proper\n");
        assert!(issue.is_none());

        // Test multiple newlines
        let issue = checker.check_final_newline("multiple\n\n");
        assert!(issue.is_some());
        assert_eq!(issue.unwrap().issue_type, IssueType::MultipleNewlines);
    }

    #[test]
    fn test_config_disabled_checks() {
        let mut config = Config::default();
        config.checks.trailing_spaces = false;
        config.checks.newline_ending = false;

        let checker = MockLineChecker { config };

        // Should not detect issues when checks are disabled
        assert!(checker.check_line("trailing   ", 1).is_none());
        assert!(checker.check_final_newline("no newline").is_none());
    }
}
