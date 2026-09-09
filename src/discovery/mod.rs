use crate::git;
use crate::{CliArgs, Config};
use glob::{Pattern, glob};
use ignore::WalkBuilder;
use std::io::{self, BufRead, BufReader};
use std::path::{Component, Path, PathBuf};

pub struct DiscoveryResult {
    pub files: Vec<PathBuf>,
    pub git_range: Option<GitRangeInfo>,
}

pub struct GitRangeInfo {
    pub from: String,
    pub to: String,
    pub changed_files: Vec<PathBuf>,
}

/// Pre-compile glob patterns once for efficient reuse.
fn compile_ignore_patterns(patterns: &[String]) -> Vec<Pattern> {
    patterns
        .iter()
        .filter_map(|s| Pattern::new(s).ok())
        .collect()
}

pub fn discover_files(
    args: &CliArgs,
    base_config: &Config,
) -> Result<DiscoveryResult, anyhow::Error> {
    let mut files = Vec::new();
    let mut git_range_info = None;

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

    // Pre-compile ignore patterns once for performance
    let ignore_patterns = compile_ignore_patterns(&config.ignore_patterns);

    // CLI flag disables .gitignore support configured in the config file
    let respect_gitignore = config.respect_gitignore && !args.no_gitignore;

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
                    && !is_ignored(&path, &ignore_patterns)
                    && !(args.no_hidden && is_hidden_file(&path))
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
                discover_files_in_dir(
                    &path,
                    args.recursive,
                    &mut files,
                    &config,
                    args.no_hidden,
                    &ignore_patterns,
                    respect_gitignore,
                )?;
            } else {
                // Try glob pattern first
                if let Ok(paths) = glob(pattern) {
                    let mut found_any = false;
                    for path in paths.flatten() {
                        if path.is_file()
                            && should_check_file(&path, &config)
                            && !is_ignored(&path, &ignore_patterns)
                            && !(args.no_hidden && is_hidden_file(&path))
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
                            && !is_ignored(&path, &ignore_patterns)
                            && !(args.no_hidden && is_hidden_file(&path))
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
                        && !is_ignored(&path, &ignore_patterns)
                        && !(args.no_hidden && is_hidden_file(&path))
                    {
                        files.push(path);
                    }
                }
            }
        }
    }

    // If --from is specified, filter to only git-changed files
    if let Some(from_commit) = &args.from {
        // Get the current working directory for git operations
        let cwd = std::env::current_dir()?;

        // Get list of changed files from git
        let to_commit = args.to.as_deref().unwrap_or("HEAD");
        let changed_files = git::get_changed_files(from_commit, args.to.as_deref(), &cwd)?;

        // Store git range info
        git_range_info = Some(GitRangeInfo {
            from: from_commit.clone(),
            to: to_commit.to_string(),
            changed_files: changed_files.clone(),
        });

        // Filter discovered files to only include changed files
        files.retain(|file| {
            // Convert to absolute path for comparison
            let abs_file = if file.is_absolute() {
                file.clone()
            } else {
                cwd.join(file)
            };

            changed_files.iter().any(|changed| {
                let abs_changed = if changed.is_absolute() {
                    changed.clone()
                } else {
                    cwd.join(changed)
                };
                abs_file == abs_changed
            })
        });
    }

    Ok(DiscoveryResult {
        files,
        git_range: git_range_info,
    })
}

fn discover_files_in_dir(
    dir: &Path,
    recursive: bool,
    files: &mut Vec<PathBuf>,
    config: &Config,
    no_hidden: bool,
    ignore_patterns: &[Pattern],
    respect_gitignore: bool,
) -> Result<(), anyhow::Error> {
    let mut builder = WalkBuilder::new(dir);
    builder
        // Opt out of all default filters, then enable only what lineguard needs
        .standard_filters(false)
        // WalkBuilder's hidden(true) means "skip hidden entries"
        .hidden(no_hidden)
        // Match previous behavior: symlinked files and directories are followed
        .follow_links(true)
        // .gitignore in the walked tree and in parent directories of the root
        .git_ignore(respect_gitignore)
        .parents(respect_gitignore)
        // .git/info/exclude
        .git_exclude(respect_gitignore)
        // Only apply git ignore rules inside an actual git repository,
        // and keep results machine-independent (no global gitignore)
        .require_git(true)
        .git_global(false);

    if !recursive {
        builder.max_depth(Some(1));
    }

    if respect_gitignore {
        // git itself never tracks the .git directory; skip its contents too
        builder.filter_entry(|entry| entry.file_name() != std::ffi::OsStr::new(".git"));
    }

    for result in builder.build() {
        let entry = match result {
            Ok(entry) => entry,
            Err(e) => {
                eprintln!("{e}");
                continue;
            },
        };
        let path = entry.path();

        // is_ignored also matches ancestor directories, so files inside
        // ignored directories are excluded just like before
        if path.is_file() && should_check_file(path, config) && !is_ignored(path, ignore_patterns) {
            files.push(path.to_path_buf());
        }
    }

    Ok(())
}

fn is_hidden_file(path: &Path) -> bool {
    if let Some(file_name) = path.file_name()
        && let Some(name_str) = file_name.to_str()
    {
        return name_str.starts_with('.');
    }
    false
}

pub fn should_check_file(path: &Path, config: &Config) -> bool {
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

fn normalize_path(path: &Path) -> PathBuf {
    let mut components = Vec::new();
    for component in path.components() {
        if component == Component::CurDir {
            continue;
        }

        if component == Component::ParentDir
            && let Some(Component::Normal(_)) = components.last()
        {
            components.pop();
            continue;
        }

        components.push(component);
    }
    if components.is_empty() {
        return PathBuf::from(".");
    }
    components.iter().collect()
}

fn is_ignored(path: &Path, ignore_patterns: &[Pattern]) -> bool {
    if ignore_patterns.is_empty() {
        return false;
    }

    let normalized_path_buf = normalize_path(path);
    let normalized_path = normalized_path_buf.as_path();

    for pattern in ignore_patterns {
        // Check normalized relative path directly
        if pattern.matches_path(normalized_path) {
            return true;
        }

        // For filename-only patterns (no path separator), check every path
        // component (the file name and each ancestor directory name), so that
        // e.g. `--ignore target` excludes files inside any `target` directory
        // whether the discovered path is relative or absolute.
        // Use matches_path with the component as a Path to maintain proper path semantics.
        // Check both '/' and '\\' explicitly since glob patterns may use either separator
        // regardless of platform, and users might write patterns with either style.
        let pattern_str = pattern.as_str();
        if !pattern_str.contains('/') && !pattern_str.contains('\\') {
            for component in normalized_path.components() {
                if let Component::Normal(name) = component
                    && pattern.matches_path(Path::new(name))
                {
                    return true;
                }
            }
        }

        // Skip the path itself (already checked above) and check its parent directories.
        for ancestor in normalized_path.ancestors().skip(1) {
            if !ancestor.as_os_str().is_empty() && pattern.matches_path(ancestor) {
                return true;
            }
        }
    }

    false
}
