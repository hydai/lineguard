//! Test fixtures for LineLint
//!
//! This module provides pre-defined test data and helper functions
//! for common testing scenarios.

/// A simple file with proper line endings
pub const SIMPLE_FILE: &str = "line 1\nline 2\nline 3\n";

/// A file with trailing spaces on some lines
pub const FILE_WITH_TRAILING_SPACES: &str = "line 1   \nline 2\nline 3  \n";

/// A file missing the final newline
pub const FILE_WITHOUT_NEWLINE: &str = "line 1\nline 2";

/// An empty file
pub const EMPTY_FILE: &str = "";

/// A file with mixed line endings (CRLF and LF)
pub const MIXED_LINE_ENDINGS: &str = "line 1\r\nline 2\nline 3\r\n";

/// Generate a large file with the specified number of lines
pub fn generate_large_file(lines: usize) -> String {
    (0..lines).map(|i| format!("line {i}\n")).collect()
}

/// Generate a file with trailing spaces on every nth line
pub fn generate_file_with_trailing_spaces(lines: usize, every_nth: usize) -> String {
    (0..lines)
        .map(|i| {
            if i % every_nth == 0 {
                format!("line {i}   \n")
            } else {
                format!("line {i}\n")
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_large_file() {
        let content = generate_large_file(3);
        assert_eq!(content, "line 0\nline 1\nline 2\n");
    }

    #[test]
    fn test_generate_file_with_trailing_spaces() {
        let content = generate_file_with_trailing_spaces(3, 2);
        assert_eq!(content, "line 0   \nline 1\nline 2   \n");
    }
}
