use lineguard::checker::{IssueType, check_file};
use std::fs;
use tempfile::TempDir;

#[test]
fn test_check_file_with_issues() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");

    // File with both missing newline and trailing spaces
    fs::write(&file_path, "line 1  \nline 2\nline 3").unwrap();

    let result = check_file(&file_path);

    assert_eq!(result.file_path, file_path);
    assert_eq!(result.issues.len(), 2);

    // Should have trailing space issue
    let trailing_space_issue = result
        .issues
        .iter()
        .find(|i| i.issue_type == IssueType::TrailingSpace)
        .expect("Should find trailing space issue");
    assert_eq!(trailing_space_issue.line, Some(1));

    // Should have missing newline issue
    let newline_issue = result
        .issues
        .iter()
        .find(|i| i.issue_type == IssueType::MissingNewline)
        .expect("Should find missing newline issue");
    assert!(newline_issue.line.is_none());
}

#[test]
fn test_check_file_no_issues() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("perfect.txt");

    // Perfect file
    fs::write(&file_path, "line 1\nline 2\nline 3\n").unwrap();

    let result = check_file(&file_path);

    assert_eq!(result.file_path, file_path);
    assert!(result.issues.is_empty());
}
