use lineguard::checker::{Issue, IssueType};
use lineguard::config::{CheckConfig, Config};
use lineguard::fixer::fix_file;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_fix_large_file_streaming() {
    let temp_dir = TempDir::new().unwrap();
    let large_file = temp_dir.path().join("large.txt");

    // Create a file larger than 10MB to trigger streaming mode
    let mut content = String::new();
    for i in 0..500000 {
        content.push_str(&format!("Line {i} with trailing spaces   \n"));
    }
    // Add some lines without newline at the end
    content.push_str("Last line without newline");

    fs::write(&large_file, &content).unwrap();

    let issues = vec![
        Issue {
            issue_type: IssueType::TrailingSpace,
            line: None,
            message: "Trailing spaces found".to_string(),
        },
        Issue {
            issue_type: IssueType::MissingNewline,
            line: None,
            message: "Missing newline at end of file".to_string(),
        },
    ];

    let config = Config::default();
    let result = fix_file(&large_file, &issues, &config, false).unwrap();

    assert!(result.fixed);
    assert_eq!(result.issues_fixed.len(), 2);

    // Verify the file was actually fixed
    let fixed_content = fs::read_to_string(&large_file).unwrap();
    assert!(fixed_content.ends_with('\n'));
    assert!(!fixed_content.contains("   \n"));
}

#[test]
fn test_fix_large_file_dry_run() {
    let temp_dir = TempDir::new().unwrap();
    let large_file = temp_dir.path().join("large_dry.txt");

    // Create a file larger than 10MB
    let mut content = String::new();
    for i in 0..500000 {
        content.push_str(&format!("Line {i} with spaces  \n"));
    }
    fs::write(&large_file, &content).unwrap();

    let issues = vec![Issue {
        issue_type: IssueType::TrailingSpace,
        line: None,
        message: "Trailing spaces found".to_string(),
    }];

    let config = Config::default();
    let result = fix_file(&large_file, &issues, &config, true).unwrap();

    assert!(result.fixed);
    assert_eq!(result.issues_fixed.len(), 1);

    // Verify the file was NOT modified (dry run)
    let actual_content = fs::read_to_string(&large_file).unwrap();
    assert_eq!(actual_content, content);
}

#[test]
fn test_fix_streaming_no_issues() {
    let temp_dir = TempDir::new().unwrap();
    let large_file = temp_dir.path().join("large_good.txt");

    // Create a large file without issues
    let mut content = String::new();
    for i in 0..500000 {
        content.push_str(&format!("Line {i} is good\n"));
    }
    fs::write(&large_file, &content).unwrap();

    let issues = vec![];
    let config = Config::default();
    let result = fix_file(&large_file, &issues, &config, false).unwrap();

    assert!(!result.fixed);
    assert!(result.issues_fixed.is_empty());
}

#[test]
fn test_fix_streaming_only_trailing_spaces() {
    let temp_dir = TempDir::new().unwrap();
    let large_file = temp_dir.path().join("large_spaces.txt");

    // Create a large file with only trailing space issues
    let mut content = String::new();
    for i in 0..300000 {
        content.push_str(&format!("Line {i} with spaces    \n"));
    }
    fs::write(&large_file, &content).unwrap();

    let issues = vec![Issue {
        issue_type: IssueType::TrailingSpace,
        line: None,
        message: "Trailing spaces found".to_string(),
    }];

    let config = Config::default();
    let result = fix_file(&large_file, &issues, &config, false).unwrap();

    assert!(result.fixed);

    // Verify spaces were removed
    let fixed_content = fs::read_to_string(&large_file).unwrap();
    assert!(!fixed_content.contains("    \n"));
}

#[test]
fn test_fix_streaming_only_newline_issues() {
    let temp_dir = TempDir::new().unwrap();
    let large_file = temp_dir.path().join("large_newlines.txt");

    // Create a large file with multiple newlines at the end
    let mut content = String::new();
    for i in 0..300000 {
        content.push_str(&format!("Line {i}\n"));
    }
    content.push_str("\n\n\n");
    fs::write(&large_file, &content).unwrap();

    let issues = vec![Issue {
        issue_type: IssueType::MultipleNewlines,
        line: None,
        message: "Multiple trailing newlines".to_string(),
    }];

    let config = Config::default();
    let result = fix_file(&large_file, &issues, &config, false).unwrap();

    assert!(result.fixed);

    // Verify extra newlines were removed
    let fixed_content = fs::read_to_string(&large_file).unwrap();
    assert!(fixed_content.ends_with('\n'));
    assert!(!fixed_content.ends_with("\n\n"));
}

#[test]
fn test_fix_with_disabled_checks() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("disabled_checks.txt");

    fs::write(&file_path, "content with spaces  ").unwrap();

    let issues = vec![
        Issue {
            issue_type: IssueType::TrailingSpace,
            line: Some(1),
            message: "Trailing spaces found".to_string(),
        },
        Issue {
            issue_type: IssueType::MissingNewline,
            line: None,
            message: "Missing newline at end of file".to_string(),
        },
    ];

    // Disable both checks
    let config = Config {
        checks: CheckConfig {
            newline_ending: false,
            trailing_spaces: false,
        },
        ..Default::default()
    };

    let result = fix_file(&file_path, &issues, &config, false).unwrap();

    assert!(!result.fixed);
    assert!(result.issues_fixed.is_empty());

    // File should remain unchanged
    let content = fs::read_to_string(&file_path).unwrap();
    assert_eq!(content, "content with spaces  ");
}

#[test]
fn test_fix_streaming_empty_lines_at_end() {
    let temp_dir = TempDir::new().unwrap();
    let large_file = temp_dir.path().join("large_empty_end.txt");

    // Create a large file with empty lines at the end
    let mut content = String::new();
    for i in 0..300000 {
        content.push_str(&format!("Line {i}\n"));
    }
    content.push_str("\n\n");
    fs::write(&large_file, &content).unwrap();

    let issues = vec![Issue {
        issue_type: IssueType::MultipleNewlines,
        line: None,
        message: "Multiple trailing newlines".to_string(),
    }];

    let config = Config::default();
    let result = fix_file(&large_file, &issues, &config, false).unwrap();

    assert!(result.fixed);

    // Verify empty lines were removed
    let fixed_content = fs::read_to_string(&large_file).unwrap();
    assert!(fixed_content.ends_with('\n'));
    assert!(!fixed_content.ends_with("\n\n"));
}

#[test]
#[cfg(unix)]
fn test_fix_with_write_error() {
    use std::os::unix::fs::PermissionsExt;

    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("readonly.txt");

    fs::write(&file_path, "content  ").unwrap();

    // Make file read-only
    let mut perms = fs::metadata(&file_path).unwrap().permissions();
    perms.set_mode(0o444);
    fs::set_permissions(&file_path, perms).unwrap();

    let issues = vec![Issue {
        issue_type: IssueType::TrailingSpace,
        line: Some(1),
        message: "Trailing spaces found".to_string(),
    }];

    let config = Config::default();
    let result = fix_file(&file_path, &issues, &config, false);

    assert!(result.is_err());

    // Restore permissions
    let mut perms = fs::metadata(&file_path).unwrap().permissions();
    perms.set_mode(0o644);
    fs::set_permissions(&file_path, perms).unwrap();
}

#[test]
fn test_fix_empty_file() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("empty.txt");

    fs::write(&file_path, "").unwrap();

    let issues = vec![];
    let config = Config::default();
    let result = fix_file(&file_path, &issues, &config, false).unwrap();

    assert!(!result.fixed);
    assert!(result.issues_fixed.is_empty());
}

#[test]
fn test_fix_file_with_only_newline() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("only_newline.txt");

    fs::write(&file_path, "\n").unwrap();

    let issues = vec![];
    let config = Config::default();
    let result = fix_file(&file_path, &issues, &config, false).unwrap();

    assert!(!result.fixed);
}

#[test]
fn test_apply_fixes_with_partial_config() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("partial.txt");

    fs::write(&file_path, "line with spaces  \nno newline").unwrap();

    // Only report trailing space issue, but config allows both fixes
    let issues = vec![Issue {
        issue_type: IssueType::TrailingSpace,
        line: Some(1),
        message: "Trailing spaces found".to_string(),
    }];

    let config = Config::default();
    let result = fix_file(&file_path, &issues, &config, false).unwrap();

    assert!(result.fixed);

    // Only trailing spaces should be fixed
    let content = fs::read_to_string(&file_path).unwrap();
    assert_eq!(content, "line with spaces\nno newline");
}

#[test]
fn test_fix_streaming_io_error() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("nonexistent_dir").join("file.txt");

    let issues = vec![Issue {
        issue_type: IssueType::TrailingSpace,
        line: Some(1),
        message: "Trailing spaces found".to_string(),
    }];

    let config = Config::default();
    let result = fix_file(&file_path, &issues, &config, false);

    assert!(result.is_err());
}
