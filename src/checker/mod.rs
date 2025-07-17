pub mod core;
pub mod file_checker;
pub mod io_trait;
pub mod traits;

#[cfg(test)]
mod tests;

use crate::config::Config;
use std::path::Path;

// Re-export traits and core
pub use core::CheckerCore;
pub use file_checker::{FileChecker, StdFileReader};
pub use io_trait::{FileMetadata, FileReader};
pub use traits::{ContentChecker, LineChecker};

#[derive(Debug, Clone, PartialEq)]
pub struct CheckResult {
    pub file_path: std::path::PathBuf,
    pub issues: Vec<Issue>,
    pub error: Option<String>,
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
    // Use FileChecker with StdFileReader for backward compatibility
    let file_checker = FileChecker::new(StdFileReader, config.clone());
    file_checker.check_file(path)
}

// Legacy functions kept for backward compatibility
// These now just delegate to CheckerCore functions

pub fn check_newline_ending(content: &str) -> Option<Issue> {
    let checker = CheckerCore::new(Config::default());
    checker.check_newline_ending(content)
}

pub fn check_trailing_spaces(content: &str) -> Vec<Issue> {
    let checker = CheckerCore::new(Config::default());
    checker.check_content_trailing_whitespace(content)
}
