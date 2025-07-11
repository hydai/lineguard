use crate::{CliArgs, Config};
use glob::glob;
use std::fs;
use std::io::{self, BufRead, BufReader};
use std::path::{Path, PathBuf};

pub fn discover_files(args: &CliArgs) -> Result<Vec<PathBuf>, anyhow::Error> {
    let mut files = Vec::new();

    if args.stdin {
        // Read file paths from stdin
        let stdin = io::stdin();
        let reader = BufReader::new(stdin.lock());

        for line in reader.lines() {
            let line = line?;
            let line = line.trim();
            if !line.is_empty() {
                let path = PathBuf::from(line);
                if path.exists() && path.is_file() {
                    // Load config to check if file should be processed
                    let config = Config::default();
                    if should_check_file(&path, &config) {
                        files.push(path);
                    }
                }
            }
        }
    } else {
        // Process files from command line arguments
        for pattern in &args.files {
            let path = PathBuf::from(pattern);

            // Check if it's a directory
            if path.is_dir() {
                discover_files_in_dir(&path, args.recursive, &mut files)?;
            } else {
                // Try glob pattern first
                if let Ok(paths) = glob(pattern) {
                    let mut found_any = false;
                    let config = Config::default();
                    for path in paths.flatten() {
                        if path.is_file() && should_check_file(&path, &config) {
                            files.push(path);
                            found_any = true;
                        }
                    }

                    // If glob didn't find anything, try as literal path
                    if !found_any {
                        let path = PathBuf::from(pattern);
                        if path.exists() && path.is_file() {
                            let config = Config::default();
                            if should_check_file(&path, &config) {
                                files.push(path);
                            }
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
    }

    Ok(files)
}

fn discover_files_in_dir(
    dir: &Path,
    recursive: bool,
    files: &mut Vec<PathBuf>,
) -> Result<(), anyhow::Error> {
    let config = Config::default();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && should_check_file(&path, &config) {
            files.push(path);
        } else if path.is_dir() && recursive {
            discover_files_in_dir(&path, recursive, files)?;
        }
    }

    Ok(())
}

pub fn should_check_file(path: &Path, config: &Config) -> bool {
    // Skip hidden files (starting with .)
    if let Some(file_name) = path.file_name() {
        if let Some(name_str) = file_name.to_str() {
            if name_str.starts_with('.') {
                return false;
            }
        }
    }

    // Get file extension
    let extension = match path.extension() {
        Some(ext) => match ext.to_str() {
            Some(ext_str) => ext_str.to_lowercase(),
            None => return true, // If can't convert to string, assume it's checkable
        },
        None => return true, // No extension, check it
    };

    // Skip common binary file extensions
    const BINARY_EXTENSIONS: &[&str] = &[
        "jpg", "jpeg", "png", "gif", "bmp", "ico", "svg", "webp", // Images
        "mp3", "mp4", "avi", "mov", "wmv", "flv", "webm", // Audio/Video
        "zip", "tar", "gz", "bz2", "xz", "7z", "rar", // Archives
        "exe", "dll", "so", "dylib", "a", "o", // Executables/Libraries
        "bin", "dat", "db", "sqlite", // Binary data
        "pdf", "doc", "docx", "xls", "xlsx", // Documents
        "class", "jar", "war", // Java
        "pyc", "pyo", // Python
        "woff", "woff2", "ttf", "otf", "eot", // Fonts
    ];

    if BINARY_EXTENSIONS.contains(&extension.as_str()) {
        return false;
    }

    // Check against configured extensions if any
    if !config.file_extensions.is_empty() {
        return config.file_extensions.contains(&extension);
    }

    true
}
