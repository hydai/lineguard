use lineguard::checker::{IssueType, check_file};
use lineguard::config::Config;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_check_file_with_issues() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");

    // File with both missing newline and trailing spaces
    fs::write(&file_path, "line 1  \nline 2\nline 3").unwrap();

    let config = Config::default();
    let result = check_file(&file_path, &config);

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

    let config = Config::default();
    let result = check_file(&file_path, &config);

    assert_eq!(result.file_path, file_path);
    assert!(result.issues.is_empty());
}

// Stdin integration tests
use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_stdin_with_empty_input() {
    let mut cmd = Command::cargo_bin("lineguard").unwrap();
    cmd.arg("--stdin");

    // Provide empty stdin
    cmd.write_stdin("");

    cmd.assert()
        .success()
        .stderr(predicate::str::contains("No files found to check"));
}

#[test]
fn test_stdin_with_file_paths() {
    let temp_dir = TempDir::new().unwrap();

    // Create test files
    let file1 = temp_dir.path().join("test1.txt");
    let file2 = temp_dir.path().join("test2.txt");
    fs::write(&file1, "content with trailing spaces  \n").unwrap();
    fs::write(&file2, "good content\n").unwrap();

    let mut cmd = Command::cargo_bin("lineguard").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg("--stdin");

    // Provide file paths via stdin
    let stdin_input = format!("{}\n{}\n", file1.to_string_lossy(), file2.to_string_lossy());
    cmd.write_stdin(stdin_input);

    cmd.assert()
        .failure() // Should fail because file1 has issues
        .stdout(predicate::str::contains("test1.txt"))
        .stdout(predicate::str::contains("Trailing spaces found"));
}

#[test]
fn test_stdin_with_nonexistent_files() {
    let mut cmd = Command::cargo_bin("lineguard").unwrap();
    cmd.arg("--stdin");

    // Provide paths to non-existent files
    cmd.write_stdin("nonexistent1.txt\nnonexistent2.txt\n");

    cmd.assert()
        .success()
        .stderr(predicate::str::contains("No files found to check"));
}
