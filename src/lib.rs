pub mod checker;
pub mod cli;
pub mod config;
pub mod discovery;
pub mod fixer;
pub mod git;
pub mod reporter;

#[cfg(test)]
pub mod testing;

// Re-export specific items to avoid conflicts
pub use checker::{
    CheckResult, FileChecker, Issue, IssueType, StdFileReader, check_file, check_newline_ending,
    check_trailing_spaces,
};
pub use cli::*;
pub use config::*;
pub use discovery::*;
pub use reporter::{
    Color, ColoredOutput, GitHubReporter, HumanReporter, JsonReporter, Output, Reporter,
    ReporterWithOutput, StdOutput,
};
