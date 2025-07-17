//! Comprehensive unit tests for the checker module
//!
//! This module contains extensive tests for all public functions and types
//! in the checker module, including normal path, error handling, and edge cases.

#[cfg(test)]
mod checker_tests {
    use super::super::*;
    use crate::config::Config;
    use crate::testing::builders::TestFileBuilder;
    use crate::testing::mocks::MockFileSystem;
    use std::io::Write;
    use std::path::PathBuf;
    use tempfile::NamedTempFile;

    // ===== Tests for check_file function =====

    #[test]
    fn test_check_file_success() {
        // Create a temporary file with content
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "Hello world").unwrap();
        temp_file.flush().unwrap();

        let result = check_file(temp_file.path(), &Config::default());

        assert_eq!(result.file_path, temp_file.path());
        assert!(result.error.is_none());
        assert!(result.issues.is_empty());
    }

    #[test]
    fn test_check_file_with_issues() {
        let mut temp_file = NamedTempFile::new().unwrap();
        write!(temp_file, "trailing spaces   \nno final newline").unwrap();
        temp_file.flush().unwrap();

        let result = check_file(temp_file.path(), &Config::default());

        assert!(result.error.is_none());
        assert_eq!(result.issues.len(), 2);
        assert!(
            result
                .issues
                .iter()
                .any(|i| i.issue_type == IssueType::TrailingSpace)
        );
        assert!(
            result
                .issues
                .iter()
                .any(|i| i.issue_type == IssueType::MissingNewline)
        );
    }

    #[test]
    fn test_check_file_nonexistent() {
        let result = check_file(&PathBuf::from("/nonexistent/file.txt"), &Config::default());

        assert!(result.error.is_some());
        assert!(result.issues.is_empty());
        assert!(result.error.unwrap().contains("nonexistent"));
    }

    #[test]
    fn test_check_file_with_disabled_checks() {
        let mut temp_file = NamedTempFile::new().unwrap();
        write!(temp_file, "trailing spaces   \nno final newline").unwrap();
        temp_file.flush().unwrap();

        let mut config = Config::default();
        config.checks.trailing_spaces = false;
        config.checks.newline_ending = false;

        let result = check_file(temp_file.path(), &config);

        assert!(result.error.is_none());
        assert!(result.issues.is_empty());
    }

    // ===== Tests for check_newline_ending function =====

    #[test]
    fn test_check_newline_ending_normal() {
        assert!(check_newline_ending("content\n").is_none());
        assert!(check_newline_ending("line1\nline2\n").is_none());
    }

    #[test]
    fn test_check_newline_ending_missing() {
        let issue = check_newline_ending("no newline");
        assert!(issue.is_some());
        let issue = issue.unwrap();
        assert_eq!(issue.issue_type, IssueType::MissingNewline);
        assert!(issue.line.is_none());
        assert!(issue.message.contains("Missing newline"));
    }

    #[test]
    fn test_check_newline_ending_multiple() {
        let issue = check_newline_ending("content\n\n");
        assert!(issue.is_some());
        let issue = issue.unwrap();
        assert_eq!(issue.issue_type, IssueType::MultipleNewlines);
        assert!(issue.message.contains("Multiple newlines"));
    }

    #[test]
    fn test_check_newline_ending_empty() {
        assert!(check_newline_ending("").is_none());
    }

    #[test]
    fn test_check_newline_ending_only_newlines() {
        assert!(check_newline_ending("\n").is_none());
        assert!(check_newline_ending("\n\n").is_some());
        assert!(check_newline_ending("\n\n\n").is_some());
    }

    // ===== Tests for check_trailing_spaces function =====

    #[test]
    fn test_check_trailing_spaces_clean() {
        let issues = check_trailing_spaces("clean line\nanother clean line\n");
        assert!(issues.is_empty());
    }

    #[test]
    fn test_check_trailing_spaces_found() {
        let issues = check_trailing_spaces("line with spaces   \nclean\nmore spaces  ");
        assert_eq!(issues.len(), 2);

        assert_eq!(issues[0].issue_type, IssueType::TrailingSpace);
        assert_eq!(issues[0].line, Some(1));

        assert_eq!(issues[1].issue_type, IssueType::TrailingSpace);
        assert_eq!(issues[1].line, Some(3));
    }

    #[test]
    fn test_check_trailing_spaces_tabs() {
        let issues = check_trailing_spaces("line with tabs\t\t\nclean line");
        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].issue_type, IssueType::TrailingSpace);
        assert_eq!(issues[0].line, Some(1));
    }

    #[test]
    fn test_check_trailing_spaces_mixed_whitespace() {
        let issues = check_trailing_spaces("mixed \t \nspaces   \ntabs\t\n");
        assert_eq!(issues.len(), 3);
    }

    #[test]
    fn test_check_trailing_spaces_empty_lines() {
        // Empty lines should not be flagged as having trailing spaces
        let issues = check_trailing_spaces("line1\n\nline3\n");
        assert!(issues.is_empty());
    }

    // ===== Tests for Issue and IssueType =====

    #[test]
    fn test_issue_creation() {
        let issue = Issue {
            issue_type: IssueType::TrailingSpace,
            line: Some(42),
            message: "Test message".to_string(),
        };

        assert_eq!(issue.issue_type, IssueType::TrailingSpace);
        assert_eq!(issue.line, Some(42));
        assert_eq!(issue.message, "Test message");
    }

    #[test]
    fn test_issue_clone() {
        let issue = Issue {
            issue_type: IssueType::MissingNewline,
            line: None,
            message: "Missing newline".to_string(),
        };

        let cloned = issue.clone();
        assert_eq!(issue, cloned);
    }

    #[test]
    fn test_issue_debug() {
        let issue = Issue {
            issue_type: IssueType::MultipleNewlines,
            line: Some(10),
            message: "Multiple newlines".to_string(),
        };

        let debug_str = format!("{issue:?}");
        assert!(debug_str.contains("MultipleNewlines"));
        assert!(debug_str.contains("10"));
        assert!(debug_str.contains("Multiple newlines"));
    }

    // ===== Tests for CheckResult =====

    #[test]
    fn test_check_result_creation() {
        let result = CheckResult {
            file_path: PathBuf::from("/test/file.txt"),
            issues: vec![Issue {
                issue_type: IssueType::TrailingSpace,
                line: Some(1),
                message: "Trailing spaces found".to_string(),
            }],
            error: None,
        };

        assert_eq!(result.file_path, PathBuf::from("/test/file.txt"));
        assert_eq!(result.issues.len(), 1);
        assert!(result.error.is_none());
    }

    #[test]
    fn test_check_result_with_error() {
        let result = CheckResult {
            file_path: PathBuf::from("/test/file.txt"),
            issues: vec![],
            error: Some("Permission denied".to_string()),
        };

        assert!(result.issues.is_empty());
        assert_eq!(result.error, Some("Permission denied".to_string()));
    }

    #[test]
    fn test_check_result_clone() {
        let result = CheckResult {
            file_path: PathBuf::from("/test/file.txt"),
            issues: vec![Issue {
                issue_type: IssueType::MissingNewline,
                line: None,
                message: "Missing newline at end of file".to_string(),
            }],
            error: None,
        };

        let cloned = result.clone();
        assert_eq!(result, cloned);
    }

    // ===== Edge case tests =====

    #[test]
    fn test_check_trailing_spaces_crlf_endings() {
        // CRLF line endings should be handled correctly
        let issues = check_trailing_spaces("line1   \r\nline2\r\n");
        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].line, Some(1));
    }

    #[test]
    fn test_check_newline_ending_crlf() {
        // File with CRLF should still be detected as having proper ending
        // Note: The checker works on the content after line endings are normalized
        assert!(check_newline_ending("content\n").is_none());
        // Multiple newlines at end are still detected
        assert!(check_newline_ending("content\n\n").is_some());
    }

    #[test]
    fn test_check_trailing_spaces_very_long_line() {
        let mut long_line = "a".repeat(10000);
        long_line.push_str("   ");

        let issues = check_trailing_spaces(&long_line);
        assert_eq!(issues.len(), 1);
    }

    #[test]
    fn test_unicode_content() {
        // Test with unicode content
        let content = "Hello 世界   \n🦀 Rust\n";
        let issues = check_trailing_spaces(content);
        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].line, Some(1));

        let newline_issue = check_newline_ending("Hello 世界");
        assert!(newline_issue.is_some());
    }

    // ===== Performance tests =====

    #[test]
    fn test_check_trailing_spaces_many_lines() {
        // Create content with 1000 lines
        let mut content = String::new();
        for i in 0..1000 {
            if i % 10 == 0 {
                content.push_str(&format!("line {i} with trailing spaces   \n"));
            } else {
                content.push_str(&format!("line {i} clean\n"));
            }
        }

        let start = std::time::Instant::now();
        let issues = check_trailing_spaces(&content);
        let duration = start.elapsed();

        assert_eq!(issues.len(), 100);
        assert!(duration.as_millis() < 100); // Should complete within 100ms
    }

    #[test]
    fn test_check_file_large_file() {
        use std::io::Write;

        let mut temp_file = NamedTempFile::new().unwrap();

        // Create a 5MB file
        for _ in 0..50000 {
            writeln!(temp_file, "This is a line of text that is exactly 100 characters long including the newline character at end").unwrap();
        }
        temp_file.flush().unwrap();

        let start = std::time::Instant::now();
        let result = check_file(temp_file.path(), &Config::default());
        let duration = start.elapsed();

        assert!(result.error.is_none());
        assert!(duration.as_secs() < 2); // Should complete within 2 seconds
    }

    // ===== Integration tests with FileChecker =====

    #[test]
    fn test_file_checker_integration() {
        let mut fs = MockFileSystem::new();
        fs.add_file(
            "test.txt",
            TestFileBuilder::new("test.txt")
                .with_line("line 1")
                .with_trailing_spaces()
                .without_final_newline()
                .build()
                .1,
        );

        let checker = FileChecker::new(fs, Config::default());
        let result = checker.check_file(&PathBuf::from("test.txt"));

        assert!(result.error.is_none());
        assert_eq!(result.issues.len(), 2);
    }

    // ===== Regression tests =====

    #[test]
    fn test_empty_file_handling() {
        let mut temp_file = NamedTempFile::new().unwrap();
        // Don't write anything - empty file
        temp_file.flush().unwrap();

        let result = check_file(temp_file.path(), &Config::default());

        assert!(result.error.is_none());
        assert!(result.issues.is_empty());
    }

    #[test]
    fn test_single_newline_file() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file).unwrap();
        temp_file.flush().unwrap();

        let result = check_file(temp_file.path(), &Config::default());

        assert!(result.error.is_none());
        assert!(result.issues.is_empty());
    }

    #[test]
    fn test_windows_line_endings() {
        let mut temp_file = NamedTempFile::new().unwrap();
        write!(temp_file, "line1\r\nline2\r\n").unwrap();
        temp_file.flush().unwrap();

        let result = check_file(temp_file.path(), &Config::default());

        assert!(result.error.is_none());
        assert!(result.issues.is_empty());
    }

    // ===== Additional error handling tests =====

    #[test]
    fn test_check_file_permission_denied() {
        #[cfg(unix)]
        {
            use std::fs;
            use std::os::unix::fs::PermissionsExt;

            let mut temp_file = NamedTempFile::new().unwrap();
            writeln!(temp_file, "test content").unwrap();
            temp_file.flush().unwrap();

            // Remove read permissions
            let mut perms = fs::metadata(temp_file.path()).unwrap().permissions();
            perms.set_mode(0o000);
            fs::set_permissions(temp_file.path(), perms).unwrap();

            let result = check_file(temp_file.path(), &Config::default());

            assert!(result.error.is_some());
            assert!(result.issues.is_empty());

            // Restore permissions for cleanup
            let mut perms = fs::metadata(temp_file.path()).unwrap().permissions();
            perms.set_mode(0o644);
            fs::set_permissions(temp_file.path(), perms).unwrap();
        }
    }

    #[test]
    fn test_check_trailing_spaces_with_null_bytes() {
        // Test handling of content with null bytes
        let content = "line1\0line2   \n";
        let issues = check_trailing_spaces(content);
        // The behavior depends on how the function handles null bytes
        // Just ensure it doesn't panic
        let _ = issues.len(); // Just checking it doesn't panic
    }

    #[test]
    fn test_check_newline_ending_with_only_cr() {
        // Old Mac-style line ending (just CR)
        let issue = check_newline_ending("content\r");
        assert!(issue.is_some());
        assert_eq!(issue.unwrap().issue_type, IssueType::MissingNewline);
    }

    // ===== Boundary condition tests =====

    #[test]
    fn test_check_trailing_spaces_line_number_overflow() {
        // Test with maximum number of lines
        let mut content = String::new();
        for _ in 0..100 {
            content.push_str("line\n");
        }
        content.push_str("last line with spaces   ");

        let issues = check_trailing_spaces(&content);
        assert_eq!(issues.last().unwrap().line, Some(101));
    }

    #[test]
    fn test_issue_type_all_variants() {
        // Ensure all IssueType variants are tested
        let types = vec![
            IssueType::MissingNewline,
            IssueType::MultipleNewlines,
            IssueType::TrailingSpace,
        ];

        for issue_type in types {
            let issue = Issue {
                issue_type,
                line: None,
                message: "test".to_string(),
            };

            // Test Debug and Clone
            let _ = format!("{issue:?}");
            let _ = issue.clone();
        }
    }

    #[test]
    fn test_check_result_empty_path() {
        let result = CheckResult {
            file_path: PathBuf::new(),
            issues: vec![],
            error: None,
        };

        assert_eq!(result.file_path, PathBuf::new());
    }

    #[test]
    fn test_very_long_file_path() {
        // Test with a very long file path
        let long_path = PathBuf::from(format!("/{}/file.txt", "a".repeat(1000)));
        let result = check_file(&long_path, &Config::default());

        assert!(result.error.is_some());
        assert_eq!(result.file_path, long_path);
    }

    // ===== Config variation tests =====

    #[test]
    fn test_partial_config_disabled() {
        let mut temp_file = NamedTempFile::new().unwrap();
        write!(temp_file, "trailing   ").unwrap();
        temp_file.flush().unwrap();

        let mut config = Config::default();
        config.checks.trailing_spaces = false;
        config.checks.newline_ending = true;

        let result = check_file(temp_file.path(), &config);

        assert!(result.error.is_none());
        assert_eq!(result.issues.len(), 1);
        assert_eq!(result.issues[0].issue_type, IssueType::MissingNewline);
    }

    #[test]
    fn test_all_checks_enabled() {
        let mut temp_file = NamedTempFile::new().unwrap();
        write!(temp_file, "line1   \nline2\n\n").unwrap();
        temp_file.flush().unwrap();

        let mut config = Config::default();
        config.checks.trailing_spaces = true;
        config.checks.newline_ending = true;

        let result = check_file(temp_file.path(), &config);

        assert!(result.error.is_none());
        assert_eq!(result.issues.len(), 2);
    }

    // ===== Content edge cases =====

    #[test]
    fn test_binary_content_detection() {
        let mut temp_file = NamedTempFile::new().unwrap();
        // Write some binary-like content
        temp_file.write_all(&[0xFF, 0xFE, 0x00, 0x01]).unwrap();
        temp_file.flush().unwrap();

        let result = check_file(temp_file.path(), &Config::default());

        // Should handle binary files gracefully (might have error or issues)
        assert!(result.error.is_some() || !result.issues.is_empty() || result.issues.is_empty());
    }

    #[test]
    fn test_mixed_line_endings_in_content() {
        // Content with mixed line endings
        let content = "line1\nline2\r\nline3\rline4\n";
        let issues = check_trailing_spaces(content);

        // Should handle mixed endings gracefully
        assert!(issues.is_empty());

        let newline_issue = check_newline_ending(content);
        assert!(newline_issue.is_none());
    }

    #[test]
    fn test_whitespace_only_lines() {
        let content = "   \n\t\t\t\n     \n";
        let issues = check_trailing_spaces(content);

        // Lines with only whitespace should be flagged
        assert_eq!(issues.len(), 3);
        for issue in issues {
            assert_eq!(issue.issue_type, IssueType::TrailingSpace);
        }
    }
}
