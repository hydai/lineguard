use crate::config::Config;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
pub struct CheckResult {
    pub file_path: std::path::PathBuf,
    pub issues: Vec<Issue>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Issue {
    pub issue_type: IssueType,
    pub line: Option<usize>,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IssueType {
    MissingNewline,
    MultipleNewlines,
    TrailingSpace,
}

pub fn check_file(path: &Path, config: &Config) -> CheckResult {
    let mut issues = Vec::new();

    // Read file content
    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(_) => {
            // If we can't read the file, return empty result
            return CheckResult {
                file_path: path.to_path_buf(),
                issues,
            };
        },
    };

    // Check newline ending if enabled
    if config.checks.newline_ending {
        if let Some(issue) = check_newline_ending(&content) {
            issues.push(issue);
        }
    }

    // Check trailing spaces if enabled
    if config.checks.trailing_spaces {
        let mut trailing_space_issues = check_trailing_spaces(&content);
        issues.append(&mut trailing_space_issues);
    }

    CheckResult {
        file_path: path.to_path_buf(),
        issues,
    }
}

pub fn check_newline_ending(content: &str) -> Option<Issue> {
    if content.is_empty() {
        // Empty files are considered valid
        return None;
    }

    if !content.ends_with('\n') {
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

pub fn check_trailing_spaces(content: &str) -> Vec<Issue> {
    let mut issues = Vec::new();

    for (line_num, line) in content.lines().enumerate() {
        let trimmed = line.trim_end();
        if trimmed.len() < line.len() {
            issues.push(Issue {
                issue_type: IssueType::TrailingSpace,
                line: Some(line_num + 1), // Line numbers are 1-based
                message: "Trailing spaces found".to_string(),
            });
        }
    }

    issues
}
