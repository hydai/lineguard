use lineguard::cli::{CliArgs, OutputFormat};
use lineguard::config::Config;
use lineguard::discovery::{discover_files, should_check_file};
use std::fs;
use tempfile::TempDir;

#[test]
fn test_discover_files_from_args() {
    let temp_dir = TempDir::new().unwrap();
    let file1 = temp_dir.path().join("file1.txt");
    let file2 = temp_dir.path().join("file2.txt");
    let file3 = temp_dir.path().join(".hidden.txt");

    fs::write(&file1, "content1").unwrap();
    fs::write(&file2, "content2").unwrap();
    fs::write(&file3, "hidden").unwrap();

    // Test file discovery from command line arguments
    let args = CliArgs {
        files: vec![
            file1.to_string_lossy().to_string(),
            file2.to_string_lossy().to_string(),
        ],
        stdin: false,
        recursive: false,
        format: OutputFormat::Human,
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
    assert_eq!(result.files.len(), 2);
}

#[test]
fn test_discover_files_with_extensions_from_cli() {
    let temp_dir = TempDir::new().unwrap();

    fs::write(temp_dir.path().join("file.txt"), "content").unwrap();
    fs::write(temp_dir.path().join("file.rs"), "content").unwrap();
    fs::write(temp_dir.path().join("file.py"), "content").unwrap();

    let args = CliArgs {
        files: vec![temp_dir.path().to_string_lossy().to_string()],
        stdin: false,
        recursive: false,
        format: OutputFormat::Human,
        quiet: false,
        verbose: false,
        no_color: false,
        config: None,
        ignore: vec![],
        extensions: Some(vec!["txt".to_string(), "rs".to_string()]),
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
    assert_eq!(result.files.len(), 2);
    assert!(result.files.iter().all(|p| {
        let ext = p.extension().and_then(|e| e.to_str()).unwrap_or("");
        ext == "txt" || ext == "rs"
    }));
}

#[test]
fn test_discover_files_with_ignore_patterns_from_cli() {
    let temp_dir = TempDir::new().unwrap();

    fs::write(temp_dir.path().join("keep.txt"), "content").unwrap();
    fs::write(temp_dir.path().join("ignore.txt"), "content").unwrap();
    fs::write(temp_dir.path().join("test.log"), "content").unwrap();

    let args = CliArgs {
        files: vec![temp_dir.path().to_string_lossy().to_string()],
        stdin: false,
        recursive: false,
        format: OutputFormat::Human,
        quiet: false,
        verbose: false,
        no_color: false,
        config: None,
        ignore: vec!["ignore.txt".to_string(), "*.log".to_string()],
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
    assert_eq!(
        result.files[0].file_name().unwrap().to_str().unwrap(),
        "keep.txt"
    );
}

#[test]
fn test_discover_files_with_invalid_glob_pattern() {
    let temp_dir = TempDir::new().unwrap();
    let file = temp_dir.path().join("[invalid.txt");
    fs::write(&file, "content").unwrap();

    let args = CliArgs {
        files: vec![file.to_string_lossy().to_string()],
        stdin: false,
        recursive: false,
        format: OutputFormat::Human,
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
    // Should still find the file as literal path
    assert_eq!(result.files.len(), 1);
}

#[test]
fn test_discover_files_glob_no_matches_then_literal() {
    let temp_dir = TempDir::new().unwrap();
    let file = temp_dir.path().join("file.txt");
    fs::write(&file, "content").unwrap();

    // Use a glob pattern that doesn't match but the literal path exists
    let pattern = format!("{}/*.rs", temp_dir.path().display());

    let args = CliArgs {
        files: vec![pattern, file.to_string_lossy().to_string()],
        stdin: false,
        recursive: false,
        format: OutputFormat::Human,
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
}

#[test]
fn test_should_check_file_binary_extensions() {
    let temp_dir = TempDir::new().unwrap();
    let config = Config::default();

    // Test various binary extensions
    let binary_files = vec![
        "image.jpg",
        "photo.png",
        "icon.ico",
        "video.mp4",
        "audio.mp3",
        "archive.zip",
        "compressed.tar.gz",
        "binary.exe",
        "library.so",
        "data.db",
        "cache.sqlite",
        "document.pdf",
        "sheet.xlsx",
        "compiled.class",
        "package.jar",
        "bytecode.pyc",
        "font.woff",
        "font.ttf",
    ];

    for filename in binary_files {
        let path = temp_dir.path().join(filename);
        assert!(
            !should_check_file(&path, &config),
            "{filename} should not be checked"
        );
    }
}

#[test]
fn test_should_check_file_with_configured_extensions() {
    let temp_dir = TempDir::new().unwrap();
    let config = Config {
        file_extensions: vec!["rs".to_string(), "txt".to_string()],
        ..Default::default()
    };

    let should_check = vec!["file.rs", "doc.txt"];
    let should_not_check = vec!["script.py", "config.json", "style.css"];

    for filename in should_check {
        let path = temp_dir.path().join(filename);
        assert!(
            should_check_file(&path, &config),
            "{filename} should be checked"
        );
    }

    for filename in should_not_check {
        let path = temp_dir.path().join(filename);
        assert!(
            !should_check_file(&path, &config),
            "{filename} should not be checked"
        );
    }
}

#[test]
fn test_should_check_file_no_extension() {
    let temp_dir = TempDir::new().unwrap();
    let config = Config::default();

    let path = temp_dir.path().join("README");
    assert!(should_check_file(&path, &config));

    let path = temp_dir.path().join("Makefile");
    assert!(should_check_file(&path, &config));
}

#[test]
#[cfg(unix)]
fn test_should_check_file_non_utf8_extension() {
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;

    let temp_dir = TempDir::new().unwrap();
    let config = Config::default();

    // Create a path with non-UTF8 extension
    let bytes = b"file.\xFF\xFF";
    let path = temp_dir.path().join(OsStr::from_bytes(bytes));

    // Should default to checking it
    assert!(should_check_file(&path, &config));
}

#[test]
#[cfg(unix)]
fn test_discover_directory_read_error() {
    use std::os::unix::fs::PermissionsExt;

    let temp_dir = TempDir::new().unwrap();
    let unreadable_dir = temp_dir.path().join("unreadable");
    fs::create_dir(&unreadable_dir).unwrap();

    // Create a file inside before making it unreadable
    fs::write(unreadable_dir.join("file.txt"), "content").unwrap();

    // Make directory unreadable
    let mut perms = fs::metadata(&unreadable_dir).unwrap().permissions();
    perms.set_mode(0o000);
    fs::set_permissions(&unreadable_dir, perms).unwrap();

    let args = CliArgs {
        files: vec![unreadable_dir.to_string_lossy().to_string()],
        stdin: false,
        recursive: false,
        format: OutputFormat::Human,
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
    // Should not panic, just skip the directory
    let result = discover_files(&args, &config);
    assert!(result.is_ok());

    // Restore permissions
    let mut perms = fs::metadata(&unreadable_dir).unwrap().permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&unreadable_dir, perms).unwrap();
}

#[test]
fn test_discover_files_recursive_with_ignored_directory() {
    let temp_dir = TempDir::new().unwrap();

    let sub_dir = temp_dir.path().join("src");
    let ignored_dir = temp_dir.path().join("target");

    fs::create_dir(&sub_dir).unwrap();
    fs::create_dir(&ignored_dir).unwrap();

    fs::write(sub_dir.join("main.rs"), "content").unwrap();
    fs::write(ignored_dir.join("debug.txt"), "content").unwrap();

    let args = CliArgs {
        files: vec![temp_dir.path().to_string_lossy().to_string()],
        stdin: false,
        recursive: true,
        format: OutputFormat::Human,
        quiet: false,
        verbose: false,
        no_color: false,
        config: None,
        ignore: vec!["target".to_string()],
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
    assert!(result.files[0].to_string_lossy().contains("main.rs"));
}

#[test]
fn test_is_ignored_with_parent_directory_pattern() {
    let temp_dir = TempDir::new().unwrap();

    let node_modules = temp_dir.path().join("node_modules");
    let deep_file = node_modules.join("package").join("dist").join("index.js");

    fs::create_dir_all(deep_file.parent().unwrap()).unwrap();
    fs::write(&deep_file, "content").unwrap();

    let args = CliArgs {
        files: vec![temp_dir.path().to_string_lossy().to_string()],
        stdin: false,
        recursive: true,
        format: OutputFormat::Human,
        quiet: false,
        verbose: false,
        no_color: false,
        config: None,
        ignore: vec!["node_modules".to_string()],
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
    assert_eq!(result.files.len(), 0);
}

#[test]
fn test_is_ignored_with_relative_path_pattern() {
    let temp_dir = TempDir::new().unwrap();

    let src_dir = temp_dir.path().join("src");
    let test_file = src_dir.join("test.rs");

    fs::create_dir(&src_dir).unwrap();
    fs::write(&test_file, "content").unwrap();
    fs::write(temp_dir.path().join("keep.rs"), "content").unwrap();

    // Change working directory to temp_dir for relative path testing
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&temp_dir).unwrap();

    let args = CliArgs {
        files: vec![".".to_string()],
        stdin: false,
        recursive: true,
        format: OutputFormat::Human,
        quiet: false,
        verbose: false,
        no_color: false,
        config: None,
        ignore: vec!["**/src/test.rs".to_string()], // Use glob pattern
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

    // Restore original directory
    std::env::set_current_dir(original_dir).unwrap();

    assert_eq!(result.files.len(), 1);
    assert!(result.files[0].file_name().unwrap().to_str().unwrap() == "keep.rs");
}
