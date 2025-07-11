use lineguard::checker::{IssueType, check_newline_ending, check_trailing_spaces};

#[test]
fn test_detect_missing_newline() {
    let content = "Hello, world!";
    let issue = check_newline_ending(content);

    assert!(issue.is_some());
    let issue = issue.unwrap();
    assert_eq!(issue.issue_type, IssueType::MissingNewline);
    assert_eq!(issue.message, "Missing newline at end of file");
    assert_eq!(issue.line, None);
}

#[test]
fn test_file_with_proper_newline() {
    let content = "Hello, world!\n";
    let issue = check_newline_ending(content);

    assert!(issue.is_none());
}

#[test]
fn test_empty_file() {
    let content = "";
    let issue = check_newline_ending(content);

    assert!(issue.is_none()); // Empty files are considered valid
}

#[test]
fn test_detect_multiple_newlines() {
    let content = "Hello, world!\n\n";
    let issue = check_newline_ending(content);

    assert!(issue.is_some());
    let issue = issue.unwrap();
    assert_eq!(issue.issue_type, IssueType::MultipleNewlines);
    assert_eq!(issue.message, "Multiple newlines at end of file");
}

#[test]
fn test_detect_many_newlines() {
    let content = "Hello, world!\n\n\n\n";
    let issue = check_newline_ending(content);

    assert!(issue.is_some());
    let issue = issue.unwrap();
    assert_eq!(issue.issue_type, IssueType::MultipleNewlines);
}

#[test]
fn test_detect_trailing_spaces() {
    let content = "line 1  \nline 2 \nline 3\n";
    let issues = check_trailing_spaces(content);

    assert_eq!(issues.len(), 2);

    assert_eq!(issues[0].issue_type, IssueType::TrailingSpace);
    assert_eq!(issues[0].line, Some(1));
    assert!(issues[0].message.contains("Trailing spaces"));

    assert_eq!(issues[1].issue_type, IssueType::TrailingSpace);
    assert_eq!(issues[1].line, Some(2));
}

#[test]
fn test_no_trailing_spaces() {
    let content = "line 1\nline 2\nline 3\n";
    let issues = check_trailing_spaces(content);

    assert!(issues.is_empty());
}

#[test]
fn test_trailing_tabs() {
    let content = "line 1\t\t\nline 2\n";
    let issues = check_trailing_spaces(content);

    assert_eq!(issues.len(), 1);
    assert_eq!(issues[0].issue_type, IssueType::TrailingSpace);
    assert_eq!(issues[0].line, Some(1));
}

#[test]
fn test_crlf_line_endings() {
    // File with CRLF line endings should be treated correctly
    let content = "line 1\r\nline 2\r\nline 3\r\n";

    // Should not detect trailing spaces from \r
    let issues = check_trailing_spaces(content);
    assert!(issues.is_empty());

    // Should not detect missing newline
    let newline_issue = check_newline_ending(content);
    assert!(newline_issue.is_none());
}

#[test]
fn test_crlf_with_trailing_spaces() {
    let content = "line 1  \r\nline 2\r\nline 3\r\n";
    let issues = check_trailing_spaces(content);

    assert_eq!(issues.len(), 1);
    assert_eq!(issues[0].line, Some(1));
}

#[test]
fn test_crlf_missing_final_newline() {
    let content = "line 1\r\nline 2\r\nline 3";
    let issue = check_newline_ending(content);

    assert!(issue.is_some());
    assert_eq!(issue.unwrap().issue_type, IssueType::MissingNewline);
}
