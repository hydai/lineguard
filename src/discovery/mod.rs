use crate::{CliArgs, Config};
use glob::{Pattern, glob};
use std::fs;
use std::io::{self, BufRead, BufReader};
use std::path::{Path, PathBuf};

pub fn discover_files(args: &CliArgs, base_config: &Config) -> Result<Vec<PathBuf>, anyhow::Error> {
    let mut files = Vec::new();

    // Merge CLI arguments with config file settings (CLI takes precedence)
    let mut config = base_config.clone();

    // CLI ignore patterns override/extend config patterns
    if !args.ignore.is_empty() {
        config.ignore_patterns = args.ignore.clone();
    }

    // CLI extensions override config extensions
    if let Some(ref extensions) = args.extensions {
        config.file_extensions = extensions.clone();
    }

    if args.stdin {
        // Read file paths from stdin
        let stdin = io::stdin();
        let reader = BufReader::new(stdin.lock());

        for line in reader.lines() {
            let line = line?;
            let line = line.trim();
            if !line.is_empty() {
                let path = PathBuf::from(line);
                if path.exists()
                    && path.is_file()
                    && should_check_file(&path, &config)
                    && !is_ignored(&path, &config.ignore_patterns)?
                {
                    files.push(path);
                }
            }
        }
    } else {
        // Process files from command line arguments
        for pattern in &args.files {
            let path = PathBuf::from(pattern);

            // Check if it's a directory
            if path.is_dir() {
                discover_files_in_dir(&path, args.recursive, &mut files, &config)?;
            } else {
                // Try glob pattern first
                if let Ok(paths) = glob(pattern) {
                    let mut found_any = false;
                    for path in paths.flatten() {
                        if path.is_file()
                            && should_check_file(&path, &config)
                            && !is_ignored(&path, &config.ignore_patterns)?
                        {
                            files.push(path);
                            found_any = true;
                        }
                    }

                    // If glob didn't find anything, try as literal path
                    if !found_any {
                        let path = PathBuf::from(pattern);
                        if path.exists()
                            && path.is_file()
                            && should_check_file(&path, &config)
                            && !is_ignored(&path, &config.ignore_patterns)?
                        {
                            files.push(path);
                        }
                    }
                } else {
                    // If glob pattern is invalid, treat as literal path
                    let path = PathBuf::from(pattern);
                    if path.exists()
                        && path.is_file()
                        && should_check_file(&path, &config)
                        && !is_ignored(&path, &config.ignore_patterns)?
                    {
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
    config: &Config,
) -> Result<(), anyhow::Error> {
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!("{}: {}", dir.display(), e);
            return Ok(()); // Continue with other directories
        },
    };

    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(e) => {
                eprintln!("Error reading directory entry: {e}");
                continue;
            },
        };
        let path = entry.path();

        if path.is_file()
            && should_check_file(&path, config)
            && !is_ignored(&path, &config.ignore_patterns)?
        {
            files.push(path);
        } else if path.is_dir() && recursive && !is_ignored(&path, &config.ignore_patterns)? {
            discover_files_in_dir(&path, recursive, files, config)?;
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

fn is_ignored(path: &Path, ignore_patterns: &[String]) -> Result<bool, anyhow::Error> {
    if ignore_patterns.is_empty() {
        return Ok(false);
    }

    for pattern_str in ignore_patterns {
        // Check if any parent directory matches the pattern
        let mut current_path = path;
        loop {
            // Try to compile as a glob pattern
            if let Ok(pattern) = Pattern::new(pattern_str) {
                // Check absolute path
                if pattern.matches_path(current_path) {
                    return Ok(true);
                }

                // Check relative path from current directory
                if let Ok(current_dir) = std::env::current_dir() {
                    if let Ok(relative_path) = current_path.strip_prefix(&current_dir) {
                        if pattern.matches_path(relative_path) {
                            return Ok(true);
                        }

                        // Also check if pattern matches any parent component
                        let relative_str = relative_path.to_string_lossy();
                        if pattern.matches(&relative_str) {
                            return Ok(true);
                        }
                    }
                }

                // Check just the filename
                if let Some(file_name) = current_path.file_name() {
                    if let Some(file_name_str) = file_name.to_str() {
                        if pattern.matches(file_name_str) {
                            return Ok(true);
                        }
                    }
                }
            }

            // Move to parent directory
            match current_path.parent() {
                Some(parent) if parent != current_path => current_path = parent,
                _ => break,
            }
        }
    }

    Ok(false)
}
