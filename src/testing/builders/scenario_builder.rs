//! Test scenario builder for creating complex test cases
//!
//! This module provides a builder for creating test scenarios with multiple files
//! and expected outcomes for comprehensive testing.

use crate::checker::FileReader;
use crate::config::Config;
use crate::testing::builders::file_builder::TestFileBuilder;
use crate::testing::mocks::{MockFileSystem, MockOutput};
use std::collections::HashMap;
use std::path::PathBuf;

/// Issue type for expected issues in test scenarios
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TestIssue {
    MissingNewline { file: PathBuf },
    TrailingSpace { file: PathBuf, line: usize },
    TrailingTab { file: PathBuf, line: usize },
}

/// Result of running a test scenario
pub struct TestResult {
    pub passed: bool,
    pub message: Option<String>,
    pub actual_issues: Vec<TestIssue>,
    pub expected_issues: Vec<TestIssue>,
}

/// Builder for creating test scenarios
pub struct TestScenario {
    files: HashMap<PathBuf, String>,
    expected_issues: Vec<TestIssue>,
    config: Config,
}

impl TestScenario {
    /// Create a new test scenario
    pub fn new() -> Self {
        Self {
            files: HashMap::new(),
            expected_issues: Vec::new(),
            config: Config::default(),
        }
    }

    /// Add a file to the scenario using a TestFileBuilder
    pub fn with_file(mut self, builder: TestFileBuilder) -> Self {
        let (path, content) = builder.build();
        self.files.insert(path, content);
        self
    }

    /// Add a file with raw content
    pub fn with_file_content<P: Into<PathBuf>>(mut self, path: P, content: &str) -> Self {
        self.files.insert(path.into(), content.to_string());
        self
    }

    /// Set the configuration for the scenario
    pub fn with_config(mut self, config: Config) -> Self {
        self.config = config;
        self
    }

    /// Expect a missing newline issue
    pub fn expecting_missing_newline<P: Into<PathBuf>>(mut self, file: P) -> Self {
        self.expected_issues
            .push(TestIssue::MissingNewline { file: file.into() });
        self
    }

    /// Expect a trailing space issue
    pub fn expecting_trailing_space<P: Into<PathBuf>>(mut self, file: P, line: usize) -> Self {
        self.expected_issues.push(TestIssue::TrailingSpace {
            file: file.into(),
            line,
        });
        self
    }

    /// Expect a trailing tab issue
    pub fn expecting_trailing_tab<P: Into<PathBuf>>(mut self, file: P, line: usize) -> Self {
        self.expected_issues.push(TestIssue::TrailingTab {
            file: file.into(),
            line,
        });
        self
    }

    /// Expect no issues
    pub fn expecting_no_issues(mut self) -> Self {
        self.expected_issues.clear();
        self
    }

    /// Build a mock filesystem from the scenario
    pub fn build_mock_filesystem(&self) -> MockFileSystem {
        let mut fs = MockFileSystem::new();
        for (path, content) in &self.files {
            fs.add_file(path.clone(), content.clone());
        }
        fs
    }

    /// Build a mock output
    pub fn build_mock_output() -> MockOutput {
        MockOutput::new()
    }

    /// Get the files in the scenario
    pub fn files(&self) -> &HashMap<PathBuf, String> {
        &self.files
    }

    /// Get the expected issues
    pub fn expected_issues(&self) -> &[TestIssue] {
        &self.expected_issues
    }

    /// Get the configuration
    pub fn config(&self) -> &Config {
        &self.config
    }
}

impl Default for TestScenario {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scenario_builder_basic() {
        let scenario = TestScenario::new()
            .with_file(
                TestFileBuilder::new("test1.txt")
                    .with_line("hello")
                    .without_final_newline(),
            )
            .expecting_missing_newline("test1.txt");

        assert_eq!(scenario.files().len(), 1);
        assert_eq!(scenario.expected_issues().len(), 1);
    }

    #[test]
    fn test_scenario_builder_multiple_files() {
        let scenario = TestScenario::new()
            .with_file(TestFileBuilder::new("file1.txt").with_line("content1"))
            .with_file(TestFileBuilder::new("file2.txt").with_line("content2"))
            .with_file_content("file3.txt", "content3\n");

        assert_eq!(scenario.files().len(), 3);
        assert!(scenario.files().contains_key(&PathBuf::from("file1.txt")));
        assert!(scenario.files().contains_key(&PathBuf::from("file2.txt")));
        assert!(scenario.files().contains_key(&PathBuf::from("file3.txt")));
    }

    #[test]
    fn test_scenario_builder_with_issues() {
        let scenario = TestScenario::new()
            .with_file(
                TestFileBuilder::new("test.txt")
                    .with_line("line 1")
                    .with_trailing_spaces()
                    .with_trailing_tabs()
                    .without_final_newline(),
            )
            .expecting_trailing_space("test.txt", 2)
            .expecting_trailing_tab("test.txt", 3)
            .expecting_missing_newline("test.txt");

        assert_eq!(scenario.expected_issues().len(), 3);
    }

    #[test]
    fn test_scenario_builder_mock_filesystem() {
        let scenario = TestScenario::new().with_file_content("test.txt", "hello world");

        let fs = scenario.build_mock_filesystem();
        let content = fs.read_to_string(&PathBuf::from("test.txt")).unwrap();
        assert_eq!(content, "hello world");
    }

    #[test]
    fn test_scenario_builder_no_issues() {
        let scenario = TestScenario::new()
            .with_file(TestFileBuilder::new("good.txt").with_line("perfect"))
            .expecting_trailing_space("good.txt", 1)
            .expecting_no_issues(); // This clears all expected issues

        assert_eq!(scenario.expected_issues().len(), 0);
    }

    #[test]
    fn test_scenario_builder_custom_config() {
        let mut config = Config::default();
        config.checks.trailing_spaces = false;

        let scenario = TestScenario::new().with_config(config.clone());

        assert!(!scenario.config().checks.trailing_spaces);
    }
}
