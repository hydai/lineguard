//! Traits for reporter module to support dependency injection and testing
//!
//! This module defines the core traits used by reporters to enable
//! testability and flexibility in output handling.

use std::io;

/// Color enum for colored output
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Black,
}

/// Trait for output operations
///
/// This trait allows reporters to write output to different destinations
/// (stdout, files, memory buffers for testing, etc.)
pub trait Output: Send + Sync {
    /// Write content to output
    fn write(&mut self, content: &str) -> io::Result<()>;

    /// Write content with a newline
    fn write_line(&mut self, content: &str) -> io::Result<()> {
        self.write(content)?;
        self.write("\n")
    }

    /// Flush the output
    fn flush(&mut self) -> io::Result<()>;
}

/// Trait for colored output operations
pub trait ColoredOutput: Output {
    /// Write colored content
    fn write_colored(&mut self, content: &str, color: Color) -> io::Result<()>;

    /// Write colored content with a newline
    fn write_line_colored(&mut self, content: &str, color: Color) -> io::Result<()> {
        self.write_colored(content, color)?;
        self.write("\n")
    }
}

/// Standard output implementation
pub struct StdOutput {
    use_color: bool,
}

impl StdOutput {
    /// Create a new standard output
    pub fn new() -> Self {
        Self { use_color: false }
    }

    /// Create a new standard output with color support
    pub fn with_color() -> Self {
        Self { use_color: true }
    }
}

impl Default for StdOutput {
    fn default() -> Self {
        Self::new()
    }
}

impl Output for StdOutput {
    fn write(&mut self, content: &str) -> io::Result<()> {
        print!("{content}");
        Ok(())
    }

    fn flush(&mut self) -> io::Result<()> {
        use std::io::Write;
        io::stdout().flush()
    }
}

impl ColoredOutput for StdOutput {
    fn write_colored(&mut self, content: &str, color: Color) -> io::Result<()> {
        if self.use_color {
            use colored::Colorize;
            let colored_text = match color {
                Color::Red => content.red().to_string(),
                Color::Green => content.green().to_string(),
                Color::Yellow => content.yellow().to_string(),
                Color::Blue => content.blue().to_string(),
                Color::Magenta => content.magenta().to_string(),
                Color::Cyan => content.cyan().to_string(),
                Color::White => content.white().to_string(),
                Color::Black => content.black().to_string(),
            };
            print!("{colored_text}");
        } else {
            print!("{content}");
        }
        Ok(())
    }
}

/// Reporter trait with output dependency injection
///
/// This trait extends the basic Reporter functionality to support
/// different output destinations for better testability.
pub trait ReporterWithOutput {
    /// Report results to the provided output
    fn report_to(&self, results: &[crate::CheckResult], output: &mut dyn Output) -> io::Result<()>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_std_output() {
        let mut output = StdOutput::new();
        // Basic functionality test - just ensure it doesn't panic
        assert!(output.write("test").is_ok());
        assert!(output.write_line("test line").is_ok());
        assert!(output.flush().is_ok());
    }

    #[test]
    fn test_std_output_colored() {
        let mut output = StdOutput::with_color();
        assert!(output.write_colored("red text", Color::Red).is_ok());
        assert!(
            output
                .write_line_colored("green line", Color::Green)
                .is_ok()
        );
    }

    #[test]
    fn test_std_output_no_color() {
        let mut output = StdOutput::new();
        // Should just print without color
        assert!(output.write_colored("plain text", Color::Blue).is_ok());
    }

    #[test]
    fn test_std_output_default() {
        let output = StdOutput::default();
        assert!(!output.use_color);
    }

    #[test]
    fn test_std_output_all_colors() {
        let mut output = StdOutput::with_color();

        // Test all color variants
        assert!(output.write_colored("Red", Color::Red).is_ok());
        assert!(output.write_colored("Green", Color::Green).is_ok());
        assert!(output.write_colored("Yellow", Color::Yellow).is_ok());
        assert!(output.write_colored("Blue", Color::Blue).is_ok());
        assert!(output.write_colored("Magenta", Color::Magenta).is_ok());
        assert!(output.write_colored("Cyan", Color::Cyan).is_ok());
        assert!(output.write_colored("White", Color::White).is_ok());
        assert!(output.write_colored("Black", Color::Black).is_ok());

        // Test with color disabled
        let mut no_color_output = StdOutput::new();
        assert!(no_color_output.write_colored("Plain", Color::Red).is_ok());
    }

    #[test]
    fn test_std_output_basic_operations() {
        let mut output = StdOutput::new();
        // Test basic write operations
        assert!(output.write("Test").is_ok());
        assert!(output.write_line("Test line").is_ok());
        assert!(output.flush().is_ok());
    }
}
