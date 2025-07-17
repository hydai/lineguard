//! Core checking logic as pure functions
//!
//! This module contains the core logic for checking file content,
//! implemented as pure functions for better testability.

use crate::config::Config;
use crate::{Issue, IssueType};

/// Core checker implementation with pure functions
pub struct CheckerCore {
    config: Config,
}

impl CheckerCore {
    /// Create a new CheckerCore with the given configuration
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// Get the configuration
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// Check if content has proper newline ending
    pub fn check_newline_ending(&self, content: &str) -> Option<Issue> {
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

    /// Check a single line for trailing whitespace
    pub fn check_line_trailing_whitespace(&self, line: &str, line_number: usize) -> Option<Issue> {
        if !self.config.checks.trailing_spaces {
            return None;
        }

        let trimmed = line.trim_end();
        if trimmed.len() < line.len() {
            Some(Issue {
                issue_type: IssueType::TrailingSpace,
                line: Some(line_number),
                message: "Trailing spaces found".to_string(),
            })
        } else {
            None
        }
    }

    /// Check all lines in content for trailing whitespace
    pub fn check_content_trailing_whitespace(&self, content: &str) -> Vec<Issue> {
        if !self.config.checks.trailing_spaces {
            return Vec::new();
        }

        let mut issues = Vec::new();
        for (line_num, line) in content.lines().enumerate() {
            if let Some(issue) = self.check_line_trailing_whitespace(line, line_num + 1) {
                issues.push(issue);
            }
        }
        issues
    }

    /// Check content for all issues
    pub fn check_content(&self, content: &str) -> Vec<Issue> {
        let mut issues = Vec::new();

        // Check trailing whitespace
        if self.config.checks.trailing_spaces {
            issues.extend(self.check_content_trailing_whitespace(content));
        }

        // Check newline ending
        if let Some(issue) = self.check_newline_ending(content) {
            issues.push(issue);
        }

        issues
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_newline_ending() {
        let checker = CheckerCore::new(Config::default());
        assert!(checker.check_newline_ending("content\n").is_none());
        assert!(checker.check_newline_ending("content").is_some());
    }

    #[test]
    fn test_check_newline_ending_empty() {
        let checker = CheckerCore::new(Config::default());
        assert!(checker.check_newline_ending("").is_none());
    }

    #[test]
    fn test_check_newline_ending_multiple() {
        let checker = CheckerCore::new(Config::default());
        let issue = checker.check_newline_ending("content\n\n");
        assert!(issue.is_some());
        assert_eq!(issue.unwrap().issue_type, IssueType::MultipleNewlines);
    }

    #[test]
    fn test_check_newline_ending_disabled() {
        let mut config = Config::default();
        config.checks.newline_ending = false;
        let checker = CheckerCore::new(config);
        assert!(checker.check_newline_ending("no newline").is_none());
    }

    #[test]
    fn test_check_line_trailing_whitespace() {
        let checker = CheckerCore::new(Config::default());

        // Test line with trailing spaces
        let issue = checker.check_line_trailing_whitespace("text   ", 1);
        assert!(issue.is_some());
        assert_eq!(issue.unwrap().issue_type, IssueType::TrailingSpace);

        // Test clean line
        assert!(
            checker
                .check_line_trailing_whitespace("clean text", 1)
                .is_none()
        );

        // Test line with trailing tabs
        let issue = checker.check_line_trailing_whitespace("text\t\t", 1);
        assert!(issue.is_some());
    }

    #[test]
    fn test_check_line_trailing_whitespace_disabled() {
        let mut config = Config::default();
        config.checks.trailing_spaces = false;
        let checker = CheckerCore::new(config);
        assert!(
            checker
                .check_line_trailing_whitespace("text   ", 1)
                .is_none()
        );
    }

    #[test]
    fn test_check_content_trailing_whitespace() {
        let checker = CheckerCore::new(Config::default());
        let content = "line1\nline2   \nline3\nline4\t\n";
        let issues = checker.check_content_trailing_whitespace(content);

        assert_eq!(issues.len(), 2);
        assert_eq!(issues[0].line, Some(2));
        assert_eq!(issues[1].line, Some(4));
    }

    #[test]
    fn test_check_content_comprehensive() {
        let checker = CheckerCore::new(Config::default());
        let content = "line1\nline2   \nline3";
        let issues = checker.check_content(content);

        assert_eq!(issues.len(), 2);
        // Should have one trailing space issue and one missing newline issue
        assert!(
            issues
                .iter()
                .any(|i| i.issue_type == IssueType::TrailingSpace)
        );
        assert!(
            issues
                .iter()
                .any(|i| i.issue_type == IssueType::MissingNewline)
        );
    }

    #[test]
    fn test_check_content_all_checks_disabled() {
        let mut config = Config::default();
        config.checks.trailing_spaces = false;
        config.checks.newline_ending = false;

        let checker = CheckerCore::new(config);
        let content = "line1   \nline2";
        let issues = checker.check_content(content);

        assert!(issues.is_empty());
    }

    #[test]
    fn test_check_content_edge_cases() {
        let checker = CheckerCore::new(Config::default());

        // Empty content
        assert!(checker.check_content("").is_empty());

        // Only newline
        assert!(checker.check_content("\n").is_empty());

        // Only spaces
        let issues = checker.check_content("   ");
        assert_eq!(issues.len(), 2); // trailing spaces + missing newline
    }

    #[test]
    fn test_checker_core_config_getter() {
        let config = Config::default();
        let checker = CheckerCore::new(config.clone());

        // Test the config() getter
        assert_eq!(
            checker.config().checks.trailing_spaces,
            config.checks.trailing_spaces
        );
        assert_eq!(
            checker.config().checks.newline_ending,
            config.checks.newline_ending
        );
    }

    #[test]
    fn test_check_content_trailing_whitespace_disabled() {
        let mut config = Config::default();
        config.checks.trailing_spaces = false;
        let checker = CheckerCore::new(config);

        // Should return empty vec immediately when check is disabled
        let content = "line with spaces   \nmore spaces   \n";
        let issues = checker.check_content_trailing_whitespace(content);
        assert!(issues.is_empty());
    }
}
