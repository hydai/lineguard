pub mod github;
pub mod human;
pub mod json;
pub mod traits;

use crate::CheckResult;

// Re-export traits
pub use traits::{Color, ColoredOutput, Output, ReporterWithOutput, StdOutput};

// Re-export reporters
pub use github::GitHubReporter;
pub use human::HumanReporter;
pub use json::JsonReporter;

pub trait Reporter {
    fn report(&self, results: &[CheckResult]);
}
