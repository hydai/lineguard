use crate::CheckResult;

pub trait Reporter {
    fn report(&self, results: &[CheckResult]);
}

pub struct HumanReporter {
    pub use_color: bool,
}

pub struct JsonReporter;

pub struct GitHubReporter;

impl Reporter for HumanReporter {
    fn report(&self, _results: &[CheckResult]) {
        todo!("Implement human-readable reporting")
    }
}

impl Reporter for JsonReporter {
    fn report(&self, _results: &[CheckResult]) {
        todo!("Implement JSON reporting")
    }
}

impl Reporter for GitHubReporter {
    fn report(&self, _results: &[CheckResult]) {
        todo!("Implement GitHub Actions reporting")
    }
}
