use crate::{CliArgs, Config};
use std::path::{Path, PathBuf};

pub fn discover_files(_args: &CliArgs) -> Result<Vec<PathBuf>, anyhow::Error> {
    todo!("Implement file discovery")
}

pub fn should_check_file(_path: &Path, _config: &Config) -> bool {
    todo!("Implement file filtering")
}
