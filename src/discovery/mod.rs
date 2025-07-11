use crate::{CliArgs, Config};
use glob::glob;
use std::path::{Path, PathBuf};

pub fn discover_files(args: &CliArgs) -> Result<Vec<PathBuf>, anyhow::Error> {
    let mut files = Vec::new();

    for pattern in &args.files {
        // Try glob pattern first
        if let Ok(paths) = glob(pattern) {
            let mut found_any = false;
            for path in paths.flatten() {
                if path.is_file() {
                    files.push(path);
                    found_any = true;
                }
            }

            // If glob didn't find anything, try as literal path
            if !found_any {
                let path = PathBuf::from(pattern);
                if path.exists() && path.is_file() {
                    files.push(path);
                }
            }
        } else {
            // If glob pattern is invalid, treat as literal path
            let path = PathBuf::from(pattern);
            if path.exists() && path.is_file() {
                files.push(path);
            }
        }
    }

    Ok(files)
}

pub fn should_check_file(_path: &Path, _config: &Config) -> bool {
    todo!("Implement file filtering")
}
