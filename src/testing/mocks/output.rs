//! Mock implementation of output operations
//!
//! This module provides mock output implementations for testing purposes,
//! allowing tests to capture and verify output without writing to stdout/stderr.

use crate::reporter::{Color, ColoredOutput, Output};
use std::io;

/// Mock output implementation that captures all output
#[derive(Default)]
pub struct MockOutput {
    /// Buffer to store all written content
    pub buffer: Vec<String>,
    /// Record of colored output calls
    pub color_calls: Vec<(String, Color)>,
    /// Whether to simulate an error
    simulate_error: bool,
}

impl MockOutput {
    /// Create a new mock output
    pub fn new() -> Self {
        Self::default()
    }

    /// Simulate an I/O error on the next write operation
    pub fn simulate_error(&mut self) {
        self.simulate_error = true;
    }

    /// Get the full output as a single string
    pub fn get_output(&self) -> String {
        self.buffer.join("")
    }

    /// Get the output lines
    pub fn get_lines(&self) -> Vec<&str> {
        self.buffer.iter().map(|s| s.as_str()).collect()
    }

    /// Clear the buffer
    pub fn clear(&mut self) {
        self.buffer.clear();
        self.color_calls.clear();
    }

    /// Check if specific content was written
    pub fn contains(&self, content: &str) -> bool {
        self.get_output().contains(content)
    }

    /// Check if specific colored content was written
    pub fn contains_colored(&self, content: &str, color: Color) -> bool {
        self.color_calls
            .iter()
            .any(|(text, c)| text.contains(content) && *c == color)
    }

    /// Check if any colored output was made
    pub fn has_color_output(&self) -> bool {
        !self.color_calls.is_empty()
    }
}

impl Output for MockOutput {
    fn write(&mut self, content: &str) -> io::Result<()> {
        if self.simulate_error {
            self.simulate_error = false;
            return Err(io::Error::other("Simulated error"));
        }
        self.buffer.push(content.to_string());
        Ok(())
    }

    fn flush(&mut self) -> io::Result<()> {
        if self.simulate_error {
            self.simulate_error = false;
            return Err(io::Error::other("Simulated flush error"));
        }
        Ok(())
    }
}

impl ColoredOutput for MockOutput {
    fn write_colored(&mut self, content: &str, color: Color) -> io::Result<()> {
        if self.simulate_error {
            self.simulate_error = false;
            return Err(io::Error::other("Simulated error"));
        }
        self.color_calls.push((content.to_string(), color));
        self.buffer.push(content.to_string());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_output_capture() {
        let mut output = MockOutput::new();
        output.write_line("test").unwrap();
        assert_eq!(output.buffer, vec!["test", "\n"]);
    }

    #[test]
    fn test_mock_output_write() {
        let mut output = MockOutput::new();
        output.write("hello").unwrap();
        output.write(" ").unwrap();
        output.write("world").unwrap();
        assert_eq!(output.get_output(), "hello world");
    }

    #[test]
    fn test_mock_output_write_line() {
        let mut output = MockOutput::new();
        output.write_line("line 1").unwrap();
        output.write_line("line 2").unwrap();
        assert_eq!(output.get_output(), "line 1\nline 2\n");
        assert_eq!(output.get_lines(), vec!["line 1", "\n", "line 2", "\n"]);
    }

    #[test]
    fn test_mock_output_colored() {
        let mut output = MockOutput::new();
        output.write_colored("error", Color::Red).unwrap();
        output.write_colored("success", Color::Green).unwrap();

        assert_eq!(output.get_output(), "errorsuccess");
        assert_eq!(output.color_calls.len(), 2);
        assert_eq!(output.color_calls[0], ("error".to_string(), Color::Red));
        assert_eq!(output.color_calls[1], ("success".to_string(), Color::Green));
    }

    #[test]
    fn test_mock_output_colored_line() {
        let mut output = MockOutput::new();
        output.write_line_colored("warning", Color::Yellow).unwrap();

        assert_eq!(output.get_output(), "warning\n");
        assert!(output.contains_colored("warning", Color::Yellow));
    }

    #[test]
    fn test_mock_output_error_simulation() {
        let mut output = MockOutput::new();
        output.simulate_error();

        let result = output.write("test");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::Other);

        // Error should be cleared after one operation
        let result2 = output.write("test");
        assert!(result2.is_ok());
    }

    #[test]
    fn test_mock_output_flush() {
        let mut output = MockOutput::new();
        assert!(output.flush().is_ok());

        output.simulate_error();
        let result = output.flush();
        assert!(result.is_err());
    }

    #[test]
    fn test_mock_output_clear() {
        let mut output = MockOutput::new();
        output.write_line("test").unwrap();
        output.write_colored("colored", Color::Blue).unwrap();

        assert!(!output.buffer.is_empty());
        assert!(!output.color_calls.is_empty());

        output.clear();
        assert!(output.buffer.is_empty());
        assert!(output.color_calls.is_empty());
    }

    #[test]
    fn test_mock_output_contains() {
        let mut output = MockOutput::new();
        output.write_line("hello world").unwrap();
        output.write_line("goodbye").unwrap();

        assert!(output.contains("hello"));
        assert!(output.contains("world"));
        assert!(output.contains("goodbye"));
        assert!(!output.contains("missing"));
    }
}
