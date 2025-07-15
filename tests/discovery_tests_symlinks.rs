use lineguard::cli::CliArgs;
use lineguard::config::Config;
use lineguard::discovery::discover_files;
use tempfile::TempDir;

#[test]
#[cfg(unix)]
fn test_discover_files_with_symlink() {
    use std::os::unix::fs::symlink;

    let temp_dir = TempDir::new().unwrap();
    let real_file = temp_dir.path().join("real_file.txt");
    let symlink_file = temp_dir.path().join("symlink_file.txt");

    std::fs::write(&real_file, "real content").unwrap();
    symlink(&real_file, &symlink_file).unwrap();

    let args = CliArgs {
        files: vec![symlink_file.to_string_lossy().to_string()],
        stdin: false,
        recursive: false,
        format: lineguard::cli::OutputFormat::Human,
        quiet: false,
        verbose: false,
        no_color: false,
        config: None,
        ignore: vec![],
        extensions: None,
        no_newline_check: false,
        no_trailing_space: false,
        fix: false,
        dry_run: false,
        from: None,
        to: None,
        no_hidden: false,
    };

    let config = Config::default();
    let result = discover_files(&args, &config).unwrap();
    assert_eq!(result.files.len(), 1);
    assert_eq!(result.files[0], symlink_file);
}

#[test]
#[cfg(unix)]
fn test_discover_files_with_symlink_directory() {
    use std::os::unix::fs::symlink;

    let temp_dir = TempDir::new().unwrap();
    let real_dir = temp_dir.path().join("real_dir");
    let symlink_dir = temp_dir.path().join("symlink_dir");

    std::fs::create_dir(&real_dir).unwrap();
    let file_in_dir = real_dir.join("file.txt");
    std::fs::write(&file_in_dir, "content").unwrap();

    symlink(&real_dir, &symlink_dir).unwrap();

    let args = CliArgs {
        files: vec![symlink_dir.to_string_lossy().to_string()],
        stdin: false,
        recursive: false,
        format: lineguard::cli::OutputFormat::Human,
        quiet: false,
        verbose: false,
        no_color: false,
        config: None,
        ignore: vec![],
        extensions: None,
        no_newline_check: false,
        no_trailing_space: false,
        fix: false,
        dry_run: false,
        from: None,
        to: None,
        no_hidden: false,
    };

    let config = Config::default();
    let result = discover_files(&args, &config).unwrap();
    assert_eq!(result.files.len(), 1);
    assert!(result.files[0].to_string_lossy().contains("file.txt"));
}

#[test]
#[cfg(unix)]
fn test_discover_files_with_broken_symlink() {
    use std::os::unix::fs::symlink;

    let temp_dir = TempDir::new().unwrap();
    let nonexistent = temp_dir.path().join("nonexistent.txt");
    let broken_symlink = temp_dir.path().join("broken_symlink.txt");

    symlink(&nonexistent, &broken_symlink).unwrap();

    let args = CliArgs {
        files: vec![broken_symlink.to_string_lossy().to_string()],
        stdin: false,
        recursive: false,
        format: lineguard::cli::OutputFormat::Human,
        quiet: false,
        verbose: false,
        no_color: false,
        config: None,
        ignore: vec![],
        extensions: None,
        no_newline_check: false,
        no_trailing_space: false,
        fix: false,
        dry_run: false,
        from: None,
        to: None,
        no_hidden: false,
    };

    let config = Config::default();
    // Broken symlinks should be skipped
    let result = discover_files(&args, &config);
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result.files.len(), 0);
}

#[test]
#[cfg(unix)]
fn test_discover_files_with_circular_symlink() {
    use std::os::unix::fs::symlink;

    let temp_dir = TempDir::new().unwrap();
    let link1 = temp_dir.path().join("link1");
    let link2 = temp_dir.path().join("link2");

    // Create circular symlinks
    symlink(&link2, &link1).unwrap();
    symlink(&link1, &link2).unwrap();

    let args = CliArgs {
        files: vec![link1.to_string_lossy().to_string()],
        stdin: false,
        recursive: false,
        format: lineguard::cli::OutputFormat::Human,
        quiet: false,
        verbose: false,
        no_color: false,
        config: None,
        ignore: vec![],
        extensions: None,
        no_newline_check: false,
        no_trailing_space: false,
        fix: false,
        dry_run: false,
        from: None,
        to: None,
        no_hidden: false,
    };

    let config = Config::default();
    // Circular symlinks should be handled gracefully
    let result = discover_files(&args, &config);
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result.files.len(), 0);
}

#[test]
#[cfg(unix)]
fn test_discover_symlink_in_recursive_mode() {
    use std::os::unix::fs::symlink;

    let temp_dir = TempDir::new().unwrap();
    let sub_dir = temp_dir.path().join("subdir");
    std::fs::create_dir(&sub_dir).unwrap();

    let real_file = sub_dir.join("real.txt");
    let symlink_file = temp_dir.path().join("link.txt");

    std::fs::write(&real_file, "content").unwrap();
    symlink(&real_file, &symlink_file).unwrap();

    let args = CliArgs {
        files: vec![temp_dir.path().to_string_lossy().to_string()],
        stdin: false,
        recursive: true,
        format: lineguard::cli::OutputFormat::Human,
        quiet: false,
        verbose: false,
        no_color: false,
        config: None,
        ignore: vec![],
        extensions: None,
        no_newline_check: false,
        no_trailing_space: false,
        fix: false,
        dry_run: false,
        from: None,
        to: None,
        no_hidden: false,
    };

    let config = Config::default();
    let result = discover_files(&args, &config).unwrap();
    // Should find both the real file and the symlink
    assert_eq!(result.files.len(), 2);
}
