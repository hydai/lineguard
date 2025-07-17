//! Test file builder for creating test files with specific characteristics
//!
//! This module provides a fluent API for building test files with various
//! line ending and whitespace issues for testing purposes.

use std::path::{Path, PathBuf};

/// Builder for creating test files with specific content patterns
#[derive(Default)]
pub struct TestFileBuilder {
    path: PathBuf,
    lines: Vec<String>,
    add_final_newline: bool,
    line_ending: LineEnding,
}

/// Line ending style
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LineEnding {
    #[default]
    Lf, // Unix style \n
    Crlf, // Windows style \r\n
}

impl TestFileBuilder {
    /// Create a new test file builder with the given path
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            lines: Vec::new(),
            add_final_newline: true,
            line_ending: LineEnding::default(),
        }
    }

    /// Add a line to the file
    pub fn with_line(mut self, line: &str) -> Self {
        self.lines.push(line.to_string());
        self
    }

    /// Add multiple lines to the file
    pub fn with_lines(mut self, lines: &[&str]) -> Self {
        for line in lines {
            self.lines.push(line.to_string());
        }
        self
    }

    /// Add a line with trailing spaces
    pub fn with_trailing_spaces(mut self) -> Self {
        self.lines.push("line with trailing spaces   ".to_string());
        self
    }

    /// Add a line with trailing tabs
    pub fn with_trailing_tabs(mut self) -> Self {
        self.lines.push("line with trailing tabs\t\t".to_string());
        self
    }

    /// Add an empty line
    pub fn with_empty_line(mut self) -> Self {
        self.lines.push(String::new());
        self
    }

    /// Configure the file to not have a final newline
    pub fn without_final_newline(mut self) -> Self {
        self.add_final_newline = false;
        self
    }

    /// Use CRLF line endings (Windows style)
    pub fn with_crlf_endings(mut self) -> Self {
        self.line_ending = LineEnding::Crlf;
        self
    }

    /// Use LF line endings (Unix style) - this is the default
    pub fn with_lf_endings(mut self) -> Self {
        self.line_ending = LineEnding::Lf;
        self
    }

    /// Build the file content and return the path and content
    pub fn build(self) -> (PathBuf, String) {
        let line_ending = match self.line_ending {
            LineEnding::Lf => "\n",
            LineEnding::Crlf => "\r\n",
        };

        let mut content = String::new();

        // Add all lines with appropriate endings
        for (index, line) in self.lines.iter().enumerate() {
            content.push_str(line);

            // Add line ending unless it's the last line and we don't want a final newline
            if index < self.lines.len() - 1 || self.add_final_newline {
                content.push_str(line_ending);
            }
        }

        (self.path, content)
    }

    /// Create a test file with common issues
    pub fn with_common_issues(self) -> Self {
        self.with_line("normal line")
            .with_trailing_spaces()
            .with_empty_line()
            .with_trailing_tabs()
            .with_line("another normal line")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_builder() {
        let (path, content) = TestFileBuilder::new("test.txt")
            .with_line("line 1")
            .with_trailing_spaces()
            .build();

        assert_eq!(path, PathBuf::from("test.txt"));
        assert!(content.contains("line 1\n"));
        assert!(content.contains("   \n"));
    }

    #[test]
    fn test_file_builder_basic() {
        let (path, content) = TestFileBuilder::new("basic.txt")
            .with_line("hello")
            .with_line("world")
            .build();

        assert_eq!(path, PathBuf::from("basic.txt"));
        assert_eq!(content, "hello\nworld\n");
    }

    #[test]
    fn test_file_builder_without_final_newline() {
        let (_, content) = TestFileBuilder::new("test.txt")
            .with_line("line 1")
            .with_line("line 2")
            .without_final_newline()
            .build();

        assert_eq!(content, "line 1\nline 2");
    }

    #[test]
    fn test_file_builder_with_crlf() {
        let (_, content) = TestFileBuilder::new("test.txt")
            .with_line("line 1")
            .with_line("line 2")
            .with_crlf_endings()
            .build();

        assert_eq!(content, "line 1\r\nline 2\r\n");
    }

    #[test]
    fn test_file_builder_trailing_whitespace() {
        let (_, content) = TestFileBuilder::new("test.txt")
            .with_trailing_spaces()
            .with_trailing_tabs()
            .build();

        assert!(content.contains("   \n"));
        assert!(content.contains("\t\t\n"));
    }

    #[test]
    fn test_file_builder_empty_lines() {
        let (_, content) = TestFileBuilder::new("test.txt")
            .with_line("before")
            .with_empty_line()
            .with_line("after")
            .build();

        assert_eq!(content, "before\n\nafter\n");
    }

    #[test]
    fn test_file_builder_with_lines() {
        let (_, content) = TestFileBuilder::new("test.txt")
            .with_lines(&["line 1", "line 2", "line 3"])
            .build();

        assert_eq!(content, "line 1\nline 2\nline 3\n");
    }

    #[test]
    fn test_file_builder_common_issues() {
        let (_, content) = TestFileBuilder::new("test.txt")
            .with_common_issues()
            .build();

        assert!(content.contains("normal line\n"));
        assert!(content.contains("   \n"));
        assert!(content.contains("\t\t\n"));
        assert!(content.contains("\n\n")); // empty line
    }
}
