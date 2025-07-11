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

#[derive(Debug, Clone, PartialEq)]
pub enum IssueType {
    MissingNewline,
    MultipleNewlines,
    TrailingSpace,
}

pub fn check_file(_path: &Path) -> CheckResult {
    todo!("Implement file checking")
}

pub fn check_newline_ending(_content: &str) -> Option<Issue> {
    todo!("Implement newline ending check")
}

pub fn check_trailing_spaces(_content: &str) -> Vec<Issue> {
    todo!("Implement trailing spaces check")
}
