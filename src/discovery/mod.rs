use crate::{CliArgs, Config};
use glob::glob;
use std::fs;
use std::path::{Path, PathBuf};

pub fn discover_files(args: &CliArgs) -> Result<Vec<PathBuf>, anyhow::Error> {
    let mut files = Vec::new();

    for pattern in &args.files {
        let path = PathBuf::from(pattern);

        // Check if it's a directory
        if path.is_dir() {
            discover_files_in_dir(&path, args.recursive, &mut files)?;
        } else {
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
    }

    Ok(files)
}

fn discover_files_in_dir(
    dir: &Path,
    recursive: bool,
    files: &mut Vec<PathBuf>,
) -> Result<(), anyhow::Error> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            files.push(path);
        } else if path.is_dir() && recursive {
            discover_files_in_dir(&path, recursive, files)?;
        }
    }

    Ok(())
}

pub fn should_check_file(_path: &Path, _config: &Config) -> bool {
    todo!("Implement file filtering")
}
